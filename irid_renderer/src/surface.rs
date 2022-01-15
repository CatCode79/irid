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
#[derive(Debug)]
pub struct Surface {
    wgpu_surface: wgpu::Surface,
    format: wgpu::TextureFormat,
    configuration: wgpu::SurfaceConfiguration,
}

impl Surface {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new Surface using the window handle and retrieves an Adapter which matches
    /// the created surface.
    pub fn new<W: Window>(
        backends: wgpu::Backends,
        window: &W,
        power_preference: wgpu::PowerPreference,
        force_fallback_adapter: bool,
        preferred_format: Option<wgpu::TextureFormat>,
        present_mode: wgpu::PresentMode,
    ) -> Result<(Self, Adapter), SurfaceError> {
        // Context for all other wgpu objects
        let wgpu_instance = wgpu::Instance::new(backends);

        // Handle to a presentable surface onto which rendered images
        let wgpu_surface = unsafe { wgpu_instance.create_surface(window.expose_inner_window()) };

        // For debug purpose prints on console all the available adapters
        #[cfg(debug_assertions)]
        enumerate_all_adapters(backends, &wgpu_instance);

        let adapter_options = wgpu::RequestAdapterOptions {
            power_preference,
            force_fallback_adapter,
            compatible_surface: Some(&wgpu_surface),
        };
        let adapter = Adapter::new(&wgpu_instance, adapter_options)
            /*.or_else(|e| Err(SurfaceError::AdapterNotObtained))*/?;

        #[cfg(debug_assertions)]
        println!(
            "Picked Adapter: {}",
            pprint_adapter_info(adapter.expose_wrapped_adapter())
        );

        let format = preferred_format.unwrap_or({
            wgpu::TextureFormat::Rgba8UnormSrgb
            // This part is commented out because if by chance the format is returned as
            // Bgra8UnormSrgb then we need to convert all textures to that format, which is
            // currently performance-heavy, if I'm not wrong, in the current crate image API.
            // Ideally, it would be an on-the-fly conversion while loading the image.
            // This eventual improvement is to be considered a very low priority thing to-do
            // and for this reason it is not even labeled as such.
            /*
            wgpu_surface
                .get_preferred_format(adapter.expose_wrapped_adapter())
                // Most images are stored using sRGB so we need to reflect that here.
                .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb)
            */
        });

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

    //- Getters ------------------------------------------------------------------------------------

    /// Returns the optimal texture format to use with this Surface.
    pub fn format(&self) -> wgpu::TextureFormat {
        self.format
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

// Shows all the adapters information for debug.
#[cfg(debug_assertions)]
fn enumerate_all_adapters(backends: wgpu::Backends, instance: &wgpu::Instance) {
    instance.poll_all(true);
    let adapters = instance.enumerate_adapters(backends);

    let mut found = false;
    for (i, adapter) in adapters.enumerate() {
        let info = pprint_adapter_info(&adapter);
        if i == 0 {
            println!("Adapter(s) found - {}", info);
        } else {
            println!("                 - {}", info);
        }
        found = true;
    }

    if !found {
        println!("No Adapter Found");
    }
}

// Wgpu adapter info pretty printing.
#[cfg(debug_assertions)]
fn pprint_adapter_info(adapter: &wgpu::Adapter) -> String {
    format!("{:?}", adapter.get_info())
        .replace("AdapterInfo { name: ", "")
        .replace(" }", "")
}
