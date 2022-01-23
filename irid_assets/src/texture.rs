//= USES ===========================================================================================

use thiserror::Error;

use irid_assets_interface::Image;

use crate::{DiffuseImage, DiffuseImageSize};

//= TEXTURE ERRORS =================================================================================

#[non_exhaustive]
#[derive(Debug, Error)] // TODO: impossible to add Clone because of image::error::ImageError
pub enum TextureError {
    #[error("Cannot load the image")]
    CannotLoad {
        #[from]
        source: image::error::ImageError,
    },
}

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Clone, Debug)]
pub struct DiffuseTexture {
    path: std::path::PathBuf,
    image: DiffuseImage,
}

impl DiffuseTexture {
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn load<P: AsRef<std::path::Path>>(filepath: P) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: DiffuseImage::load(filepath)?,
        })
    }

    pub fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: DiffuseImage::load_with_guessed_format(filepath)?,
        })
    }

    //- Getters ------------------------------------------------------------------------------------

    pub fn path(&self) -> &std::path::PathBuf {
        &self.path
    }

    pub fn image(&self) -> &DiffuseImage {
        &self.image
    }

    pub fn size(&self) -> DiffuseImageSize {
        self.image.size()
    }
}
