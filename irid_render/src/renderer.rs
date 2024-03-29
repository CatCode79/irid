//= USES =====================================================================

use std::fmt::{Display, Formatter};
use std::{error::Error, fmt::Debug, fs::read_to_string, path::Path};

use bytemuck::Pod;
use irid_assets::DiffuseTexture;
use irid_assets::{Index, Vertex};

use crate::{
    camera::Camera,
    camera_bind::CameraBindGroup,
    device::Device,
    instance::Instance,
    queue::{Queue, QueueError},
    shader::{DEFAULT_FRAGMENT_ENTRY_POINT, DEFAULT_VERTEX_ENTRY_POINT},
    surface::Surface,
    texture_metadata::{TextureBindGroupMetadatas, TextureDepthMetadatas, TextureImageMetadata},
    utils::log2,
    CameraController, PipelineLayoutBuilder, RenderPipeline, RenderPipelineBuilder,
};

//= ERRORS ===================================================================

///
#[derive(Debug)]
pub enum RendererError {
    SurfaceAdapterRequest,
    DeviceRequest { source: wgpu::RequestDeviceError },
    LoadTexture { source: irid_assets::TextureError },
    WriteTexture { source: QueueError },
}

impl Display for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::SurfaceAdapterRequest => write!(f, "Unable to get a Surface or Adapter"),
            RendererError::DeviceRequest { source } => {
                write!(f, "Unable to get a Device: {}", source)
            }
            RendererError::LoadTexture { source } => {
                write!(f, "Unable to load the texture: {}", source)
            }
            RendererError::WriteTexture { source } => {
                write!(f, "Unable to enqueue the texture: {}", source)
            }
        }
    }
}

impl Error for RendererError {}

//= CONSTS ===================================================================

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

//= RENDERER BUILDER =========================================================

///
#[derive(Clone, Debug)]
pub struct RendererConfig<'a, C: Camera, PS: AsRef<Path>, PT: AsRef<Path>, V: Vertex, I: Index> {
    // First tier support backends for the Instance request
    backends: wgpu::Backends,

    // Fifo is "vsync on". Immediate is "vsync off".
    // Mailbox is a hybrid between the two (gpu doesn't block if running
    // faster than the display, but screen tearing doesn't happen).
    present_mode: wgpu::PresentMode,

    // Options for the Device request
    features: wgpu::Features,
    limits: wgpu::Limits,

    camera: Option<C>,
    shader_path: Option<PS>,
    texture_path: Option<PT>,
    vertices: Option<&'a [V]>,
    indices: Option<&'a [I]>,
    clear_color: Option<wgpu::Color>,
}

impl<'a, C, PS, PT, V, I> Default for RendererConfig<'a, C, PS, PT, V, I>
where
    C: Camera + Clone,
    PS: AsRef<Path> + Debug,
    PT: AsRef<Path> + Debug,
    V: Vertex + Pod,
    I: Index + Pod,
{
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::DX12
                | wgpu::Backends::VULKAN
                | wgpu::Backends::METAL
                | wgpu::Backends::GL,
            present_mode: wgpu::PresentMode::Fifo,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_defaults(),
            camera: None,
            shader_path: None,
            texture_path: None,
            vertices: None,
            indices: None,
            clear_color: None,
        }
    }
}

