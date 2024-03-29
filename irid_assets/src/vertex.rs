//= USES =====================================================================

use crate::Vertex;

//= MODEL VERTEX =============================================================

/// This is the Vertex Trait main implementation.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn new() -> Self {
        Self::default()
    }

    fn position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    fn colors(&mut self, _: [f32; 3]) {}

    fn tex_coords(&mut self, tex_coords: [f32; 2]) {
        self.tex_coords = tex_coords;
    }

    fn normal(&mut self, normal: [f32; 3]) {
        self.normal = normal
    }

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tex_coords
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

//= COLORED VERTEX ===========================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub colors: [f32; 3],
}

impl Vertex for ColorVertex {
    fn new() -> Self {
        Self::default()
    }

    fn position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    fn colors(&mut self, colors: [f32; 3]) {
        self.colors = colors
    }

    fn tex_coords(&mut self, _: [f32; 2]) {}

    fn normal(&mut self, _: [f32; 3]) {}

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

//= TEXTURED VERTEX ==========================================================

///
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextCoordsVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex for TextCoordsVertex {
    fn new() -> Self {
        Self::default()
    }

    fn position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    fn colors(&mut self, _: [f32; 3]) {}

    fn tex_coords(&mut self, tex_coords: [f32; 2]) {
        self.tex_coords = tex_coords;
    }

    fn normal(&mut self, _: [f32; 3]) {}

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TextCoordsVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tex_coords
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

//= MESH CREATION ============================================================

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
