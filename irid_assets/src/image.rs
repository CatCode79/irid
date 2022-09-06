//= USES =====================================================================

use std::convert::TryFrom;
use std::num::{NonZeroU32, TryFromIntError};

use crate::{Image, ImageSize};

//= DIFFUSE IMAGE ============================================================

/// A Diffuse Image
#[derive(Clone, Debug)]
pub struct DiffuseImage {
    image: image_crate::DynamicImage,
    size: DiffuseImageSize,
}

impl DiffuseImage {
    //- Constructor Handler --------------------------------------------------

    fn handle_new<P: AsRef<std::path::Path>>(
        filepath: P,
        guess_the_format: bool,
    ) -> image_crate::ImageResult<Self> {
        let file_reader = if guess_the_format {
            image_crate::io::Reader::open(filepath)?.with_guessed_format()?
        } else {
            image_crate::io::Reader::open(filepath)?
        };

        let image = file_reader.decode()?;

        let size = {
            use image_crate::GenericImageView;
            image.dimensions().into()
        };

        Ok(Self { image, size })
    }
}

impl Image for DiffuseImage {
    //- Associated Types -----------------------------------------------------

    type Output = Self;
    type Size = DiffuseImageSize;

    //- Constructors ---------------------------------------------------------

    /// Open and decode a file to read, format will be guessed from path.
    ///
    /// If you want to inspect the content for a better guess on the format,
    /// which does not depend on file extensions, see
    /// [new_with_guessed_format](DiffuseImage::new_with_guessed_format).
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> image_crate::ImageResult<Self> {
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
        filepath: P,
    ) -> image_crate::ImageResult<Self> {
        DiffuseImage::handle_new(filepath, true)
    }

    //- Getters --------------------------------------------------------------

    /// The width and height of this image.
    fn size(&self) -> Self::Size {
        self.size
    }

    //- Color Data Conversions -----------------------------------------------

    fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        use image_crate::EncodableLayout;
        match self.image.as_rgba8() {
            None => None,
            Some(rgba8) => Some(rgba8.as_bytes()),
        }
    }
}

//= DIFFUSE IMAGE SIZE =======================================================

#[derive(Clone, Copy, Debug)]
pub struct DiffuseImageSize {
    width: NonZeroU32,
    height: NonZeroU32,
}

impl ImageSize for DiffuseImageSize {
    //- Constructors ---------------------------------------------------------

    fn new(width: u32, height: u32) -> Option<Self> {
        if width == 0 {
            return None;
        }
        if height == 0 {
            return None;
        }
        Some(Self {
            width: NonZeroU32::new(width).unwrap(),
            height: NonZeroU32::new(height).unwrap(),
        })
    }

    fn new_unchecked(width: u32, height: u32) -> Self {
        Self {
            width: NonZeroU32::new(width).unwrap(),
            height: NonZeroU32::new(height).unwrap(),
        }
    }

    fn try_new(width: u32, height: u32) -> Result<Self, TryFromIntError> {
        Ok(Self {
            width: NonZeroU32::try_from(width)?,
            height: NonZeroU32::try_from(height)?,
        })
    }

    //- Getters --------------------------------------------------------------

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
        Self::new_unchecked(tuple.0, tuple.1)
    }
}

impl From<[u32; 2]> for DiffuseImageSize {
    fn from(array: [u32; 2]) -> Self {
        Self::new_unchecked(array[0], array[1])
    }
}

// These newtypes must exist due to this compiler error:
// "[E0117] Only traits defined in the current crate can be implemented for arbitrary types"
pub(crate) struct ImageSizeTuple((u32, u32));
pub(crate) struct ImageSizeArray([u32; 2]);

impl From<ImageSizeTuple> for Option<DiffuseImageSize> {
    fn from(tuple: ImageSizeTuple) -> Self {
        DiffuseImageSize::new(tuple.0 .0, tuple.0 .1)
    }
}

impl From<ImageSizeArray> for Option<DiffuseImageSize> {
    fn from(array: ImageSizeArray) -> Self {
        DiffuseImageSize::new(array.0[0], array.0[1])
    }
}
