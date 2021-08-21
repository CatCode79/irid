
//= RENDERER STRUCT ================================================================================

///
pub struct Renderer {
    config: std::rc::Rc<crate::app::Config>,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    pub(crate) device: std::rc::Rc<wgpu::Device>,
    pub(crate) queue: wgpu::Queue,
    pub(crate) swap_chains: Vec<crate::renderer::SwapChain>,
    pub(crate) pipelines: Vec<crate::renderer::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,  // TODO forse questo devo spostarlo in render_pass o pipeline
    num_vertices: u32,
}


impl Renderer {
    pub fn new(
        window: &winit::window::Window,
        config: &std::rc::Rc<crate::app::Config>,
        vertices: &[crate::vertex::Vertex]
    ) -> Self {
        //window.fullscreen  TODO
        let size = window.inner_size();

        // Context for all other wgpu objects
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // Handle to a presentable surface onto which rendered images
        let surface = unsafe { instance.create_surface(window) };

        // The device is an open connection to a graphics and/or compute device responsible
        // for the creation of most rendering and compute resources.
        // The queue executes recorded CommandBuffer and writes to buffers and textures.
        let (device, queue) = {
            // For debug purpose prints on console all the available adapters
            enumerate_all_adapters(&instance);

            // Adapter can be used to open a connection to the corresponding graphical device
            let adapter = futures::executor::block_on(async {
                instance.request_adapter(
                    &wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::HighPerformance,
                        compatible_surface: Some(&surface),
                    }
                ).await
            }).unwrap();  // todo Result check

            futures::executor::block_on(async {
                adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("New Device & Queue"),
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                    },
                    None, // Trace path
                ).await
            }).unwrap() // todo Result check
        };

        let rc_device = std::rc::Rc::new(device);

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: crate::texture::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = crate::renderer::SwapChain::new(&rc_device, &surface, swap_chain_desc);

        use wgpu::util::DeviceExt;
        let vertex_buffer = rc_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        let num_vertices = vertices.len() as u32;

        Self {
            config: std::rc::Rc::clone(&config),
            size,
            surface,
            device: rc_device,
            queue,
            swap_chains: vec![swap_chain],
            pipelines: vec![],
            vertex_buffer,
            num_vertices,
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
            sc.update(&self.surface, self.size);
        }
    }

    //- Pipeline Methods ---------------------------------------------------------------------------

    ///
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
        uniforms: crate::uniform::Uniforms
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
        self.device.create_command_encoder(
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

                render_pass.set_pipeline(self.pipelines.get(0).unwrap().expose_wrapped_render_pipeline());  // TODO: avoid get and unwrap
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.draw(0..3, 0..1);
            }

            self.queue.submit(std::iter::once(encoder.finish()));
        }

        Ok(())
    }
}


/// Show all the adapters information for debug.
//#[cfg(debug_assertions)]
fn enumerate_all_adapters(instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(wgpu::BackendBit::all()) {
        use log::info;
        info!("{:#?}\n", adapter.get_info())
    }
}
