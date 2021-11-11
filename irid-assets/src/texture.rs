//= USES ===========================================================================================

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
    // TODO I have to create the metas in static manner and after the surface/device creation, so I can create a texture without use those parameters (maybe after that we can move this object inside irid-assets crate)
    fn load(image: &I) -> anyhow::Result<Self::Output> {
        Ok(Self {
            image
        })
    }

    // TODO: to be used instead of  dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    fn as_bytes(&self) -> Option<&[u8]> {
        self.image.as_rgba8_bytes()
    }
}
