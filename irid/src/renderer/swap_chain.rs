
//= SWAP CHAIN WRAPPER =============================================================================

/// A SwapChain represents the image or series of images that will be presented to a Surface.
pub(crate) struct SwapChain {
    swap_chain_desc: wgpu::SwapChainDescriptor,
    wgpu_swap_chain: wgpu::SwapChain,
}


impl SwapChain {
    pub(crate) fn new(device: &wgpu::Device, surface: &wgpu::Surface, swap_chain_desc: wgpu::SwapChainDescriptor) -> Self {
        let wgpu_swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
        Self {
            swap_chain_desc,
            wgpu_swap_chain,
        }
    }

    pub(crate) fn update(&mut self, device: &wgpu::Device, surface: &wgpu::Surface, size: winit::dpi::PhysicalSize<u32>) {
        self.swap_chain_desc.width = size.width;
        self.swap_chain_desc.height = size.height;
        self.wgpu_swap_chain = device.create_swap_chain(surface, &self.swap_chain_desc);
    }

    #[inline(always)]
    pub(crate) fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.wgpu_swap_chain.get_current_frame()
    }
}
