//= USES ===========================================================================================

use anyhow::anyhow;
use crate::renderer::Adapter;


//= SURFACE WRAPPER ================================================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto which rendered images
/// may be presented.
pub struct Surface(wgpu::Surface, wgpu::TextureFormat, wgpu::SurfaceConfiguration);


impl Surface {
    /// Create a new Surface using the window handle and retrieves an Adapter which matches
    /// the created surface.
    pub fn new(
        backends: wgpu::Backends,
        window: &winit::window::Window,
        size: winit::dpi::PhysicalSize<u32>
    ) -> anyhow::Result<(Self, Adapter)> {
        // Context for all other wgpu objects
        let wgpu_instance = wgpu::Instance::new(backends);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window) };

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter = crate::renderer::Adapter::new(&wgpu_instance, &wgpu_surface)?;

        #[cfg(debug_assertions)]
        println!("Picked Adapter: {:?}", adapter.get_info());

        // Most images are stored using sRGB so we need to reflect that here.
        //let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;  // TODO must be choosable by user
        let preferred_format = wgpu_surface.get_preferred_format(
            adapter.expose_wrapped_adapter()
        );
        if preferred_format.is_none() {
            return Err(anyhow!("Surface incompatible with adapter {:?}: no preferred format was found",
                adapter.get_info()));
        }
        let preferred_format = preferred_format.unwrap();

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off".
            // Mailbox is a hybrid between the two (gpu doesn't block if running faster
            // than the display, but screen tearing doesn't happen)
            present_mode: wgpu::PresentMode::Fifo,  // TODO: to be choosable by the user
        };

        let surface = Self {
            0: wgpu_surface,
            1: preferred_format,
            2: configuration,
        };

        Ok((surface, adapter))
    }

    /// Initializes Surface for presentation.
    pub fn configure(&self, device: &crate::renderer::Device) {
        self.0.configure(device.expose_wgpu_device(), &self.2);
    }

    /// Updates the Surface for presentation.
    pub fn update(&mut self, device: &crate::renderer::Device, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.2.width = size.width;
            self.2.height = size.height;
            self.0.configure(&device.expose_wgpu_device(), &self.2);
        }
    }

    /// Returns the next texture to be presented by the Surface for drawing.
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SurfaceFrame, wgpu::SurfaceError> {
        self.0.get_current_frame()
    }

    //- Wrapped Methods ----------------------------------------------------------------------------

    /// Returns an optimal texture format to use for with the previously created Surface
    /// and Adapter.
    pub fn get_preferred_format(&self) -> wgpu::TextureFormat {
        self.1
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
