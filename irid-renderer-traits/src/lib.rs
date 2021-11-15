//= VERTEX =========================================================================================

///
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
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

// TODO da togliere da qui e mettere in renderer
/*            let vertex_buffer = device.create_vertex_buffer_init(
                &format!("{:?} Vertex Buffer", path.as_ref()),
                vertices.as_slice(),
            );

            let index_buffer = device.create_indices_buffer_init(
                &format!("{:?} Index Buffer", path.as_ref()),
                obj_model.mesh.indices.as_slice(),
            );*/
