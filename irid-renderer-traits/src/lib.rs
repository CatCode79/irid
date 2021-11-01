///

//= TEXTURE ========================================================================================

/*///
pub trait Texture {
    ///
    fn load(surface: &Surface, device: &Device, filepath: &std::path::Path) -> anyhow::Result<Self>;

    ///
    fn as_bytes(&self) -> Option<&[u8]>;
}*/

//= VERTEX =========================================================================================

///
pub trait Vertex {
    ///
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
