/**
 * bytemuck::Pod indicates that our Vertex is "Plain Old Data", and can be interpreted as a &[u8].
 * bytemuck::Zeroable indicates that we can use std::mem::zeroed().
 */

//= TRAIT ==========================================================================================

/*pub trait Vertex {

}*/


//= COLORED VERTEX =================================================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexColor {
    pub position: [f32; 3],
    pub color: [f32; 3],
}


impl VertexColor {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x3,
        },
    ];

    ///
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexColor>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VertexColor::ATTRIBUTES,
        }
    }
}


//= TEXTURED VERTEX ================================================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexTexture {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}


impl VertexTexture {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = [
        wgpu::VertexAttribute {
            offset: 0,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x3,
        },
        wgpu::VertexAttribute {
            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x2,
        },
    ];

    ///
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexTexture>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &VertexTexture::ATTRIBUTES,
        }
    }
}


//= FNS ============================================================================================

/*
pub fn create_polygon(_num_vertices: u16) -> (Vec<Vertex>, Vec<u16>) {
    let angle = std::f32::consts::PI * 2.0 / num_vertices as f32;
    let vertices = (0..num_vertices).map(|i| {
        let theta = angle * i as f32;
        Vertex {
            position: [0.5 * theta.cos(), -0.5 * theta.sin(), 0.0],
            color: [(1.0 + theta.cos()) / 2.0, (1.0 + theta.sin()) / 2.0, 1.0],
        }
    })
    .collect::<Vec<_>>();

    let indices = (1u16..num_vertices + 2 - 1)
        .into_iter()
        .flat_map(|i| vec![i + 1, i, 0])
        .collect::<Vec<_>>();

    (vertices, indices)
}
*/
