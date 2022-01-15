//= USES ===========================================================================================

use irid_assets_interface::{Image, Texture, TextureError};

use crate::{DiffuseImage, DiffuseImageSize};

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Clone, Debug)]
pub struct DiffuseTexture {
    path: std::path::PathBuf,
    image: DiffuseImage,
}

impl Texture for DiffuseTexture {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;
    type Img = DiffuseImage;
    type Size = DiffuseImageSize;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: Self::Img::load(filepath)?,
        })
    }

    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            path: filepath.as_ref().to_path_buf(),
            image: Self::Img::load_with_guessed_format(filepath)?,
        })
    }

    //- Getters ------------------------------------------------------------------------------------

    fn path(&self) -> &std::path::PathBuf {
        &self.path
    }

    fn image(&self) -> &Self::Img {
        &self.image
    }

    fn size(&self) -> Self::Size {
        self.image.size()
    }
}
