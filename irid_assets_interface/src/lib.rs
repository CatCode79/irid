pub trait Vertex {
    ///
    fn new() -> Self;

    ///
    fn position(&mut self, position: [f32; 3]);

    ///
    fn colors(&mut self, colors: [f32; 3]);

    ///
    fn tex_coords(&mut self, tex_coords: [f32; 2]);

    ///
    fn normal(&mut self, normal: [f32; 3]);

    ///
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}
