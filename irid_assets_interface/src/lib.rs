//= USES ===========================================================================================

use std::convert::TryFrom;

//= VERTEX TRAIT ===================================================================================

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
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

//= INDEX TRAIT ====================================================================================

/// Opinable Super Trait to identify u16 and u32
// TODO: probably i can do better and simpler than that!
pub trait Index: Default + PartialEq + From<u8> + TryFrom<u64> {}

// Nothing to implement, since u16 and u32 already supports the other traits.
impl Index for u16 {}
impl Index for u32 {}
