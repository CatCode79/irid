//= USES ===========================================================================================

use std::num::NonZeroU32;

use irid_assets_traits::{Image, ImageSize};

//= DYNAMIC IMAGE ==================================================================================

/// A Diffuse Image
///
/// It is a wrapper of [image::DynamicImage](image::DynamicImage) object.
#[derive(Clone, Debug)]
pub struct DiffuseImage {
    image: image::DynamicImage,
    size: ImageSize,
}

impl DiffuseImage {

    //- Constructor Handler ------------------------------------------------------------------------

    fn handle_new(filepath: &std::path::Path, guess_the_format:bool) -> image::ImageResult<Self> {
        let file_reader = if guess_the_format {
            image::io::Reader::open(filepath)?.with_guessed_format()?  // TODO: use anyhow context instead, also below
        } else {
            image::io::Reader::open(filepath)?
        };

        let image = file_reader.decode()?;

        let size = {
            use image::GenericImageView;
            ImageSize::from(image.dimensions())
        };

        Ok(Self {
            image,
            size,
        })
    }
}

impl Image for DiffuseImage {

    //- Associated Types ---------------------------------------------------------------------------

    type Img = Self;
    type ImgSz = DiffuseImageSize;

    //- Constructors -------------------------------------------------------------------------------

    /// Open and decode a file to read, format will be guessed from path.
    ///
    /// If you want to inspect the content for a better guess on the format,
    /// which does not depend on file extensions, see
    /// [new_with_guessed_format](DynamicImage::new_with_guessed_format).
    fn new(filepath: &std::path::Path) -> image::ImageResult<Self> {
        DiffuseImage::handle_new(filepath, false)
    }

    /// Open and decode a file to read, format will be guessed from path first
    /// (like the [DynamicImage::new](DynamicImage::new) method) and then make a format guess
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
    fn new_with_guessed_format(filepath: &std::path::Path) -> image::ImageResult<Self> {
        DiffuseImage::handle_new(filepath, true)
    }

    //- Getters ------------------------------------------------------------------------------------

    /// The width and height of this image.
    fn size(&self) -> &Self::ImgSz {
        &self.size
    }

    /// The width of this image.
    fn width(&self) -> u32 {
        self.size.width()
    }

    /// The height of this image.
    fn height(&self) -> u32 {
        self.size.height()
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

//= DIFFUSE IMAGE SIZE =============================================================================

#[derive(Clone, Copy, Debug)]
struct DiffuseImageSize {
    width: NonZeroU32,
    height: NonZeroU32,
}

impl ImageSize for DiffuseImageSize {

    //- Constructors -------------------------------------------------------------------------------



    //- Getters ------------------------------------------------------------------------------------

    fn width(&self) -> u32 {
        self.width.unwrap()
    }

    fn height(&self) -> u32 {
        self.height.unwrap()
    }

    fn as_tuple(&self) -> (u32, u32) {
        (self.width.unwrap(), self.height.unwrap())
    }
}


impl From<((u32, u32))> for DiffuseImageSize {  // TODO it's actually works?
    fn from(tuple: (u32, u32)) -> Self {
        ImageSize::new(tuple.0, tuple.1)
    }
}

impl From<[(u32, u32); 2]> for DiffuseImageSize {
    fn from(tuple: (u32, u32)) -> Self {
        ImageSize::new(tuple.0, tuple.1)
    }
}
