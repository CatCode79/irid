//= USES ===========================================================================================

use std::path::Path;

use irid_assets_traits::{GenericImage, GenericTexture};

use crate::image::DiffuseImageSize;

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Debug)]
pub struct DiffuseTexture<I: GenericImage> {
    image: I,
}

impl<I: GenericImage + GenericImage<Img= I>> GenericTexture for DiffuseTexture<I> {
    //- Associated Types ---------------------------------------------------------------------------

    type Txtr = Self;
    type ImgSz = DiffuseImageSize;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Txtr> {
        Ok(Self {
            image: I::load(filepath)?
        })
    }

    fn load_with_guessed_format(filepath: &Path) -> anyhow::Result<Self::Txtr> {
        Ok(Self {
            image: I::load_with_guessed_format(filepath)?
        })
    }

    // TODO: to be used instead dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    fn as_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }

    fn size(&self) -> Self::ImgSz {
        self.image.size()
    }
}
