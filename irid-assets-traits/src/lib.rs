/// Refer to the documentation of the individual signatures in a generic way
/// because the individual implementations may vary in detail.

//= IMAGE ==========================================================================================

/// Trait that describes the generic behavior of an image object.
///
/// # Known Implementations:
///
/// - [irid-assets::DiffuseImage](irid-assets::DiffuseImage)
pub trait Image {
    /// **Associated type** regarding the implementation of this trait.
    type I;

    /// **Associated type** regarding the implementation of the [ImageSize] trait.
    type S;

    /// Open and decode a file to read, format will be guessed from path.
    fn new(filepath: &std::path::Path) -> image::ImageResult<Self::I>;  // TODO utilise anyhow instead, also below

    /// Open and decode a file to read, format will be guessed from content.
    fn new_with_guessed_format(filepath: &std::path::Path) -> image::ImageResult<Self::I>;

    /// Returns a value that implements the [ImageSize](ImageSize) trait.
    fn size(&self) -> Self::S;

    /// The width of this image.
    fn width(&self) -> u32;

    /// The height of this image.
    fn height(&self) -> u32;

    /// Get the bytes from the image as 8bit RGBA.
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;
}

//= IMAGE SIZE =====================================================================================

/// Trait that describes the generic behavior of an image size info object.
///
/// # Known Implementations:
///
/// - [irid-assets::DiffuseImageSize](irid-assets::DiffuseImageSize)
pub trait ImageSize {
    /// Returns the [Image] width.
    fn width(&self) -> u32;

    /// Returns the [Image] height.
    fn height(&self) -> u32;

    /// Returns the [Image] width and height (in that order) as tuple.
    fn as_tuple(&self) -> (u32, u32);
}

//= MODEL ==========================================================================================

///
pub trait Model {
    type Output;

    fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self::Output>;
}

//= TEXTURE ========================================================================================

///
pub trait Texture {
    type Output;

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Output>;

    ///
    fn as_bytes(&self) -> Option<&[u8]>;
}
