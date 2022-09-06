//= MODS =====================================================================

pub(crate) mod image;
pub(crate) mod model;
pub(crate) mod texture;
pub(crate) mod vertex;

//= USES =====================================================================

pub use self::image::*;
pub use self::model::*;
pub use self::texture::*;
pub use self::vertex::*;

use std::{convert::TryFrom, num::TryFromIntError};

//= IMAGE TRAIT ==============================================================

/// Trait that describes the generic behavior of an image object.
///
/// # Known Implementations:
///
/// - [irid_assets::DiffuseImage](irid_assets::DiffuseImage)
pub trait Image {
    /// **Associated type** regarding the construction.
    type Output: Image;
    /// **Associated type** regarding the image size.
    type Size: ImageSize;

    /// Open and decode a file to read, format will be guessed from path.
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> image_crate::ImageResult<Self::Output>;

    /// Open and decode a file to read, format will be guessed from content.
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> image_crate::ImageResult<Self::Output>;

    /// Returns a value that implements the [ImageSize](ImageSize) trait.
    fn size(&self) -> Self::Size;

    /// Return bytes from the image as 8bit-Rgba format.
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;
}

//= IMAGE SIZE TRAIT =========================================================

/// Trait that describes the generic behavior of an image size info object.
///
/// # Known Implementations:
///
/// - [irid_assets::DiffuseImageSize](irid_assets::DiffuseImageSize)
pub trait ImageSize: From<(u32, u32)> + From<[u32; 2]> {
    ///
    fn new(width: u32, height: u32) -> Option<Self>;

    ///
    fn new_unchecked(width: u32, height: u32) -> Self;

    ///
    fn try_new(width: u32, height: u32) -> Result<Self, TryFromIntError>;

    /// Returns the [Image] width.
    fn width(&self) -> u32;

    /// Returns the [Image] height.
    fn height(&self) -> u32;

    /// Returns the [Image] width and height (in that order) as tuple.
    fn as_tuple(&self) -> (u32, u32);
}

//= VERTEX TRAIT =============================================================

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

//= INDEX TRAIT ==============================================================

/// Super Trait to identify u16 and u32
// TODO: possibly we can do it simpler than that
pub trait Index: Default + PartialEq + From<u8> + TryFrom<u64> {}

// Nothing to implement, since u16 and u32 already supports the other traits.
impl Index for u16 {}
impl Index for u32 {}
