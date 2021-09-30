
//= DEVICE WRAPPER =================================================================================

/// Open connection to a graphics and/or compute device.
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
                    limits: wgpu::Limits::downlevel_defaults(),  // TODO: farlo configurabile lato utente
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

    ///
    #[inline]
    pub fn expose_wgpu_device(&self) -> &wgpu::Device {
        &self.0
    }
}
