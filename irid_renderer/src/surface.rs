//= USES =====================================================================

use pollster::FutureExt;
use thiserror::Error;

use crate::device::Device;

//= ERRORS ===================================================================

#[derive(Debug, Error)]
pub(crate) enum SurfaceError {
    #[error("An adapter compatible with the given surface could not be obtained")]
    AdapterNotObtained,
}

//= SURFACE WRAPPER ==========================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto
/// which rendered images may be presented.
#[derive(Debug)]
pub(crate) struct Surface {
    wgpu_surface: wgpu::Surface,
    format: wgpu::TextureFormat,
    configuration: wgpu::SurfaceConfiguration,
}

impl Surface {
    //- Constructors ---------------------------------------------------------

    /// Create a new Surface using the window handle and retrieves an Adapter
    /// which matches the created surface.
    pub(crate) fn new(
        backends: wgpu::Backends,
        window: &winit::window::Window,
        power_preference: wgpu::PowerPreference,
        force_fallback_adapter: bool,
        preferred_format: Option<wgpu::TextureFormat>,
        present_mode: wgpu::PresentMode,
    ) -> Result<(Self, wgpu::Adapter), SurfaceError> {
        // Context for all other wgpu objects
        let wgpu_instance = wgpu::Instance::new(backends);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window) };

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter = {
            let adapter_options = wgpu::RequestAdapterOptions {
                power_preference,
                force_fallback_adapter,
                compatible_surface: Some(&wgpu_surface),
            };

            let adapter_option =
                async { wgpu_instance.request_adapter(&adapter_options).await }.block_on();

            if let Some(a) = adapter_option {
                Ok(a)
            } else {
                Err(SurfaceError::AdapterNotObtained)
            }
        }?;

        log::info!("Picked Adapter: {}", pprint_adapter_info(&adapter));

        let format =
            preferred_format.unwrap_or_else(|| wgpu_surface.get_supported_formats(&adapter)[0]);

        log::info!("Preferred Texture Color Format: {:?}", format);

        let window_size = window.inner_size();

        let configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window_size.width,
            height: window_size.height,
            present_mode,
        };

        let surface = Self {
            wgpu_surface,
            format,
            configuration,
        };

        Ok((surface, adapter))
    }

    //- Getters --------------------------------------------------------------

    /// Returns the optimal texture format to use with this Surface.
    pub(crate) fn format(&self) -> wgpu::TextureFormat {
        self.format
    }

    // Swapchain -------------------------------------------------------------

    /// Initializes Surface for presentation.
    pub(crate) fn configure(&self, device: &Device) {
        self.wgpu_surface
            .configure(device.expose_wrapped_device(), &self.configuration);
    }

    /// Updates the Surface for presentation.
    pub(crate) fn update(&mut self, device: &Device, size: winit::dpi::PhysicalSize<u32>) {
        self.configuration.width = size.width;
        self.configuration.height = size.height;
        self.wgpu_surface
            .configure(device.expose_wrapped_device(), &self.configuration);
    }

    /// Returns the next texture to be presented by the Surface for drawing.
    #[inline(always)]
    pub(crate) fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.wgpu_surface.get_current_texture()
    }
}

//= FUNCTIONS ================================================================

// Shows all the adapters information.
fn enumerate_all_adapters(backends: wgpu::Backends, instance: &wgpu::Instance) {
    let _ = instance.poll_all(true);
    let adapters = instance.enumerate_adapters(backends);

    let mut found = false;
    for (i, adapter) in adapters.enumerate() {
        let info = pprint_adapter_info(&adapter);
        if i == 0 {
            log::info!("Adapter(s) found - {}", info);
        } else {
            log::info!("                 - {}", info);
        }
        found = true;
    }

    if !found {
        log::info!("No Adapter Found");
    }
}

// Wgpu adapter info pretty printing.
fn pprint_adapter_info(adapter: &wgpu::Adapter) -> String {
    format!("{:?}", adapter.get_info())
        .replace("AdapterInfo { name: ", "")
        .replace(" }", "")
}
