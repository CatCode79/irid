//= VERTEX =========================================================================================

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
