
//= DEVICE WRAPPER =================================================================================

/// Open connection to a graphics and/or compute device.
///
/// Responsible for the creation of most rendering and compute resources.
/// These are then used in commands, which are submitted to a [`Queue`](wgpu::Queue).
///
/// A device may be requested from an adapter with
/// [`Adapter::request_device`](Adapter::request_device).
#[derive(Debug)]
pub struct Device(wgpu::Device);


impl Device {
    /// Create a new Device and Queue given ad adapter.
    pub fn new(
        adapter: &crate::renderer::Adapter
    ) -> anyhow::Result<(Self, wgpu::Queue), wgpu::RequestDeviceError> {
        let (wgpu_device, queue) = pollster::block_on(async {
            adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("New Device & Queue"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),  // TODO to be choosable by user
                },
                None, // Trace path
            ).await
        })?;

        let device = Self {
            0: wgpu_device,
        };

        Ok((device, queue))
    }

    /// Creates a vertex Buffer with data to initialize it.
    pub fn create_vertex_buffer_init(
        &self,
        label_text: &str,
        vertices: &[crate::assets::ModelVertex]
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.0.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }

    /// Creates a indices Buffer with data to initialize it.
    pub fn create_indices_buffer_init(&self, label_text: &str, indices: &[u32]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.0.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }

    //- Crate-Public Methods -----------------------------------------------------------------------

    // This method MUST remain public at the crate level.
    pub(crate) fn expose_wgpu_device(&self) -> &wgpu::Device {
        &self.0
    }
}
