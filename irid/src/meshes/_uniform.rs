
//= USES ===========================================================================================

use crate::renderer::{Camera, OPENGL_TO_WGPU_MATRIX};


//= UNIFORMS =======================================================================================

// We need this for Rust to store our data correctly for the assets.shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    model_view_proj: [[f32; 4]; 4],
}


impl Uniforms {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            model_view_proj: cgmath::Matrix4::identity().into(),
        }
    }
}

//= UNIFORM STAGING ================================================================================

/**
 * We can create a separate buffer and copy it's contents to our uniform_buffer.
 * The new buffer is known as a staging buffer.
 * This method is usually how it's done as it allows the contents of the main buffer
 * (in this case uniform_buffer) to only be accessible by the gpu.
 * The gpu can do some speed optimizations which it couldn't if we could access the buffer
 * via the cpu.
 */
pub struct UniformStaging {
    pub camera: Camera,
    pub model_rotation: cgmath::Deg<f32>,
}


impl UniformStaging {
    /**
     *
     */
    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            model_rotation: cgmath::Deg(0.0),
        }
    }

    /**
     *
     */
    pub fn update_uniforms(&self, uniforms: &mut Uniforms) {
        uniforms.model_view_proj = (OPENGL_TO_WGPU_MATRIX
            * self.camera.build_view_projection_matrix()
            * cgmath::Matrix4::from_angle_z(self.model_rotation))
            .into();
    }

    /**
     *
     */
    #[inline(always)]
    pub fn update_camera(&mut self, new_aspect_ratio: f32) {
        self.camera.aspect = new_aspect_ratio;
    }
}


//= FNS ============================================================================================

pub fn create_buffer_init(
    renderer: &crate::renderer::Renderer,
    label_text: &str,
    uniforms: Uniforms
)-> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    renderer.device.expose_wgpu_device().create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some(label_text),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    )
}


pub fn create_bind_group_layout(
    renderer: &crate::renderer::Renderer,
    label_text: &str
)-> wgpu::BindGroupLayout {
    renderer.device.expose_wgpu_device().create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some(label_text),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    // Because we only really need camer information in the vertex shader,
                    // as that's what we'll use to manipulate our vertices
                    visibility: wgpu::ShaderStages::VERTEX,
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
    )
}


pub fn create_bind_group(
    renderer: &crate::renderer::Renderer,
    label_text: &str,
    layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer
) -> wgpu::BindGroup {
    renderer.device.expose_wgpu_device().create_bind_group(
        &wgpu::BindGroupDescriptor {
            label: Some(label_text),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        }
    )
}
