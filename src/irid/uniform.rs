
//= USES ===========================================================================================

use crate::irid::camera::Camera;


//= UNIFORMS =======================================================================================

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
}


impl Uniforms {
    pub(crate) fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}


//= FNS ============================================================================================

pub(crate) fn create_bind_group_layout_desc_for_uniforms(label_text: &str) -> wgpu::BindGroupLayoutDescriptor {
    wgpu::BindGroupLayoutDescriptor {
        label: Some(label_text),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                // Because we only really need camera information in the vertex shader,
                // as that's what we'll use to manipulate our vertices
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    // The dynamic field indicates whether this buffer will change size or not.
                    // This is useful if we want to store an array of things in our uniforms.
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],
    }
}
