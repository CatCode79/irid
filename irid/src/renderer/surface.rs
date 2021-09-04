//= CONSTS =========================================================================================

// TODO: ricavarlo a runtime, anche solo per debug, dal device. Ci sono delle perplessità
//  relativamente alla uniformità dei valori floati cui colori si comportano.
// Most images are stored using sRGB so we need to reflect that here.
pub(crate) const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= SURFACE WRAPPER ================================================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto which rendered images
/// may be presented.
pub struct Surface(wgpu::Surface, wgpu::Adapter, wgpu::SurfaceConfiguration);


impl Surface {
    /// See wgpu::Backends for the complete list.
    pub const SUPPORTED_BACKENDS: wgpu::Backends =
        wgpu::Backends::VULKAN /*| wgpu::Backends::DX12 | wgpu::Backends::GL*/;

    ///
    pub fn new(
        window: &winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>
    ) -> Self {
        // Context for all other wgpu objects
        let instance = wgpu::Instance::new(Surface::SUPPORTED_BACKENDS);

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(&instance);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { instance.create_surface(window) };

        let adapter = futures::executor::block_on(async {
            instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&wgpu_surface),
                }
            ).await
        }).unwrap();  // TODO: ritornare il risultato

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off".
            // Mailbox is a hybrid between the two (gpu doesn't block if running faster
            // than the display, but screen tearing doesn't happen)
            present_mode: wgpu::PresentMode::Fifo,  // TODO: far scegliere al giocatore prima dell'avvio del gioco
        };

        Self {
            0: wgpu_surface,
            1: adapter,
            2: configuration,
        }
    }

    ///
    pub fn update(&mut self, device: &crate::renderer::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.2.width = size.width;
        self.2.height = size.height;
        self.0.configure(&device.expose_wgpu_device(), &self.2);
    }

    ///
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SurfaceFrame, wgpu::SurfaceError> {
        self.0.get_current_frame()
    }

    /// Adapter can be used to open a connection to the corresponding graphical device.
    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.1
    }
}


// TODO: diamine non funziona..
/// Show all the adapters information for debug.
//#[cfg(debug_assertions)]
fn enumerate_all_adapters(instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(wgpu::Backends::all()) {
        use log::info;
        info!("{:#?}\n", adapter.get_info())
    }
}
