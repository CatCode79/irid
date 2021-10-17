
//= USES ===========================================================================================

use anyhow::anyhow;


//= ADAPTER WRAPPER ================================================================================

/// Handle to a physical graphics and/or compute device.
///
/// Adapters can be used to open a connection to the corresponding [`Device`]
/// on the host system by using [`Adapter::request_device`].
pub struct Adapter(wgpu::Adapter);


impl Adapter {
    /// Retrieves an Adapter which matches the given surface.
    /// Some options are "soft", so treated as non-mandatory. Others are "hard".
    /// If no adapters are found that suffice all the "hard" options, Err is returned.
    pub fn new(
        wgpu_instance: &wgpu::Instance,
        wgpu_surface: &wgpu::Surface
    ) -> anyhow::Result<Self> {
        let wgpu_adapter = pollster::block_on(async {
            // About force_fallback_adapter: https://github.com/gfx-rs/wgpu/issues/2063
            wgpu_instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,  // TODO maybe better to give power of choice
                    compatible_surface: Some(&wgpu_surface),
                }
            ).await
        });

        match wgpu_adapter {
            None => Err(anyhow!("An adapter compatible with the given surface could not be obtained")),
            Some(wgpu_adapter) => Ok(Self {
                0: wgpu_adapter,
            })
        }
    }

    //- Wrapped Methods ----------------------------------------------------------------------------

    /// Requests a connection to a physical device, creating a logical device.
    ///
    /// Returns the Device together with a Queue that executes command buffers.
    ///
    /// # Arguments
    ///
    /// - `desc` - Description of the features and limits requested from the given device.
    /// - `trace_path` - Can be used for API call tracing, if that feature is
    ///   enabled in `wgpu-core`.
    ///
    /// # Panics
    ///
    /// - Features specified by `desc` are not supported by this adapter.
    /// - Unsafe features were requested but not enabled when requesting the adapter.
    /// - Limits requested exceed the values provided by the adapter.
    /// - Adapter does not support all features wgpu requires to safely operate.
    pub fn request_device(
        &self,
        desc: &wgpu::DeviceDescriptor,
        trace_path: Option<&std::path::Path>
    ) -> impl std::future::Future<Output =
    anyhow::Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError>> + Send {
        self.0.request_device(desc, trace_path)
    }

    /// Get info about the adapter itself.
    pub fn get_info(&self) -> wgpu::AdapterInfo {
        self.0.get_info()
    }

    ///- Crate-Public Methods ----------------------------------------------------------------------

    // This method MUST remain public at the crate level.
    pub(crate) fn expose_wrapped_adapter(&self) -> &wgpu::Adapter {
        &self.0
    }
}
