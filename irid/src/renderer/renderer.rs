
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
    pub(crate) surface: crate::renderer::Surface,
    pub(crate) device: crate::renderer::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) pipeline: crate::renderer::RenderPipeline,
    texture_metadatas: crate::renderer::TextureMetadatas,
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

        let diffuse_image = crate::assets::DynamicImage::new(texture_path);

        let (diffuse_image_width, diffuse_image_height) = {
            let dimensions = diffuse_image.dimensions().unwrap();
            (dimensions.0, dimensions.1)
        };

        let texture_metadatas =
            crate::renderer::TextureMetadatas::new(&device, diffuse_image_width, diffuse_image_height);

        let pipeline = crate::renderer::RenderPipeline::new(
            &device,
            &texture_metadatas,
            shader_source
        );

        // TODO decisamente bisognerà fare qualche cosa con questi passaggi di parametri e clones
        queue.write_texture(
            texture_metadatas.new_image_copy(),
            diffuse_image.as_bytes().unwrap(),  // TODO: piace poco l'unwrap
            texture_metadatas.clone_image_data_layout(),
            texture_metadatas.clone_image_size()
        );

        let vertex_buffer = device.create_vertex_buffer_init("Vertex Buffer", vertices);
        let index_buffer = device.create_indices_buffer_init("Index Buffer", indices);

        let num_indices = indices.len() as u32;

        Self {
            config: std::rc::Rc::clone(&config),
            size,
            surface,
            device,
            queue,
            pipeline,
            texture_metadatas,
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

    //- Queue Methods ------------------------------------------------------------------------------

    ///
    #[inline]
    pub fn add_buffer_to_queue(
        &self,
        uniform_buffer: &wgpu::Buffer,
        offset: u64,
        uniforms: crate::meshes::Uniforms
    ) {
        self.queue.write_buffer(&uniform_buffer, offset, bytemuck::cast_slice(&[uniforms]));
    }


    //- Command Encoder Methods --------------------------------------------------------------------

    ///
    #[inline(always)]
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device.expose_wgpu_device().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            }
        )
    }

    //- Rendering Methods --------------------------------------------------------------------------

    pub(crate) fn redraw(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_frame()?.output;  // Must be let binded!
        let frame_view = output
            .texture.create_view(&FRAME_TEXTURE_VIEW);
            //.texture.create_view(&wgpu::TextureViewDescriptor::default());  // (TT)

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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
