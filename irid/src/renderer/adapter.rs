
//= ADAPTER WRAPPER ================================================================================

pub struct Adapter(wgpu::Adapter);

impl Adapter {
    pub fn new(wgpu_instance: &wgpu::Instance, wgpu_surface: &wgpu::Surface) -> Option<Self> {
        let wgpu_adapter = pollster::block_on(async {
            wgpu_instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&wgpu_surface),
                }
            ).await
        })?;

        Some(Self {
            0: wgpu_adapter,
        })
    }

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
    ) -> impl std::future::Future<Output = anyhow::Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError>> + Send {
        self.0.request_device(desc, trace_path)
    }
}