impl<'a, C, PS, PT, V, I> RendererConfig<'a, C, PS, PT, V, I>
where
    C: Camera + Clone,
    PS: AsRef<Path> + Debug,
    PT: AsRef<Path> + Debug,
    V: Vertex + Pod,
    I: Index + Pod,
{
    //- Constructors ---------------------------------------------------------

    ///
    pub fn new() -> Self {
        Self::default()
    }

    //- Setters --------------------------------------------------------------

    ///
    #[inline]
    pub fn with_backends(mut self, backends: wgpu::Backends) -> Self {
        self.backends = backends;
        self
    }

    ///
    #[inline]
    pub fn with_present_mode(mut self, present_mode: wgpu::PresentMode) -> Self {
        self.present_mode = present_mode;
        self
    }

    ///
    #[inline]
    pub fn with_features(mut self, features: wgpu::Features) -> Self {
        self.features = features;
        self
    }

    ///
    #[inline]
    pub fn with_limits(mut self, limits: wgpu::Limits) -> Self {
        self.limits = limits;
        self
    }

    ///
    #[inline]
    pub fn with_camera<IC: Into<Option<C>>>(mut self, camera: IC) -> Self {
        self.camera = camera.into();
        self
    }

    ///
    #[inline]
    pub fn with_shader_path(mut self, shader_path: PS) -> Self {
        self.shader_path = Some(shader_path);
        self
    }

    ///
    #[inline]
    pub fn with_texture_path(mut self, texture_path: PT) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    ///
    #[inline]
    pub fn with_vertices(mut self, vertices: &'a [V]) -> Self {
        self.vertices = Some(vertices);
        self
    }

    ///
    #[inline]
    pub fn with_indices(mut self, indices: &'a [I]) -> Self {
        self.indices = Some(indices);
        self
    }

    /// Set a clear color with rgb channels as arguments.
    /// The alpha channel is set to 1.0 by default.
    /// See also the method [with_clear_color_rgba].
    #[inline]
    pub fn with_clear_color_rgb(mut self, r: f32, g: f32, b: f32) -> Self {
        self.clear_color = Some(wgpu::Color {
            r: r as f64,
            g: g as f64,
            b: b as f64,
            a: 1.0_f64,
        });
        self
    }

    /// Set a clear color with rgba channels as arguments.
    /// See also the method [with_clear_color].
    #[inline]
    pub fn with_clear_color_rgba(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.clear_color = Some(wgpu::Color {
            r: r as f64,
            g: g as f64,
            b: b as f64,
            a: a as f64,
        });
        self
    }

    /// Color used by a
    /// [render pass color attachment](wgpu::RenderPassColorAttachment)
    /// to perform a [clear operation](wgpu::LoadOp).
    #[inline]
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = clear_color.into();
        self
    }

    //- Build ----------------------------------------------------------------

    ///
    pub fn build(&self, window: &'a winit::window::Window) -> Result<Renderer<C>, RendererError> {
        //- Surface, Device, Queue -------------------------------------------

        let window_size = window.inner_size();

        let (surface, adapter) = Surface::new(self.backends, window, self.present_mode)
            .map_err(|_| RendererError::SurfaceAdapterRequest)?;

        // TODO: better find a way to remove the limits.clone()
        let (device, queue) = Device::new(&adapter, self.features, self.limits.clone())
            .map_err(|e| RendererError::DeviceRequest { source: e })?;

        surface.configure(&device);

        //- Camera -----------------------------------------------------------

        let (camera_metadatas, camera_controller) = if self.camera.is_some() {
            (
                Some(CameraBindGroup::new(self.camera.as_ref().unwrap(), &device)),
                Some(CameraController::new(0.2)),
            )
        } else {
            (None, None)
        };

        //- Texture Metadatas ------------------------------------------------

        let texture_image_metadatas = if self.texture_path.is_some() {
            RendererConfig::<'a, C, PS, PT, V, I>::create_texture_image_metadatas(
                &device,
                surface.configuration(),
            )
        } else {
            vec![]
        };

        let texture_bind_group_metadatas = if self.texture_path.is_some() {
            RendererConfig::<'a, C, PS, PT, V, I>::create_texture_bind_group_metadatas(
                &device,
                &texture_image_metadatas,
            )
        } else {
            vec![]
        };

        let texture_depth_metadatas = TextureDepthMetadatas::new(
            &device,
            window_size,
        );

        //- Pipeline ---------------------------------------------------------

        let renderer_pipeline = if self.shader_path.is_some() {
            let path = std::env::current_dir()
                .unwrap()
                .as_path()
                .join(self.shader_path.as_ref().unwrap());
            let content = match read_to_string(&path) {
                Ok(content) => content,
                Err(err) => panic!("Couldn't open {:?} file: {}", path, err),
            };

            let source = wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(content));
            //#[cfg(feature = "glsl")]
            //wgpu::ShaderSource::Glsl(std::borrow::Cow::Owned(shader_key))

            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source,
            });

            // TODO: no good...
            let vertex_buffers = [V::desc()];
            // TODO: raw instances must be optional
            //let vertex_buffers = [V::desc(), InstanceRaw::desc()];

            let vertex_state = if self.vertices.is_some() {
                wgpu::VertexState {
                    module: &shader_module,
                    entry_point: DEFAULT_VERTEX_ENTRY_POINT,
                    buffers: &vertex_buffers,
                }
            } else {
                wgpu::VertexState {
                    module: &shader_module,
                    entry_point: DEFAULT_VERTEX_ENTRY_POINT,
                    buffers: &[],
                }
            };

            let color_targets = [Some(wgpu::ColorTargetState {
                format: surface.configuration().format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })];

            let fragment_states = wgpu::FragmentState {
                module: &shader_module,
                entry_point: DEFAULT_FRAGMENT_ENTRY_POINT,
                targets: &color_targets,
            };

            let pipeline_layout = if texture_bind_group_metadatas.is_empty() {
                let plb = PipelineLayoutBuilder::new();
                if self.camera.is_some() {
                    let camera_bgl = camera_metadatas.as_ref().unwrap().bind_group_layout(); // TODO: remove unwrap
                    plb.with_bind_group_layouts(&[camera_bgl]).build(&device)
                } else {
                    plb.build(&device)
                }
            } else {
                // TODO: 256x256 texture, hardcoded for now :(
                let texture_bgl = texture_bind_group_metadatas[8][8].bind_group_layout();

                let plb = PipelineLayoutBuilder::new();
                if self.camera.is_some() {
                    let camera_bgl = camera_metadatas.as_ref().unwrap().bind_group_layout(); // TODO: remove unwrap
                    plb.with_bind_group_layouts(&[texture_bgl, camera_bgl])
                        .build(&device)
                } else {
                    plb.with_bind_group_layouts(&[texture_bgl]).build(&device)
                }
            };

            Some(
                RenderPipelineBuilder::new(vertex_state)
                    .with_fragment(fragment_states)
                    .with_layout(&pipeline_layout)
                    .build(&device),
            )
        } else {
            None
        };

        //- Queue Schedule ---------------------------------------------------

        if self.texture_path.is_some() {
            // TODO: here we use unwrap because texture loading will probably not be done at this point
            //  and therefore it is useless to add a new type of error
            queue
                .write_texture(
                    &texture_image_metadatas,
                    DiffuseTexture::load(self.texture_path.as_ref().unwrap())
                        .map_err(|e| RendererError::LoadTexture { source: e })?,
                )
                .map_err(|e| RendererError::WriteTexture { source: e })?
        }

        //- Vertex and Index Buffers -----------------------------------------

        let vertex_buffer = self
            .vertices
            .map(|v| device.create_vertex_buffer_init("Vertex Buffer", v));

        let index_buffer = self
            .indices
            .map(|i| device.create_indices_buffer_init("Index Buffer", i));

        let num_indices = if self.indices.is_some() {
            self.indices.unwrap().len() as u32
        } else {
            0_u32
        };

        //- Instances --------------------------------------------------------

        let (instances, instances_buffer) = if self.vertices.is_some() {
            let instances = RendererConfig::<'a, C, PS, PT, V, I>::create_instances();
            let instances_buffer =
                RendererConfig::<'a, C, PS, PT, V, I>::create_instances_buffer(&device, &instances);
            (Some(instances), Some(instances_buffer))
        } else {
            (None, None)
        };

        //- Renderer Creation ------------------------------------------------

        Ok(Renderer {
            window_size,
            clear_color: self.clear_color.unwrap_or(wgpu::Color::WHITE),
            surface,
            device,
            queue,

            camera: self.camera.clone(),
            camera_metadatas,
            camera_controller,

            texture_image_metadatas,
            texture_bind_group_metadatas,
            texture_depth_metadatas,

            renderer_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            instances,
            instances_buffer,
        })
    }

    ///
    ///
    /// It can't cache zero sized textures.
    pub fn create_texture_image_metadatas(
        device: &Device,
        configuration: &wgpu::SurfaceConfiguration,
    ) -> Vec<Vec<TextureImageMetadata>> {
        let qty = log2(wgpu::Limits::downlevel_defaults().max_texture_dimension_2d as i32) as usize;
        let mut vec_w = Vec::<Vec<TextureImageMetadata>>::with_capacity(qty);
        for width in 0..qty {
            let mut vec_h = Vec::<TextureImageMetadata>::with_capacity(qty);
            for height in 0..qty {
                vec_h.push(TextureImageMetadata::new(
                    device,
                    2_u32.pow(width as u32),
                    2_u32.pow(height as u32),
                    configuration,
                ));
            }
            vec_w.push(vec_h);
        }
        vec_w
    }

    ///
    pub fn create_texture_bind_group_metadatas(
        device: &Device,
        texture_image_metadatas: &[Vec<TextureImageMetadata>],
    ) -> Vec<Vec<TextureBindGroupMetadatas>> {
        let qty = texture_image_metadatas.len();
        let mut vec_w = Vec::<Vec<TextureBindGroupMetadatas>>::with_capacity(qty);
        for vec_width_metadatas in texture_image_metadatas.iter() {
            let mut vec_h = Vec::<TextureBindGroupMetadatas>::with_capacity(qty);
            for metadata_element in vec_width_metadatas.iter() {
                vec_h.push(TextureBindGroupMetadatas::new(
                    device,
                    metadata_element.texture(),
                ));
            }
            vec_w.push(vec_h);
        }
        vec_w
    }

    fn create_instances() -> Vec<Instance> {
        (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                use cgmath::{InnerSpace, Rotation3, Zero};

                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    let position = cgmath::Vector3 {
                        x: x as f32,
                        y: 0.0,
                        z: z as f32,
                    } - INSTANCE_DISPLACEMENT;

                    let rotation = if position.is_zero() {
                        // this is needed so an object at (0, 0, 0) won't get scaled to zero
                        // as Quaternions can effect scale if they're not created correctly
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Rad(0.0f32),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(
                            position.normalize(),
                            cgmath::Rad(std::f32::consts::PI / 4.0f32),
                        )
                    };

                    Instance::new(position, rotation)
                })
            })
            .collect::<Vec<_>>()
    }

    fn create_instances_buffer(device: &Device, instances: &[Instance]) -> wgpu::Buffer {
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        // TODO: When we will create the generics about Vertices we will use the
        //  Device.create_vertex_buffer_init instead
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }
}

