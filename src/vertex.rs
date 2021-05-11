//= STRUCTS ========================================================================================

/**
 Pod indicates that our Vertex is "Plain Old Data", and thus can be interpretted as a &[u8].
 Zeroable indicates that we can use std::mem::zeroed().
*/
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}


//= IMPLS ==========================================================================================

impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ],
        }
    }
}


//= CONSTS =========================================================================================

/**
 We arrange the vertices in counter clockwise order: top, bottom left, bottom right.
 We do it this way partially out of tradition, but mostly because we specified
 in the rasterization_state of the render_pipeline that we want the front_face of our triangle
 to be wgpu::FrontFace::Ccw so that we cull the back face.
 This means that any triangle that should be facing us should have its vertices
 in counter clockwise order.
 */
/*pub const VERTICES: &[Vertex] = &[
    Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];*/

pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.08682410,  0.49240386, 0.0], color: [0.10, 0.0, 0.50] },  // 0
    Vertex { position: [-0.49513406,  0.06958647, 0.0], color: [0.20, 0.0, 0.40] },  // 1
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.25, 0.0, 0.25] },  // 2
    Vertex { position: [ 0.35966998, -0.34732910, 0.0], color: [0.40, 0.0, 0.50] },  // 3
    Vertex { position: [ 0.44147372,  0.23473590, 0.0], color: [0.50, 0.0, 0.10] },  // 4
];

pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
