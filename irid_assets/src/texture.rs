//= USES =====================================================================

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{DiffuseImage, DiffuseImageSize, Image};

//= TEXTURE ERRORS ===========================================================

#[derive(Debug)]
pub enum TextureError {
    CannotLoad {
        source: image_crate::error::ImageError,
    },
}

impl Display for TextureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureError::CannotLoad { source } => writeln!(f, "Cannot load the image: {}", source),
        }
    }
}

impl Error for TextureError {}

//= DIFFUSE TEXTURE ==========================================================

///
#[derive(Clone, Debug)]
pub struct DiffuseTexture {
    path: std::path::PathBuf,
    image: DiffuseImage,
}

impl DiffuseTexture {
    //- Constructors ---------------------------------------------------------

    ///
    pub fn load<P: AsRef<std::path::Path>>(filepath: P) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: DiffuseImage::load(filepath)
                .map_err(|e| TextureError::CannotLoad { source: e })?,
        })
    }

    pub fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: DiffuseImage::load_with_guessed_format(filepath)
                .map_err(|e| TextureError::CannotLoad { source: e })?,
        })
    }

    //- Getters --------------------------------------------------------------

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
