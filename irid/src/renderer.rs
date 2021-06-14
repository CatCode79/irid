
//= STRUCTS ========================================================================================

pub struct Renderer {
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}


impl Renderer {
    pub fn new(window: &winit::window::Window) -> Self {
        //window.fullscreen  TODO
        let size = window.inner_size();

        // Context for all other wgpu objects
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        // Handle to a presentable surface onto which rendered images
        let surface = unsafe { instance.create_surface(window) };

        // Adapter can be used to open a connection to the corresponding graphical device
        enumerate_all_adapters(&instance);
        let adapter = futures::executor::block_on(async {
            instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            }).await
        }).unwrap();  // todo None check

        // The device is an open connection to a graphics and/or compute device responsible
        // for the creation of most rendering and compute resources.
        // The queue executes recorded CommandBuffer and writes to buffers and textures.
        let (device, queue) = futures::executor::block_on(async {
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("New Device & Queue"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            ).await
        }).unwrap();  // todo None check

        // A SwapChain represents the image or series of images that will be presented to a Surface.
        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: crate::texture::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        Self {
            size,
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
        }
    }

    //- Physical Size Methods ----------------------------------------------------------------------

    /**
     * Getter for the windows's physical size attribute.
     */
    #[inline]
    pub fn get_size(&self) -> crate::window::PhysicalSize {
        self.size
    }

    /**
     * Setter for the windows's physical size attribute.
     */
    #[inline]
    pub fn set_size(&mut self, new_size: crate::window::PhysicalSize) {
        self.size = new_size;
    }

    /**
     * Calculate the aspect ratio of the window's inner size.
     */
    #[inline]
    pub fn calc_aspect_ratio(&self) -> f32 {
        self.size.width as f32 / self.size.height as f32
    }

    /**
     * Resize the renderer window.
     */
    pub(crate) fn resize(&mut self, new_size: &crate::window::PhysicalSize) {
        self.set_size(*new_size);
        self.refresh_current_size();
    }

    #[inline]
    pub(crate) fn refresh_current_size(&mut self) {
        self.update_swap_chain();
    }

    //- Pipeline Methods ---------------------------------------------------------------------------

    /**
     *
     */
    pub fn create_pipeline_layout(
        &self,
        label_text: &str,
        bind_group_layouts: &[&wgpu::BindGroupLayout]
    ) -> wgpu::PipelineLayout {
        self.device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(label_text),
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        )
    }

    /**
     *
     */
    pub fn create_render_pipeline(
        &self,
        label_text: &str,
        render_pipeline_layout: &wgpu::PipelineLayout,
        vs_module: &wgpu::ShaderModule,
        fs_module: &wgpu::ShaderModule
    ) -> wgpu::RenderPipeline {
        self.device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(label_text),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[crate::vertex::Vertex::desc(), crate::instance::InstanceRaw::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: self.swap_chain_desc.format,
                        write_mask: wgpu::ColorWrite::ALL,
                        blend: Option::from(wgpu::BlendState::REPLACE),
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Option::from(wgpu::Face::Back),
                    clamp_depth: false,
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            }
        )
    }

    //- Swap Chain Methods -------------------------------------------------------------------------

    /**
     *
     */
    pub fn update_swap_chain(&mut self) {
        self.swap_chain_desc.width = self.size.width;
        self.swap_chain_desc.height = self.size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
    }

    /**
     *
     */
    #[inline]
    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.swap_chain.get_current_frame()
    }

    //- Queue Methods ------------------------------------------------------------------------------

    /**
     *
     */
    #[inline]
    pub fn add_buffer_to_queue(
        &self,
        uniform_buffer: &wgpu::Buffer,
        offset: u64,
        uniforms: crate::uniform::Uniforms
    ) {
        self.queue.write_buffer(&uniform_buffer, offset, bytemuck::cast_slice(&[uniforms]));
    }

    /**
     *
     */
    #[inline]
    pub fn submit_command_buffers(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
    }

    //- Command Encoder Methods --------------------------------------------------------------------

    /**
     *
     */
    pub fn create_command_encoder(&self, label_text: &str) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some(label_text),
            }
        )
    }
}


/**
 * Show all the adapters information for debug.
 */
#[cfg(debug_assertions)]
fn enumerate_all_adapters(instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(wgpu::BackendBit::all()) {
        use log::info;
        info!("{:#?}\n", adapter.get_info())
    }
}
