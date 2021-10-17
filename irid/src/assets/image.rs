
//= DYNAMIC IMAGE ==================================================================================

/// A Dynamic Image
///
/// It is a wrapper of the [image::DynamicImage](image::DynamicImage) object.
#[derive(Debug)]
pub struct DiffuseImage(image::DynamicImage, u32, u32);


impl DiffuseImage {
    /// Open and decode a file to read, format will be guessed from path.
    ///
    /// If you want to inspect the content for a better guess on the format,
    /// which does not depend on file extensions, see
    /// [new_with_guessed_format](DynamicImage::new_with_guessed_format).
    pub fn new(filepath: &std::path::Path) -> image::ImageResult<Self> {
        DiffuseImage::new_handler(filepath, false)
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
    pub fn new_with_guessed_format(filepath: &std::path::Path) -> image::ImageResult<Self> {
        DiffuseImage::new_handler(filepath, true)
    }

    fn new_handler(filepath: &std::path::Path, guess_the_format:bool) -> image::ImageResult<Self> {
        let file_reader = if guess_the_format {
            image::io::Reader::open(filepath)?.with_guessed_format()?  // TODO: use anyhow context instead, also below
        } else {
            image::io::Reader::open(filepath)?
        };

        let dynamic_image = file_reader.decode()?;

        let image_dimensions = {
            use image::GenericImageView;
            dynamic_image.dimensions()
        };

        Ok(Self {
            0: dynamic_image,
            1: image_dimensions.0,
            2: image_dimensions.1,
        })
    }

    //- Color Data Conversion Methods --------------------------------------------------------------

    /// Get the bytes from the image as 8bit RGBA.
    pub fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        use image::EncodableLayout;
        match self.0.as_rgba8() {
            None => { None }
            Some(rgba8) => { Some(rgba8.as_bytes()) }
        }
    }

    //- Getter Methods -----------------------------------------------------------------------------

    /// The width and height of this image.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.1, self.2)
    }

    /// The width of this image.
    pub fn width(&self) -> u32 {
        self.1
    }

    /// The height of this image.
    pub fn height(&self) -> u32 {
        self.2
    }
}
