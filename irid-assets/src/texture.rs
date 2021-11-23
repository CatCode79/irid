//= USES ===========================================================================================

use crate::{DiffuseImage, DiffuseImageSize, Image, ImageSize};

//= TEXTURE INTERFACE ==============================================================================

///
// TODO: create (maybe) a super trait with GenericImage
pub trait Texture<S: ImageSize> {
    type Output: Texture<S>;
    type Img;

    ///
    fn load<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> anyhow::Result<Self::Output>;

    ///
    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> anyhow::Result<Self::Output>;

    ///
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;

    ///
    fn size(&self) -> S;
}

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Debug)]
pub struct DiffuseTexture<S: ImageSize + Copy = DiffuseImageSize> {
    image: DiffuseImage<S>,
}

impl<S: ImageSize + Copy> Texture<S> for DiffuseTexture<S> {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;
    type Img = DiffuseImage<S>;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> anyhow::Result<Self> {
        Ok(Self {
            image: Self::Img::load(filepath)?
        })
    }

    fn load_with_guessed_format<P: AsRef<std::path::Path>>(
        filepath: P
    ) -> anyhow::Result<Self> {
        Ok(Self {
            image: Self::Img::load_with_guessed_format(filepath)?
        })
    }

    // TODO: to be used instead dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }

    fn size(&self) -> S {
        self.image.size()
    }
}

//= TEXTURE SIZE INTERFACE =========================================================================

// TODO: aliases and wrapper from GenericImageSize
