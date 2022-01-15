//= USES ===========================================================================================

use irid_assets_interface::{Image, ImageSize, Texture, TextureError};

use crate::DiffuseImage;

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Clone, Debug)]
pub struct DiffuseTexture<S: ImageSize + Copy> {
    image: DiffuseImage<S>,
}

impl<S> Texture<S> for DiffuseTexture<S>
where
    S: ImageSize + Copy,
{
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;
    type Img = DiffuseImage<S>;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load<P: AsRef<std::path::Path>>(filepath: P) -> Result<Self, TextureError> {
        Ok(Self {
            image: Self::Img::load(filepath)?,
        })
    }

    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P,
    ) -> Result<Self, TextureError> {
        Ok(Self {
            image: Self::Img::load_with_guessed_format(filepath)?,
        })
    }

    //- Getters ------------------------------------------------------------------------------------

    fn image(&self) -> &Self::Img {
        &self.image
    }

    fn size(&self) -> S {
        self.image.size()
    }
}
