//= USES ===========================================================================================

use irid_assets_traits::Vertex;
use irid_renderer_traits::VertexFormat;

//= MODEL VERTEX ===================================================================================

/// This is the Vertex Trait main implementation.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

impl Vertex for ModelVertex {
    type F = VertexFormat;

    fn vertex_formats() -> &[Self::F; 3] {
        &[
            VertexFormat::Float32x3,
            VertexFormat::Float32x2,
            VertexFormat::Float32x3
        ]
    }
}

//= COLORED VERTEX =================================================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for ColorVertex {
    type F = VertexFormat;

    fn vertex_formats() -> [Self::F; 2] {
        [
            VertexFormat::Float32x3,
            VertexFormat::Float32x3
        ]
    }
}

//= TEXTURED VERTEX ================================================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextCoordsVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}


impl Vertex for TextCoordsVertex {
    type F = VertexFormat;

    fn vertex_formats() -> [Self::F; 2] {
        [
            VertexFormat::Float32x3,
            VertexFormat::Float32x2,
        ]
    }
}

//= MESH CREATION ==================================================================================

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
