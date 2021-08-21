
//= SWAP CHAIN WRAPPER =============================================================================

/// A SwapChain represents the image or series of images that will be presented to a Surface.
pub struct SwapChain {
    device: std::rc::Rc<wgpu::Device>,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    wgpu_swap_chain: wgpu::SwapChain,
}


impl SwapChain {
    pub fn new(device: &std::rc::Rc<wgpu::Device>, surface: &wgpu::Surface, swap_chain_desc: wgpu::SwapChainDescriptor) -> Self {
        let wgpu_swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
        Self {
            device: std::rc::Rc::clone(device),
            swap_chain_desc,
            wgpu_swap_chain,
        }
    }

    pub fn update(&mut self, surface: &wgpu::Surface, size: winit::dpi::PhysicalSize<u32>) {
        self.swap_chain_desc.width = size.width;
        self.swap_chain_desc.height = size.height;
        self.wgpu_swap_chain = self.device.create_swap_chain(surface, &self.swap_chain_desc);
    }

    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SwapChainFrame, wgpu::SwapChainError> {
        self.wgpu_swap_chain.get_current_frame()
    }
}
