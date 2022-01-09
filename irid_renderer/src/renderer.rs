//= USES ===========================================================================================

use std::{fmt::Debug, fs::read_to_string, marker::PhantomData, path::Path};

use bytemuck::Pod;
use thiserror::Error;

use irid_app_interface::Window;
use irid_assets::{ImageSize, Texture};
use irid_assets_interface::{Index, Vertex};

use crate::texture_metadatas::{
    TextureBindGroupMetadatas, TextureDepthMetadatas, TextureImageMetadatas,
};
use crate::utils::log2;
use crate::{
    Adapter, Camera, CameraController, CameraMetadatas, Device, Instance, PipelineLayoutBuilder,
    Queue, RenderPipeline, RenderPipelineBuilder, ShaderModuleBuilder, Surface,
    DEFAULT_FRAGMENT_ENTRY_POINT, DEFAULT_VERTEX_ENTRY_POINT,
};

//= ERRORS =========================================================================================

///
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RendererError {
    #[error("unable to get a Surface or Adapter")]
    SurfaceAdapterRequest,
    #[error("unable to get a Device")]
    DeviceRequest {
        #[from]
        source: wgpu::RequestDeviceError,
    },
}

//= CONSTS =========================================================================================

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

//= TRAIT FOR DEFAULT PATH TYPE ====================================================================

pub trait RendererPathType {
    type P: AsRef<std::path::Path>;
}

//= RENDERER BUILDER ===============================================================================

///
#[derive(Clone, Debug)]
pub struct RendererBuilder<
    'a,
    W: Window,
    P: AsRef<Path>,
    V: Vertex,
    I: Index,
    S: ImageSize,
    T: Texture<S>,
> {
    window: &'a W,

    clear_color: Option<wgpu::Color>,
    shader_path: Option<P>,
    texture_path: Option<P>,
    // TODO: Probably better to encapsulate the [ModelVertex] logic or use an Into
    vertices: Option<&'a [V]>,
    indices: Option<&'a [I]>,

    generic_size: PhantomData<S>,
    generic_texture: PhantomData<T>,
}

