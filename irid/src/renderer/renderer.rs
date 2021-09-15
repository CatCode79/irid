
//= CONSTS =========================================================================================

const FRAME_TEXTURE_VIEW: wgpu::TextureViewDescriptor = wgpu::TextureViewDescriptor {
    label: None,
    format: None,
    dimension: None,
    aspect: wgpu::TextureAspect::All,
    base_mip_level: 0,
    mip_level_count: None,
    base_array_layer: 0,
    array_layer_count: None
};


//= RENDERER STRUCT ================================================================================

///
pub struct Renderer {
    config: std::rc::Rc<crate::app::Config>,
    size: winit::dpi::PhysicalSize<u32>,
    surface: crate::renderer::Surface,
    device: crate::renderer::Device,
    queue: wgpu::Queue,
    texture_metadatas: crate::renderer::TextureMetadatas,
    camera: crate::renderer::Camera,
    camera_metadatas: crate::renderer::CameraMetadatas,
    camera_controller: crate::renderer::CameraController,
    pipeline: crate::renderer::RenderPipeline,
    vertex_buffer: wgpu::Buffer,  // TODO: forse questo devo spostarlo in render_pass o pipeline, anche quello sotto
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}


impl Renderer {
    pub fn new(
        window: &winit::window::Window,
        config: &std::rc::Rc<crate::app::Config>,
        shader_source: String,
        texture_path: &str,
        vertices: &[crate::meshes::VertexTexture],
        indices: &[u16]
    ) -> Self {
        let size = window.inner_size();  // TODO: window.fullscreen at startup

        let surface = crate::renderer::Surface::new(window, size);

        let (device, queue) = crate::renderer::Device::new(&surface);

        surface.configure(&device);

        //- Texture --------------------------------------------------------------------------------

        let diffuse_image = crate::assets::DynamicImage::new(texture_path);

        let (image_width, image_height) = {
            let dimensions = diffuse_image.dimensions().unwrap();
            (dimensions.0, dimensions.1)
        };

        let texture_metadatas =
            crate::renderer::TextureMetadatas::new(&device, image_width, image_height);

        //- Camera ---------------------------------------------------------------------------------

        let camera = crate::renderer::Camera::new(size.width as f32, size.height as f32);
        let camera_metadatas = camera.create_metadatas(&device);
        let camera_controller = crate::renderer::CameraController::new(0.2);

        //- Pipeline -------------------------------------------------------------------------------

        let pipeline = crate::renderer::RenderPipeline::new(
            &device,
            texture_metadatas.bind_group_layout(),
            camera_metadatas.bind_group_layout(),
            shader_source
        );

        //- Queue Schedule -------------------------------------------------------------------------

        // TODO decisamente bisognerà fare qualche cosa con questi passaggi di parametri e clones
        queue.write_texture(
            texture_metadatas.new_image_copy(),
            diffuse_image.as_bytes().unwrap(),  // TODO: piace poco l'unwrap
            texture_metadatas.clone_image_data_layout(),
            texture_metadatas.clone_image_size()
        );

        //- Vertex and Index Buffers ---------------------------------------------------------------

        let vertex_buffer = device.create_vertex_buffer_init("Vertex Buffer", vertices);
        let index_buffer = device.create_indices_buffer_init("Index Buffer", indices);

        let num_indices = indices.len() as u32;

        //- Renderer Creation ----------------------------------------------------------------------

        Self {
            config: std::rc::Rc::clone(&config),
            size,
            surface,
            device,
            queue,
            texture_metadatas,
            camera,
            camera_metadatas,
            camera_controller,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    //- Size Methods -------------------------------------------------------------------------------

    /// Getter for the windows's physical size attribute.
    #[inline]
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Calculate the aspect ratio of the window's inner size.
    #[inline]
    pub fn calc_aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }

    /// Resize the renderer window.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.refresh_current_size();
    }

    #[inline]
    pub(crate) fn refresh_current_size(&mut self) {
        self.surface.update(&self.device, self.size);
    }

    //- Camera Methods -----------------------------------------------------------------------------

    #[inline(always)]
    pub fn process_camera_events(&mut self, input: &winit::event::KeyboardInput) -> bool {
        self.camera_controller.process_events(input)
    }

    //- Command Encoder Methods --------------------------------------------------------------------

    ///
    #[inline(always)]
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device.expose_wgpu_device().create_command_encoder(  // TODO: probabilmente è meglio spostarlo in device
            &wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            }
        )
    }

    //- Rendering Methods --------------------------------------------------------------------------

    pub(crate) fn redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
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
        // TODO: test results with wgpu 0.11: NO TEST PERFORMED

        let output = self.surface.get_current_frame()?.output;
        let frame_view = output.texture.create_view(&FRAME_TEXTURE_VIEW);
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
                            load: wgpu::LoadOp::Clear(self.config.clear_color),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                }
            );

            render_pass.set_pipeline(self.pipeline.expose_wrapped_render_pipeline());
            render_pass.set_bind_group(0, &self.texture_metadatas.bind_group(), &[]);
            render_pass.set_bind_group(1, &self.camera_metadatas.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
