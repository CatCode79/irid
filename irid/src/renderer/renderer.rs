
//= RENDERER STRUCT ================================================================================

use std::rc::Rc;
use crate::renderer::TextureMetaDatas;

///
pub struct Renderer<'a> {
    config: std::rc::Rc<crate::app::Config>,
    size: winit::dpi::PhysicalSize<u32>,
    pub(crate) device: crate::renderer::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) swap_chains: Vec<crate::renderer::SwapChain>,
    pub(crate) pipelines: Vec<crate::renderer::RenderPipeline>,
    texture_meta_datas: TextureMetaDatas<'a>,
    vertex_buffer: wgpu::Buffer,  // TODO forse questo devo spostarlo in render_pass o pipeline
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}


impl<'a> Renderer<'a> {
    pub fn new(
        window: &winit::window::Window,
        config: &Rc<crate::app::Config>,
        shader_source: String,
        texture_path: &str,
        vertices: &[crate::meshes::VertexTexture],
        indices: &[u16]
    ) -> Self {
        //window.fullscreen  TODO
        let size = window.inner_size();

        let (device, queue) = crate::renderer::Device::new(window);

        let swap_chain = crate::renderer::SwapChain::new(&device, size);

        let texture_meta_datas =
            crate::renderer::TextureMetaDatas::new_default_size(&device);

        let pipeline = crate::renderer::RenderPipeline::new(
            &device,
            &texture_meta_datas,
            shader_source
        );

        let texture = crate::renderer::Texture::new(texture_path);

        // TODO decisamente bisognerà fare qualche cosa con questi passaggi di parametri e clones
        queue.write_texture(
            texture_meta_datas.texture.clone(),
            texture.get_data(),
            texture_meta_datas.image_data_layout.clone(),
            texture_meta_datas.size.clone()
        );

        let vertex_buffer = device.create_vertex_buffer_init("Vertex Buffer", vertices);
        let index_buffer = device.create_indices_buffer_init("Index Buffer", indices);

        let num_indices = indices.len() as u32;

        Self {
            config: std::rc::Rc::clone(&config),
            size,
            device,
            queue,
            swap_chains: vec![swap_chain],
            pipelines: vec![pipeline],
            texture_meta_datas,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }

    //- Window Inner Size Methods ------------------------------------------------------------------

    /// Getter for the windows's physical size attribute.
    #[inline]
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Setter for the windows's physical size attribute.
    #[inline]
    pub fn set_size(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
    }

    /// Calculate the aspect ratio of the window's inner size.
    #[inline]
    pub fn calc_aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }

    /// Resize the renderer window.
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.set_size(new_size);
        self.refresh_current_size();
    }

    pub(crate) fn refresh_current_size(&mut self) {
        for sc in self.swap_chains.iter_mut() {
            sc.update(&self.device.surface, self.size);
        }
    }

    //- Pipeline Methods ---------------------------------------------------------------------------

    ///
    #[allow(dead_code)]
    pub(crate) fn add_pipeline(&mut self, pipeline: crate::renderer::RenderPipeline) {
        self.pipelines.push(pipeline);
    }

    //- Buffer's Methods ---------------------------------------------------------------------------


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


    ///
    #[inline]
    pub fn submit_command_buffers(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    //- Command Encoder Methods --------------------------------------------------------------------

    ///
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device.expose_wgpu_device().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            }
        )
    }

    //- Rendering Methods --------------------------------------------------------------------------

    pub(crate) fn redraw(&self) -> Result<(), wgpu::SwapChainError> {
        for sc in self.swap_chains.iter() {
            let frame = sc.get_current_frame()?.output;

            let mut encoder = self.create_command_encoder("Render Encoder");

            {
                let mut render_pass = encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(self.config.clear_color),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    }
                );

                render_pass.set_pipeline(self.pipelines.get(0).unwrap().expose_wrapped_render_pipeline());  // TODO: avoid get and unwrap overhead
                render_pass.set_bind_group(0, &self.texture_meta_datas.diffuse_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }

            self.queue.submit(std::iter::once(encoder.finish()));
        }

        Ok(())
    }
}
