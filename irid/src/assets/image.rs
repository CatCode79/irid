
//= DYNAMIC IMAGE ==================================================================================

pub struct DynamicImage(image::DynamicImage, u32, u32);

impl DynamicImage {
    pub fn load(filepath: &std::path::Path) -> image::ImageResult<Self> {
        // TODO: cercare di fare a meno dell'altro unwrap
        let dynamic_image = image::io::Reader::open(filepath).unwrap().decode()?;

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

    /// Return a reference to an 8bit RGBA image.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        use image::EncodableLayout;
        match self.0.as_rgba8() {
            None => { None }
            Some(rgba8) => { Some(rgba8.as_bytes()) }
        }
    }

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
