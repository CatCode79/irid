//= INSTANCE =================================================================

/// Instances allows us to draw the same object multiple times with different properties
/// (position, orientation, size, color, etcetera).
#[derive(Clone, Debug)]
pub(crate) struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub(crate) fn new(
        position: cgmath::Vector3<f32>,
        rotation: cgmath::Quaternion<f32>,
    ) -> Instance {
        Instance { position, rotation }
    }

    /// Convert an Instance to a structure GPU readable.
    pub(crate) fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}

//= INSTANCE FOR SHADERS =====================================================

/// This is the data that will go into the wgpu::Buffer.
/// We keep these separate so that we can update the Instance as much
/// as we want without needing to mess with quaternions.
/// We only need to update the raw data before we draw.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    ///
    #[allow(dead_code)]
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance.
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance.
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0,
                    // and 1 now, in later tutorials we'll be using 2, 3,
                    // and 4, for Vertex.
                    // We'll start at slot 5 not conflict with them later.
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s.
                // We need to define a slot for each vec4.
                // We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
