
//= DEVICE WRAPPER =================================================================================

///
pub struct Device {
    wgpu_device: std::rc::Rc<wgpu::Device>,
}


impl Device {
    /// The device is an open connection to a graphics and/or compute device responsible
    /// for the creation of most rendering and compute resources.
    /// The queue executes recorded CommandBuffer and writes to buffers and textures.
    pub fn new(surface: &crate::renderer::Surface) -> (Self, wgpu::Queue) {
        let (wgpu_device, queue) = futures::executor::block_on(async {
            surface.get_adapter().request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("New Device & Queue"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            ).await
        }).unwrap(); // todo Result check

        let device = Self {
            wgpu_device: std::rc::Rc::new(wgpu_device),
        };
        (device, queue)
    }

    ///
    pub fn create_vertex_buffer_init(
        &self,
        label_text: &str,
        vertices: &[crate::meshes::VertexTexture]
    ) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }

    ///
    pub fn create_indices_buffer_init(&self, label_text: &str, indices: &[u16]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;
        self.wgpu_device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(label_text),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }

    ///
    pub fn expose_wgpu_device(&self) -> &std::rc::Rc<wgpu::Device> {
        &self.wgpu_device
    }
}