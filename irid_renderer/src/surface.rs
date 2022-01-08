//= USES ===========================================================================================

use thiserror::Error;

use irid_app_interface::Window;

use crate::{adapter::Adapter, device::Device, AdapterError};

//= ERRORS =========================================================================================

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum SurfaceError {
    #[error("no preferred format was found: Surface incompatible with adapter {:?}", .0)]
    NoPreferredFormat(wgpu::AdapterInfo),
    #[error("An adapter compatible with the given surface could not be obtained")]
    AdapterNotObtained {
        #[from]
        source: AdapterError,
    },
}

//= SURFACE WRAPPER ================================================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto which rendered images
/// may be presented.
pub struct Surface {
    wgpu_surface: wgpu::Surface,
    preferred_format: wgpu::TextureFormat,
    configuration: wgpu::SurfaceConfiguration,
}

impl Surface {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new Surface using the window handle and retrieves an Adapter which matches
    /// the created surface.
    pub fn new<W: Window>(
        backends: wgpu::Backends,
        window: &W,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Result<(Self, Adapter), SurfaceError> {
        // Context for all other wgpu objects
        let wgpu_instance = wgpu::Instance::new(backends);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window.expose_inner_window()) };

        // For debug purpose prints on console all the available adapters
        #[cfg(debug_assertions)]
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter = pollster::block_on(Adapter::new(&wgpu_instance, &wgpu_surface))
            /*.or_else(|e| Err(SurfaceError::AdapterNotObtained))*/?;

        #[cfg(debug_assertions)]
        println!("Picked Adapter: {:?}", adapter.get_info());

        // Most images are stored using sRGB so we need to reflect that here.
        //let preferred_format = wgpu::TextureFormat::Rgba8UnormSrgb;  // TODO: to be made also user-choosable
        let preferred_format = wgpu_surface
            .get_preferred_format(adapter.expose_wrapped_adapter())
            .ok_or_else(|| SurfaceError::NoPreferredFormat(adapter.get_info()))?;

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            width: size.width,
            height: size.height,
            // Fifo is "vsync on". Immediate is "vsync off".
            // Mailbox is a hybrid between the two (gpu doesn't block if running faster
            // than the display, but screen tearing doesn't happen)
            present_mode: wgpu::PresentMode::Fifo, // TODO user choosable
        };

        let surface = Self {
            wgpu_surface,
            preferred_format,
            configuration,
        };

        Ok((surface, adapter))
    }

    //- Getters ------------------------------------------------------------------------------------

    /// Returns an optimal texture format to use for with the previously created Surface
    /// and Adapter.
    pub fn preferred_format(&self) -> wgpu::TextureFormat {
        self.preferred_format
    }

    // Swapchain -----------------------------------------------------------------------------------

    /// Initializes Surface for presentation.
    pub fn configure(&self, device: &Device) {
        self.wgpu_surface
            .configure(device.expose_wrapped_device(), &self.configuration);
    }

    /// Updates the Surface for presentation.
    pub fn update(&mut self, device: &Device, size: winit::dpi::PhysicalSize<u32>) {
        self.configuration.width = size.width;
        self.configuration.height = size.height;
        self.wgpu_surface
            .configure(device.expose_wrapped_device(), &self.configuration);
    }

    /// Returns the next texture to be presented by the Surface for drawing.
    #[inline(always)]
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.wgpu_surface.get_current_texture()
    }
}

//= FUNCTIONS ======================================================================================

/// Shows all the adapters information for debug.
#[cfg(debug_assertions)]
fn enumerate_all_adapters(backends: wgpu::Backends, instance: &wgpu::Instance) {
    instance.poll_all(true);
    for adapter in instance.enumerate_adapters(backends) {
        println!("Adapter found: {:?}", adapter.get_info());
    }
}
