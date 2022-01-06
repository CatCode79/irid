//= USES ===========================================================================================

use std::num::NonZeroU32;

//= IMAGE INTERFACE ================================================================================

/// Trait that describes the generic behavior of an image object.
///
/// # Known Implementations:
///
/// - [irid_assets::DiffuseImage](irid_assets::DiffuseImage)
pub trait Image<S: ImageSize> {
    /// **Associated type** regarding the implementation of this trait.
    type Output;

    /// Open and decode a file to read, format will be guessed from path.
    fn load<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> image::ImageResult<Self::Output>;

    /// Open and decode a file to read, format will be guessed from content.
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> image::ImageResult<Self::Output>;

    /// Returns a value that implements the [ImageSize](ImageSize) trait.
    fn size(&self) -> S;

    /// Get the bytes from the image as 8bit RGBA.
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;
}

//= DIFFUSE IMAGE ==================================================================================

/// A Diffuse Image
#[derive(Clone, Debug)]
pub struct DiffuseImage<S: ImageSize + Copy> {
    image: image::DynamicImage,
    size: S,
}

impl<S: ImageSize + Copy> DiffuseImage<S> {
    //- Constructor Handler ------------------------------------------------------------------------

    fn handle_new<P: AsRef<std::path::Path>>(
        filepath: P,
        guess_the_format:bool
    ) -> image::ImageResult<Self> {
        let file_reader = if guess_the_format {
            image::io::Reader::open(filepath)?.with_guessed_format()?  // TODO: use anyhow context instead, also below
        } else {
            image::io::Reader::open(filepath)?
        };

        let image = file_reader.decode()?;

        let size = {
            use image::GenericImageView;
            image.dimensions().into()
        };

        Ok(Self {
            image,
            size,
        })
    }
}

impl<S: ImageSize + Copy> Image<S> for DiffuseImage<S> {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;

    //- Constructors -------------------------------------------------------------------------------

    /// Open and decode a file to read, format will be guessed from path.
    ///
    /// If you want to inspect the content for a better guess on the format,
    /// which does not depend on file extensions, see
    /// [new_with_guessed_format](DiffuseImage::new_with_guessed_format).
    fn load<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> image::ImageResult<Self> {
        DiffuseImage::handle_new(filepath, false)
    }

    /// Open and decode a file to read, format will be guessed from path first
    /// (like the [new](DiffuseImage::new) method) and then make a format guess
    /// based on the content, replacing it on success.
    ///
    /// If the guess was unable to determine a format then the format from path is used.
    /// Returns Ok with the guess if no io error occurs.
    /// Additionally, replaces the current format if the guess was successful.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying reader fails. The format is unchanged.
    /// The error is a std::io::Error and not ImageError since the only error case is an error
    /// when the underlying reader seeks.
    ///
    /// **When an error occurs, the reader may not have been properly reset and it is potentially
    /// hazardous to continue with more IO operations**.
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> image::ImageResult<Self> {
        DiffuseImage::handle_new(filepath, true)
    }

    //- Getters ------------------------------------------------------------------------------------

    /// The width and height of this image.
    fn size(&self) -> S {
        self.size
    }

    //- Color Data Conversions ---------------------------------------------------------------------

    /// Get the bytes from the image as 8bit RGBA.
    fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        use image::EncodableLayout;
        match self.image.as_rgba8() {
            None => { None }
            Some(rgba8) => { Some(rgba8.as_bytes()) }
        }
    }
}

//= IMAGE SIZE INTERFACE ===========================================================================

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

//= DIFFUSE IMAGE SIZE =============================================================================

#[derive(Clone, Copy, Debug)]
pub struct DiffuseImageSize {
    width: NonZeroU32,
    height: NonZeroU32,
}

impl ImageSize for DiffuseImageSize {
    //- Constructors -------------------------------------------------------------------------------

    fn new(width: u32, height: u32) -> Self {
        Self {
            width: NonZeroU32::new(width).unwrap(),  // TODO: we have to create try_new constructor here, to check the non-zeroity
            height: NonZeroU32::new(height).unwrap(),
        }
    }

    //- Getters ------------------------------------------------------------------------------------

    /// The value is non-zero guaranteed.
    fn width(&self) -> u32 {
        self.width.get()
    }

    /// The value is non-zero guaranteed.
    fn height(&self) -> u32 {
        self.height.get()
    }

    /// The values are non-zero guaranteed.
    fn as_tuple(&self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }
}

impl From<(u32, u32)> for DiffuseImageSize {
    fn from(tuple: (u32, u32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<[u32; 2]> for DiffuseImageSize {
    fn from(array: [u32; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}