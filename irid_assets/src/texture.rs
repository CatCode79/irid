//= USES ===========================================================================================

use irid_assets_interface::{Image, ImageSize, Texture, TextureError};

use crate::DiffuseImage;

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Clone, Debug)]
pub struct DiffuseTexture<S: ImageSize + Copy> {
    image: DiffuseImage<S>,
}

impl<S> Texture<S> for DiffuseTexture<S> where S: ImageSize + Copy {
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

    // TODO: use instead dynamic_image.as_rgba8_bytes on queue.create_texture after IridQueue refact
    fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }

    fn size(&self) -> S {
        self.image.size()
    }
}
