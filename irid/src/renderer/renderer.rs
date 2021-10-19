
//= USES ===========================================================================================

use crate::assets::DiffuseImage;
use crate::renderer::{TextureBindGroupMetadatas, TextureDepthMetadatas, TextureImageMetadatas};


//= CONSTS =========================================================================================

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5
);


//= RENDERER STRUCT ================================================================================

///
pub struct Renderer {
    window_size: winit::dpi::PhysicalSize<u32>,
    surface: crate::renderer::Surface,
    _adapter: crate::renderer::Adapter,
    device: crate::renderer::Device,
    queue: wgpu::Queue,
    camera: crate::renderer::Camera,
    camera_metadatas: crate::renderer::CameraMetadatas,
    camera_controller: crate::renderer::CameraController,
    _texture_image_metadatas: crate::renderer::TextureImageMetadatas,
    texture_bind_group_metadatas: crate::renderer::TextureBindGroupMetadatas,
    texture_depth_metadatas: crate::renderer::TextureDepthMetadatas,
    pipeline: crate::renderer::RenderPipeline,
    vertex_buffer: wgpu::Buffer,  // TODO: maybe this is better to move this buffer, and the index buffer, inside the render_pass or pipeline object
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instances: Vec<crate::renderer::Instance>,
    instance_buffer: wgpu::Buffer,
}


