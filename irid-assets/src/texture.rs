//= USES ===========================================================================================

use std::path::Path;

use crate::{DiffuseImage, DiffuseImageSize, GenericImage};

//= TEXTURE INTERFACE ==============================================================================

///
// TODO: create a super trait with GenericImage
pub trait GenericTexture {
    type Output;
    type Img;
    type ImgSz;

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Output>;

    ///
    fn load_with_guessed_format(filepath: &std::path::Path) -> anyhow::Result<Self::Output>;

    ///
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;

    ///
    fn size(&self) -> Self::ImgSz;
}

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Debug)]
pub struct DiffuseTexture {
    image: DiffuseImage,
}

impl GenericTexture for DiffuseTexture {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;
    type Img = DiffuseImage;
    type ImgSz = DiffuseImageSize;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Output> {
        Ok(Self {
            image: Self::Img::load(filepath)?
        })
    }

    fn load_with_guessed_format(filepath: &Path) -> anyhow::Result<Self::Output> {
        Ok(Self {
            image: Self::Img::load_with_guessed_format(filepath)?
        })
    }

    // TODO: to be used instead dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    fn as_rgba8_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }

    fn size(&self) -> DiffuseImageSize {
        self.image.size()
    }
}

//= TEXTURE SIZE INTERFACE =========================================================================

// TODO: aliases and wrapper from GenericImageSize