//= RENDERER OBJECT ==========================================================

///
#[derive(Debug)]
pub struct Renderer<C: Camera> {
    window_size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    surface: Surface,
    device: Device,
    queue: Queue,

    camera: Option<C>,
    camera_metadatas: Option<CameraBindGroup>,
    camera_controller: Option<CameraController>,

    #[allow(dead_code)]
    texture_image_metadatas: Vec<Vec<TextureImageMetadata>>,
    texture_bind_group_metadatas: Vec<Vec<TextureBindGroupMetadatas>>,
    texture_depth_metadatas: TextureDepthMetadatas,

    renderer_pipeline: Option<RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
    instances: Option<Vec<Instance>>,
    instances_buffer: Option<wgpu::Buffer>,
}

impl<C> Renderer<C>
where
    C: Camera + Clone,
{
    //- Surface (Re)size -----------------------------------------------------

    /// Getter for the windows's physical size attribute.
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    /// Calculate the aspect ratio of the window's inner size.
    pub fn calc_aspect_ratio(&self) -> f32 {
        self.window_size.width as f32 / self.window_size.height as f32
    }

    /// Resize the renderer window.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = new_size;
        if new_size.width > 0 && new_size.height > 0 {
            self.texture_depth_metadatas =
                TextureDepthMetadatas::new(&self.device, self.window_size);
            self.refresh_current_size();
        }
    }

    ///
    pub fn refresh_current_size(&mut self) {
        self.surface.update(&self.device, self.window_size);
    }

    //- Camera ---------------------------------------------------------------

    ///
    pub fn process_camera_events(&mut self, input: winit::event::KeyboardInput) -> bool {
        if self.camera_controller.is_some() {
            self.camera_controller
                .as_mut()
                .unwrap()
                .process_events(input) // TODO: remove Unwrap
        } else {
            true
        }
    }

    //- Command Encoder ------------------------------------------------------

    ///
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            })
    }

    //- Rendering ------------------------------------------------------------

    ///
    pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        // TODO: remove clone (and probably) also unwraps
        if self.camera.is_some() {
            let camera = self.camera.as_mut().unwrap();
            self.camera_controller
                .as_ref()
                .unwrap()
                .update_camera(camera);
            self.queue.write_camera_buffer(
                self.camera.as_ref().unwrap(),
                self.camera_metadatas.as_ref().unwrap(),
            );
        }

        let frame = self.surface.get_current_texture()?;
        let texture = &frame.texture;
        let frame_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.create_command_encoder("Render Encoder");

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: self.texture_depth_metadatas.view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            if self.renderer_pipeline.is_some() {
                let pipeline = self.renderer_pipeline.as_ref().unwrap();
                // TODO: remove this expose call creating an RenderPass wrapper
                render_pass.set_pipeline(pipeline.expose_wrapped_render_pipeline());

                if self.texture_bind_group_metadatas.is_empty() {
                    if self.camera.is_some() {
                        render_pass.set_bind_group(
                            0,
                            self.camera_metadatas.as_ref().unwrap().bind_group(), // TODO: Remove unwrap()
                            &[],
                        );
                    }
                } else {
                    render_pass.set_bind_group(
                        0,
                        // TODO: hardcoded :(
                        self.texture_bind_group_metadatas[8][8].bind_group(),
                        &[],
                    );
                    if self.camera.is_some() {
                        render_pass.set_bind_group(
                            1,
                            self.camera_metadatas.as_ref().unwrap().bind_group(), // TODO: Remove unwrap()
                            &[],
                        );
                    }
                }

                if self.vertex_buffer.is_some() {
                    render_pass
                        .set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
                }

                if self.instances_buffer.is_some() {
                    render_pass
                        .set_vertex_buffer(1, self.instances_buffer.as_ref().unwrap().slice(..));
                }

                if self.index_buffer.is_some() {
                    render_pass.set_index_buffer(
                        self.index_buffer.as_ref().unwrap().slice(..),
                        wgpu::IndexFormat::Uint16,
                    );
                    render_pass.draw_indexed(
                        0..self.num_indices,
                        0,
                        0..self.instances.as_ref().unwrap().len() as _,
                    );
                } else {
                    // TODO: uhm, sound like a bug. Probably too tied with lw_03_example and vertices
                    render_pass.draw(0..3, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
