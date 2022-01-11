//= USES ===========================================================================================

use std::convert::TryFrom;

use thiserror::Error;

//= ERRORS =========================================================================================

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TextureError {
    #[error("can’t identify any monitor as a primary one")]
    ImageError {
        #[from]
        source: image::error::ImageError,
    },
}

//= IMAGE TRAIT ====================================================================================

/// Trait that describes the generic behavior of an image object.
///
/// # Known Implementations:
///
/// - [irid_assets::DiffuseImage](irid_assets::DiffuseImage)
pub trait Image<S: ImageSize> {
    /// **Associated type** regarding the implementation of this trait.
    type Output;

    /// Open and decode a file to read, format will be guessed from path.
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> image::ImageResult<Self::Output>;

    /// Open and decode a file to read, format will be guessed from content.
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> image::ImageResult<Self::Output>;

    /// Returns a value that implements the [ImageSize](ImageSize) trait.
    fn size(&self) -> S;

    /// Get the bytes from the image as 8bit RGBA.
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;
}

//= IMAGE SIZE TRAIT ===============================================================================

/// Trait that describes the generic behavior of an image size info object.
///
/// # Known Implementations:
///
/// - [irid_assets::DiffuseImageSize](irid_assets::DiffuseImageSize)
pub trait ImageSize: From<(u32, u32)> + From<[u32; 2]> {
    ///
    fn new(width: u32, height: u32) -> Self;

    /// Returns the [Image] width.
    fn width(&self) -> u32;

    /// Returns the [Image] height.
    fn height(&self) -> u32;

    /// Returns the [Image] width and height (in that order) as tuple.
    fn as_tuple(&self) -> (u32, u32);
}

//= TEXTURE TRAIT ==================================================================================

///
// TODO: create (maybe) a super trait with GenericImage
pub trait Texture<S: ImageSize> {
    type Output: Texture<S>;
    type Img;

    ///
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> Result<Self::Output, TextureError>;

    ///
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> Result<Self::Output, TextureError>;

    ///
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;

    ///
    fn size(&self) -> S;
}

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
