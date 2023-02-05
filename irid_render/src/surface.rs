//= USES =====================================================================

use std::error::Error;
use std::fmt::{Display, Formatter};

use pollster::FutureExt;

use crate::device::Device;

//= SURFACE WRAPPER ==========================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto
/// which rendered images may be presented.
#[derive(Debug)]
pub(crate) struct Surface {
    wgpu_surface: wgpu::Surface,
    capabilities: wgpu::SurfaceCapabilities,
    configuration: wgpu::SurfaceConfiguration,
}

impl Surface {
    //- Constructors ---------------------------------------------------------

    /// Create a new Surface using the window handle and retrieves an Adapter
    /// which matches the created surface.
    pub(crate) fn new(
        backends: wgpu::Backends,
        window: &winit::window::Window,
        present_mode: wgpu::PresentMode,
    ) -> Result<(Self, wgpu::Adapter), SurfaceError> {
        // Context for all other wgpu objects
        let wgpu_instance = {
            let desc = wgpu::InstanceDescriptor {
                backends,
                ..Default::default()
            };
            wgpu::Instance::new(desc)
        };

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = match unsafe { wgpu_instance.create_surface(window) } {
            Ok(s) => Ok(s),
            Err(e) => Err(SurfaceError::Creation(e)),
        }?;

        // For debug purpose prints on console all the available adapters
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter = {
            let adapter_options = wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&wgpu_surface),
            };

            let adapter =
                async { wgpu_instance.request_adapter(&adapter_options).await }.block_on();
            if let Some(a) = adapter {
                Ok(a)
            } else {
                Err(SurfaceError::AdapterNotObtained)
            }
        }?;
        log::info!("Picked Adapter: {}", pprint_adapter_info(&adapter));

        let capabilities = wgpu_surface.get_capabilities(&adapter);
        let (format, view_formats) = get_formats(&capabilities);
        log::info!(
            "Picked Texture Color Format: {:?} from {:?}",
            format,
            capabilities.formats
        );

        let configuration = {
            let window_size = window.inner_size();

            wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: window_size.width,
                height: window_size.height,
                present_mode,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats,
            }
        };

        let surface = Self {
            wgpu_surface,
            capabilities,
            configuration,
        };

        Ok((surface, adapter))
    }

    //- Getters --------------------------------------------------------------

    /// Returns the capabilities related to this Surface.
    #[allow(unused)]
    pub(crate) fn capabilities(&self) -> &wgpu::SurfaceCapabilities {
        &self.capabilities
    }

    /// Returns the surface's configuration, useful to get format and view_formats.
    pub(crate) fn configuration(&self) -> &wgpu::SurfaceConfiguration {
        &self.configuration
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

fn get_formats(
    capabilities: &wgpu::SurfaceCapabilities,
) -> (wgpu::TextureFormat, Vec<wgpu::TextureFormat>) {
    (capabilities.formats[0], vec![])
}

// Wgpu adapter info pretty printing.
fn pprint_adapter_info(adapter: &wgpu::Adapter) -> String {
    format!("{:?}", adapter.get_info())
        .replace("AdapterInfo { name: ", "")
        .replace(" }", "")
}

//= ERRORS ===================================================================

#[derive(Debug)]
pub(crate) enum SurfaceError {
    Creation(wgpu::CreateSurfaceError),
    AdapterNotObtained,
}

impl Display for SurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SurfaceError::Creation(e) => write!(f, "{}", e),
            SurfaceError::AdapterNotObtained => write!(
                f,
                "An adapter compatible with the given surface could not be obtained"
            ),
        }
    }
}

impl Error for SurfaceError {}
