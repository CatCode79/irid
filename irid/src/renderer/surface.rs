//= USES ===========================================================================================

use anyhow::anyhow;

use crate::renderer::{Adapter, Device};


//= SURFACE WRAPPER ================================================================================

/// A Surface represents a platform-specific surface (e.g. a window) onto which rendered images
/// may be presented.
pub struct Surface {
    wgpu_surface: wgpu::Surface,
    preferred_format: wgpu::TextureFormat,
    configuration: wgpu::SurfaceConfiguration,
    color_target_states: [wgpu::ColorTargetState; 1],
}


impl Surface {
    //- Constructor Methods ------------------------------------------------------------------------

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

        let adapter = Adapter::new(&wgpu_instance, &wgpu_surface)?;

        #[cfg(debug_assertions)]
        println!("Picked Adapter: {:?}", adapter.get_info());

        // Most images are stored using sRGB so we need to reflect that here.
        //let preferred_format = wgpu::TextureFormat::Rgba8UnormSrgb;  // TODO must be choosable by user
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

        let color_target_states = [wgpu::ColorTargetState {
            format: preferred_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }];

        let surface = Self {
            wgpu_surface,
            preferred_format,
            configuration,
            color_target_states,
        };

        Ok((surface, adapter))
    }

    //- Getter Methods -----------------------------------------------------------------------------

    /// Returns an optimal texture format to use for with the previously created Surface
    /// and Adapter.
    pub fn get_preferred_format(&self) -> wgpu::TextureFormat {
        self.preferred_format
    }

    /// Return an array with a [ColorTargetState](wgpu::ColorTargetState) single value,
    /// it's a default value mainly used on
    /// [FragmentStateBuilder](crate::shader::FragmentStateBuilder).
    pub fn color_target_states(&self) -> &[wgpu::ColorTargetState] {
        &self.color_target_states
    }

    // Swapchain Methods ---------------------------------------------------------------------------

    /// Initializes Surface for presentation.
    pub fn configure(&self, device: &Device) {
        self.wgpu_surface.configure(device.expose_wrapped_device(), &self.configuration);
    }

    /// Updates the Surface for presentation.
    pub fn update(&mut self, device: &Device, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.configuration.width = size.width;
            self.configuration.height = size.height;
            self.wgpu_surface.configure(&device.expose_wrapped_device(), &self.configuration);
        }
    }

    /// Returns the next texture to be presented by the Surface for drawing.
    #[inline(always)]
    pub fn get_current_frame(&self) -> Result<wgpu::SurfaceFrame, wgpu::SurfaceError> {
        self.wgpu_surface.get_current_frame()
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
