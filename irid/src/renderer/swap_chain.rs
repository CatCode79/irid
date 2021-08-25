
//= SWAPCHAIN WRAPPER ==============================================================================

/// A SwapChain represents the image or series of images that will be presented to a Surface.
pub struct SwapChain {
    wgpu_device: std::rc::Rc<wgpu::Device>,
    desc: wgpu::SwapChainDescriptor,
    wgpu_swap_chain: wgpu::SwapChain,
}


impl SwapChain {
    ///
    pub fn new(device: &crate::renderer::Device, size: winit::dpi::PhysicalSize<u32>) -> Self {
        let desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off". Mailbox is a hybrid between the two
            // (gpu doesn't block if running faster than the display, but screen tearing doesn't happen)
            // TODO far scegliere al giocatore prima dell'avvio del gioco
            present_mode: wgpu::PresentMode::Fifo,
        };

        let wgpu_device = std::rc::Rc::clone(device.expose_wgpu_device());
        let wgpu_swap_chain = wgpu_device.create_swap_chain(&device.surface, &desc);

        Self {
            wgpu_device,
            desc,
            wgpu_swap_chain,
        }
    }

    ///
    pub fn update(&mut self, surface: &wgpu::Surface, size: winit::dpi::PhysicalSize<u32>) {
        self.desc.width = size.width;
        self.desc.height = size.height;
        self.wgpu_swap_chain = self.wgpu_device.create_swap_chain(surface, &self.desc);
    }

    ///
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.wgpu_swap_chain.get_current_frame()
    }
}
