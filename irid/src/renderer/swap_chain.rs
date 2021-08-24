
//= SWAPCHAIN WRAPPER ==============================================================================

/// A SwapChain represents the image or series of images that will be presented to a Surface.
pub struct SwapChain<'a> {
    device: &'a crate::renderer::Device<'a>,
    desc: wgpu::SwapChainDescriptor,
    wgpu_swap_chain: wgpu::SwapChain,
}


impl<'a> SwapChain<'a> {
    ///
    pub fn new(device: &'a crate::renderer::Device, desc: wgpu::SwapChainDescriptor) -> Self {
        let wgpu_swap_chain = device.expose_wgpu_device()
            .create_swap_chain(&device.surface, &desc);

        Self {
            device,
            desc,
            wgpu_swap_chain,
        }
    }

    ///
    pub fn update(&mut self, surface: &wgpu::Surface, size: winit::dpi::PhysicalSize<u32>) {
        self.desc.width = size.width;
        self.desc.height = size.height;
        self.wgpu_swap_chain = self.device.expose_wgpu_device()
            .create_swap_chain(surface, &self.desc);
    }

    ///
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.wgpu_swap_chain.get_current_frame()
    }
}
