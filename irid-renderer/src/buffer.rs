//= USES ===========================================================================================

//use irid_renderer_traits::Vertex;

//= VERTEX BUFFER LAYOUT ===========================================================================

pub struct VertexBufferLayout<'a, Vertex>;

impl<'a, Vertex> VertexBufferLayout<'a, Vertex> {
    fn desc() -> wgpu::VertexBufferLayout<'a> {
        // TODO better an iter map?
        let mut offset = 0;
        let mut loc = 0;
        let mut attributes = &vec![];
        for vformat in Vertex::vertex_formats() {
            attributes.push(wgpu::VertexAttribute {
                offset: 0,
                shader_location: loc,
                format: wgpu::VertexFormat::Float32x3,
            });
            loc = loc + 1;
        }

        let mut offset = 0;
        let mut i = -1;
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                Vertex::vertex_formats().into_iter().map(|x| wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: i + 1,
                    format: wgpu::VertexFormat::Float32x3,
                }).collect()

/*                wgpu::VertexAttribute {  // position
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {  // tex_coords
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {  // normal
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },*/

            ],
        }
    }
}
