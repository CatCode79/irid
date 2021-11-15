//= USES ===========================================================================================

use std::path::Path;

use irid_assets_traits::{Image, Texture};

//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Debug)]
pub struct DiffuseTexture<I: Image> {
    image: I,
}

impl<I: Image> Texture for DiffuseTexture<I> {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = Self;

    //- Constructors -------------------------------------------------------------------------------

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Output> {
        Ok(Self {
            image: I::load(filepath)?
        })
    }

    fn load_with_guessed_format(filepath: &Path) -> anyhow::Result<Self::Output> {
        Ok(Self {
            image: I::load_with_guessed_format(filepath)?
        })
    }

    // TODO: to be used instead dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    fn as_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }
}
