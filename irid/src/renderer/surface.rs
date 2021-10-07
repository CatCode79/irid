//= CONSTS =========================================================================================

// TODO: ricavarlo a runtime, anche solo per debug, dal device. Ci sono delle perplessità
//  relativamente alla uniformità dei valori floati cui colori si comportano.
// Most images are stored using sRGB so we need to reflect that here.
pub(crate) const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= SURFACE WRAPPER ================================================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto which rendered images
/// may be presented.
pub struct Surface(wgpu::Surface, wgpu::SurfaceConfiguration);


impl Surface {
    /// Create a new Surface and a Adapter using a window.
    pub fn new(
        backends: wgpu::Backends,
        window: &winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>
    ) -> anyhow::Result<(Self, crate::renderer::Adapter)> {
        // Context for all other wgpu objects
        let wgpu_instance = wgpu::Instance::new(backends);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window) };

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            //format: wgpu_surface.get_preferred_format(&adapter).unwrap(),  // TODO: mi ha dato problemi
            format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off".
            // Mailbox is a hybrid between the two (gpu doesn't block if running faster
            // than the display, but screen tearing doesn't happen)
            present_mode: wgpu::PresentMode::Fifo,  // TODO: far scegliere al giocatore prima dell'avvio del gioco
        };

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter = crate::renderer::Adapter::new(&wgpu_instance, &wgpu_surface)?;

        #[cfg(debug_assertions)]
        println!("Picked Adapter: {:?}", adapter.get_info());

        let surface = Self {
            0: wgpu_surface,
            1: configuration,
        };

        Ok((surface, adapter))
    }

    /// Initializes Surface for presentation.
    pub fn configure(&self, device: &crate::renderer::Device) {
        self.0.configure(device.expose_wgpu_device(), &self.1);
    }

    /// Update the Surface for presentation.
    pub fn update(&mut self, device: &crate::renderer::Device, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.1.width = size.width;
            self.1.height = size.height;
            self.0.configure(&device.expose_wgpu_device(), &self.1);
        }
    }

    /// Returns the next texture to be presented by the swapchain for drawing.
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SurfaceFrame, wgpu::SurfaceError> {
        self.0.get_current_frame()
    }
}


//= FUNCTIONS ======================================================================================

/// Show all the adapters information for debug.
#[cfg(debug_assertions)]
fn enumerate_all_adapters(backends: wgpu::Backends, instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(backends) {
        println!("Adapter found: {:?}", adapter.get_info());
    }
}
