//= USES ===========================================================================================

use image::EncodableLayout;
use wgpu::TextureFormat;

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

    // Rgba8UnormSrgb is the format most supported by the gpus.
    // TODO: need to return Result and remove unwrap()
    fn image_bytes(&self, format: wgpu::TextureFormat) -> &[u8] {
        match format {
            TextureFormat::Rgba8Unorm => self.image.as_rgba8().unwrap().as_bytes(),
            TextureFormat::Rgba8UnormSrgb => self.image.as_rgba8().unwrap().as_bytes(),
            TextureFormat::Bgra8Unorm => self.image.as_bgra8().unwrap().as_bytes(),
            TextureFormat::Bgra8UnormSrgb => self.image.as_bgra8().unwrap().as_bytes(),
            _ => {
                panic!("TextureFormat {:?} not yet supported", format)
            }
        }
    }

    fn size(&self) -> S {
        self.image.size()
    }
}