impl Renderer {
    ///
    pub fn new(
        window: &winit::window::Window,
        shader_source: String,
        texture_path: &std::path::Path,
        vertices: &[crate::assets::ModelVertex],
        indices: &[u32]
    ) -> anyhow::Result<Self> {
        let window_size = window.inner_size();

        let backends = wgpu::Backends::VULKAN | wgpu::Backends::DX12;
        let (surface, adapter) = crate::renderer::Surface::new(backends, window, window_size)?;

        let (device, queue) = crate::renderer::Device::new(&adapter)?;

        surface.configure(&device);

        //- Camera ---------------------------------------------------------------------------------

        let camera = crate::renderer::Camera::new(window_size.width as f32, window_size.height as f32);
        let camera_metadatas = camera.create_metadatas(&device);
        let camera_controller = crate::renderer::CameraController::new(0.2);

        //- Texture --------------------------------------------------------------------------------

        let diffuse_image = DiffuseImage::new(texture_path)?;

        let texture_image_metadatas = TextureImageMetadatas::new(
            &surface, &device, diffuse_image.width(), diffuse_image.height()
        );

        let texture_bind_group_metadatas= TextureBindGroupMetadatas::new(
            &device, &texture_image_metadatas.texture()
        );

        let texture_depth_metadatas = TextureDepthMetadatas::new(&device, window_size);

        //- Pipeline -------------------------------------------------------------------------------

        let pipeline = crate::renderer::RenderPipeline::new(
            &surface,
            &device,
            texture_bind_group_metadatas.bind_group_layout(),
            camera_metadatas.bind_group_layout(),
            shader_source
        );

        //- Queue Schedule -------------------------------------------------------------------------

        // TODO we have to create a IridQueue object to remove those args (also we have to think about clones)
        queue.write_texture(
            texture_image_metadatas.create_image_copy(),
            diffuse_image.as_rgba8_bytes().unwrap(),  // TODO: piace poco l'unwrap
            texture_image_metadatas.image_data_layout().clone(),
            texture_image_metadatas.image_size().clone()
        );

        //- Vertex and Index Buffers ---------------------------------------------------------------

        let vertex_buffer = device.create_vertex_buffer_init("Vertex Buffer", vertices);
        let index_buffer = device.create_indices_buffer_init("Index Buffer", indices);

        let num_indices = indices.len() as u32;

        //- Instances ------------------------------------------------------------------------------

        let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            use cgmath::{Zero, Rotation3, InnerSpace};

            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let position =
                    cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 } - INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can effect scale if they're not created correctly
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(),
                                                        cgmath::Rad(0.0f32))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(),
                                                        cgmath::Rad(std::f32::consts::PI / 4.0f32))
                };

                crate::renderer::Instance {
                    position,
                    rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(crate::renderer::Instance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(  // TODO when we will create the generics avout Vertices we will use the Device.create_vertex_buffer_init instead
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        //- Renderer Creation ----------------------------------------------------------------------

        Ok(Self {
            window_size,
            surface,
            _adapter: adapter,
            device,
            queue,
            _texture_image_metadatas: texture_image_metadatas,
            texture_bind_group_metadatas,
            texture_depth_metadatas,
            camera,
            camera_metadatas,
            camera_controller,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            instances,
            instance_buffer,
        })
    }

    //- Size Methods -------------------------------------------------------------------------------

    /// Getter for the windows's physical size attribute.
    #[inline]
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    /// Calculate the aspect ratio of the window's inner size.
    #[inline]
    pub fn calc_aspect_ratio(&self) -> f32 {
        self.window_size.width as f32 / self.window_size.height as f32
    }

    /// Resize the renderer window.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = new_size;
        self.texture_depth_metadatas =
            crate::renderer::TextureDepthMetadatas::new(&self.device, self.window_size);
        self.refresh_current_size();
    }

    ///
    pub fn refresh_current_size(&mut self) {
        self.surface.update(&self.device, self.window_size);
    }

    //- Camera Methods -----------------------------------------------------------------------------

    ///
    pub fn process_camera_events(&mut self, input: &winit::event::KeyboardInput) -> bool {
        self.camera_controller.process_events(input)
    }

    //- Command Encoder Methods --------------------------------------------------------------------

    ///
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            }
        )
    }

    //- Rendering Methods --------------------------------------------------------------------------

    ///
    pub(crate) fn redraw(&mut self, config: &crate::app::Config) -> Result<(), wgpu::SurfaceError> {
        self.camera_controller.update_camera(&mut self.camera);
        let mut camera_uniform = *self.camera_metadatas.uniform();
        camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_metadatas.buffer(),
            0,
            bytemuck::cast_slice(&[camera_uniform])
        );

        //- ¡WARNING STARTS! -----------------------------------------------------------------------
        // output variable must be let binded and not used inline or it give me a validation error:
        /*
[2021-09-12T18:39:07Z ERROR wgpu_hal::vulkan::instance] VALIDATION [VUID-VkPresentInfoKHR-pImageIndices-01296 (0xc7aabc16)]
    	Validation Error: [ VUID-VkPresentInfoKHR-pImageIndices-01296 ] Object 0: handle = 0x21511202e68, type = VK_OBJECT_TYPE_QUEUE; | MessageID = 0xc7aabc16 | vkQueuePresentKHR(): pSwapchains[0] images passed to present must be in layout VK_IMAGE_LAYOUT_PRESENT_SRC_KHR or VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR but is in VK_IMAGE_LAYOUT_UNDEFINED. The Vulkan spec states: Each element of pImageIndices must be the index of a presentable image acquired from the swapchain specified by the corresponding element of the pSwapchains array, and the presented image subresource must be in the VK_IMAGE_LAYOUT_PRESENT_SRC_KHR layout at the time the operation is executed on a VkDevice (https://github.com/KhronosGroup/Vulkan-Docs/search?q=)VUID-VkPresentInfoKHR-pImageIndices-01296)
[2021-09-12T18:39:07Z ERROR wgpu_hal::vulkan::instance] 	objects: (type: QUEUE, hndl: 0x21511202e68, name: ?)
thread 'main' panicked at 'Texture[1] does not exist', C:\Users\DarkWolf\.cargo\registry\src\github.com-1ecc6299db9ec823\wgpu-core-0.10.1\src\hub.rs:129:32
        */
        // Also if I move those two lines inside the render_pass scope I still have the error.
        // I suspect a bug inside the wgpu. I found this on wgpu v0.10.
        // (I should probably mention it as an issue on the official github repo but I'm a lazy cat)
        // TODO: test results with wgpu 0.11: NO TEST PERFORMED, probably the logic is changed because some modification about frame and output

        let output = self.surface.get_current_frame()?.output;
        let frame_view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: None,
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None
        });
        //- ¡WARNING ENDS! -------------------------------------------------------------------------

        let mut encoder = self.create_command_encoder("Render Encoder");

        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &frame_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(config.clear_color()),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.texture_depth_metadatas.view(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
                }
            );

            render_pass.set_pipeline(self.pipeline.expose_wrapped_render_pipeline());  // TODO we can remove this expose call creating an RenderPass wrapper
            render_pass.set_bind_group(0, self.texture_bind_group_metadatas.bind_group(), &[]);
            render_pass.set_bind_group(1, self.camera_metadatas.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    //- Getters ------------------------------------------------------------------------------------

    ///
    pub fn texture_bind_group_metadatas(&self) -> &crate::renderer::TextureBindGroupMetadatas {
        &self.texture_bind_group_metadatas
    }
}