impl<'a, W, P, V, I, S, T> RendererBuilder<'a, W, P, V, I, S, T>
where
    W: Window,
    P: AsRef<Path> + Debug,
    V: Vertex + Pod,
    I: Index + Pod,
    S: ImageSize,
    T: Texture<S>,
{
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new(window: &'a W) -> Self {
        Self {
            window,
            clear_color: None,
            shader_path: None,
            texture_path: None,
            vertices: None,
            indices: None,
            generic_size: Default::default(),
            generic_texture: Default::default(),
        }
    }

    //- Setters ------------------------------------------------------------------------------------

    ///
    pub fn with_window(mut self, window: &'a W) -> Self {
        self.window = window;
        self
    }

    /// Color used by a [render pass color attachment](wgpu::RenderPassColorAttachment)
    /// to perform a [clear operation](wgpu::LoadOp).
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = clear_color.into();
        self
    }

    ///
    pub fn with_shader_path(mut self, shader_path: P) -> Self {
        self.shader_path = Some(shader_path);
        self
    }

    ///
    pub fn with_texture_path(mut self, texture_path: P) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    ///
    pub fn with_vertices<IV: Into<Option<&'a [V]>>>(mut self, vertices: IV) -> Self {
        self.vertices = vertices.into();
        self
    }

    ///
    pub fn with_indices<II: Into<Option<&'a [I]>>>(mut self, indices: II) -> Self {
        self.indices = indices.into();
        self
    }

    //- Build --------------------------------------------------------------------------------------

    ///
    pub fn build(self) -> Result<Renderer, RendererError> {
        //- Surface, Device, Queue -----------------------------------------------------------------

        let window_size = self.window.inner_size();

        let backends = wgpu::Backends::VULKAN | wgpu::Backends::DX12;
        let (surface, adapter) = Surface::new(backends, self.window, window_size)
            // TODO: better pass `e` as argument to SurfaceAdapterRequest for chaining error descr?
            .map_err(|_| RendererError::SurfaceAdapterRequest)?;

        let (device, queue) = pollster::block_on(Device::new(&adapter))?;

        surface.configure(&device);

        //- Camera ---------------------------------------------------------------------------------

        let camera = Camera::new(window_size.width as f32, window_size.height as f32);
        let camera_metadatas = camera.create_metadatas(&device);
        let camera_controller = CameraController::new(0.2);

        //- Texture Metadatas ----------------------------------------------------------------------

        let texture_image_metadatas = if self.texture_path.is_some() {
            self.create_texture_image_metadatas(&device, surface.preferred_format())
        } else {
            vec![]
        };

        let texture_bind_group_metadatas = if self.texture_path.is_some() {
            self.create_texture_bind_group_metadatas(&device, &texture_image_metadatas)
        } else {
            vec![]
        };

        let texture_depth_metadatas = TextureDepthMetadatas::new(&device, window_size);

        //- Pipeline -------------------------------------------------------------------------------

        let renderer_pipeline = if self.shader_path.is_some() {
            let path = std::env::current_dir()
                .unwrap()
                .as_path()
                .join(&self.shader_path.unwrap());
            let content = match read_to_string(&path) {
                Ok(content) => content,
                Err(err) => panic!("Couldn't open {:?} file: {}", path, err),
            };

            let source = wgpu::ShaderSource::Wgsl(std::borrow::Cow::Owned(content));
            //#[cfg(feature = "glsl")]
            //wgpu::ShaderSource::Glsl(std::borrow::Cow::Owned(shader_key))

            let shader_module = ShaderModuleBuilder::new(source).build(&device);

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

            let color_targets = [wgpu::ColorTargetState {
                format: surface.preferred_format(), //.unwrap_or(wgpu::TextureFormat::Rgba16Float),
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            }];

            let fragment_states = wgpu::FragmentState {
                module: &shader_module,
                entry_point: DEFAULT_FRAGMENT_ENTRY_POINT,
                targets: &color_targets,
            };

            let pipeline_layout = if texture_bind_group_metadatas.is_empty() {
                let camera_bgl = camera_metadatas.bind_group_layout();
                PipelineLayoutBuilder::new()
                    .with_bind_group_layouts(&[camera_bgl])
                    .build(&device)
            } else {
                // TODO: 256x256 texture, hardcoded for now :(
                let texture_bgl = texture_bind_group_metadatas[8][8].bind_group_layout();
                let camera_bgl = camera_metadatas.bind_group_layout();
                PipelineLayoutBuilder::new()
                    .with_bind_group_layouts(&[texture_bgl, camera_bgl])
                    .build(&device)
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

        //- Queue Schedule -------------------------------------------------------------------------

        if self.texture_path.is_some() {
            // TODO: here we use unwrap because texture loading will probably not be done at this point
            //  and therefore it is useless to add a new type of error
            queue.write_texture(
                &texture_image_metadatas,
                T::load(self.texture_path.unwrap()).unwrap(),
            );
        }

        //- Vertex and Index Buffers ---------------------------------------------------------------

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

        //- Instances ------------------------------------------------------------------------------

        let (instances, instances_buffer) = if self.vertices.is_some() {
            let instances = RendererBuilder::<'a, W, P, V, I, S, T>::create_instances();
            let instances_buffer = RendererBuilder::<'a, W, P, V, I, S, T>::create_instances_buffer(
                &device, &instances,
            );
            (Some(instances), Some(instances_buffer))
        } else {
            (None, None)
        };

        //- Renderer Creation ----------------------------------------------------------------------

        Ok(Renderer {
            window_size,
            clear_color: self.clear_color.unwrap_or(wgpu::Color::WHITE),
            surface,
            adapter,
            device,
            queue,

            camera,
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
        &self,
        device: &Device,
        preferred_format: wgpu::TextureFormat,
    ) -> Vec<Vec<TextureImageMetadatas>> {
        // Better to check not the current limits but the default ones
        // so as to obtain consistent behavior on all devices.
        let qty = log2(wgpu::Limits::default().max_texture_dimension_2d as i32) as usize;
        let mut vec_w = Vec::<Vec<TextureImageMetadatas>>::with_capacity(qty);
        for (width, w_element) in vec_w.iter_mut().enumerate() {
            let mut vec_h = Vec::<TextureImageMetadatas>::with_capacity(qty);
            for (height, h_element) in vec_h.iter_mut().enumerate() {
                *h_element = TextureImageMetadatas::new(
                    device,
                    preferred_format,
                    2_u32.pow(width as u32),
                    2_u32.pow(height as u32),
                );
            }
            *w_element = vec_h;
        }
        vec_w
    }

    ///
    pub fn create_texture_bind_group_metadatas(
        &self,
        device: &Device,
        texture_image_metadatas: &[Vec<TextureImageMetadatas>],
    ) -> Vec<Vec<TextureBindGroupMetadatas>> {
        let qty = texture_image_metadatas.len();
        let mut vec_w = Vec::<Vec<TextureBindGroupMetadatas>>::with_capacity(qty);
        for (width, w_element) in vec_w.iter_mut().enumerate() {
            let mut vec_h = Vec::<TextureBindGroupMetadatas>::with_capacity(qty);
            for (height, h_element) in vec_h.iter_mut().enumerate() {
                *h_element = TextureBindGroupMetadatas::new(
                    device,
                    texture_image_metadatas[width][height].texture(),
                );
            }
            *w_element = vec_h;
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

                    Instance { position, rotation }
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

//= RENDERER OBJECT ================================================================================

///
pub struct Renderer {
    window_size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    surface: Surface,
    #[allow(dead_code)]
    adapter: Adapter,
    device: Device,
    queue: Queue,

    camera: Camera,
    camera_metadatas: CameraMetadatas,
    camera_controller: CameraController,

    #[allow(dead_code)]
    texture_image_metadatas: Vec<Vec<TextureImageMetadatas>>,
    texture_bind_group_metadatas: Vec<Vec<TextureBindGroupMetadatas>>,
    texture_depth_metadatas: TextureDepthMetadatas,

    renderer_pipeline: Option<RenderPipeline>,
    // TODO: maybe this is better to move inside the render_pass or pipeline object (also the fields below)
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
    instances: Option<Vec<Instance>>,
    instances_buffer: Option<wgpu::Buffer>,
}

impl Renderer {
    //- Surface (Re)size ---------------------------------------------------------------------------

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

    //- Camera -------------------------------------------------------------------------------------

    ///
    pub fn process_camera_events(&mut self, input: &winit::event::KeyboardInput) -> bool {
        self.camera_controller.process_events(input)
    }

    //- Command Encoder ----------------------------------------------------------------------------

    ///
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            })
    }

    //- Rendering ----------------------------------------------------------------------------------

    ///
    pub fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.camera_controller.update_camera(&mut self.camera);
        self.queue
            .write_camera_buffer(&self.camera, &self.camera_metadatas);

        let frame = self.surface.get_current_texture()?;
        let texture = &frame.texture;
        let frame_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.create_command_encoder("Render Encoder");

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
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
                let rp = self.renderer_pipeline.as_ref().unwrap();
                // TODO: remove this expose call creating an RenderPass wrapper
                render_pass.set_pipeline(rp.expose_wrapped_render_pipeline());
                if self.texture_bind_group_metadatas.is_empty() {
                    render_pass.set_bind_group(0, self.camera_metadatas.bind_group(), &[]);
                } else {
                    render_pass.set_bind_group(
                        0,
                        // TODO: hardcoded :(
                        self.texture_bind_group_metadatas[8][8].bind_group(),
                        &[],
                    );
                    render_pass.set_bind_group(1, self.camera_metadatas.bind_group(), &[]);
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
                    render_pass.draw(0..3, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}
