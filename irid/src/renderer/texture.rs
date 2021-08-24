//= USES ===========================================================================================

use image::{DynamicImage, EncodableLayout};


//= CONSTS =========================================================================================

// TODO farlo come funzione per ricavare, anche solo per debug, che cosa è preferito dal device
// Most images are stored using sRGB so we need to reflect that here.
pub(crate) const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= Texture ========================================================================================

pub struct Texture<'a> {
    diffuse_image: DynamicImage,
    //pub data: &'a [u8],
    pub size: wgpu::Extent3d,
    pub texture: wgpu::ImageCopyTexture<'a>,
    pub data_layout: wgpu::ImageDataLayout,
//    pub view: wgpu::TextureView,
}


// TODO: per via di device questa funzione è meglio spostarla in irid::Device o quello che sarà
impl<'a> Texture<'a> {
    pub fn new(device: &crate::renderer::Device, filepath: &str) -> Self{
        let diffuse_image = image::io::Reader::open(filepath).unwrap().decode().unwrap();
        //let data = diffuse_image.as_rgba8().unwrap().as_bytes();

        let image_dimensions = {
            use image::GenericImageView;
            diffuse_image.dimensions()
        };

        // TODO: se la size fosse sempre la stessa potrei creare una Texture di default da device, che viene pure passata tramite indirizzo
        let size = wgpu::Extent3d {
            width: image_dimensions.0,
            height: image_dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture by setting depth to 1
            depth_or_array_layers: 1,
        };

        //let texture = ;

        let data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * image_dimensions.0),
            rows_per_image: std::num::NonZeroU32::new(image_dimensions.1),
        };

        //let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            diffuse_image,
            //data,
            size,
            texture: wgpu::ImageCopyTexture {
                texture: &device.expose_wgpu_device().create_texture(
                    &wgpu::TextureDescriptor {
                        size: size,
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format: PREFERRED_TEXTURE_FORMAT,
                        // SAMPLED tells wgpu that we want to use this texture in shaders
                        // COPY_DST means that we want to copy data to this texture
                        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                        label: Some("Diffuse Texture"),
                    }
                ),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data_layout,
//            view,
        }
    }

    pub fn get_data(&self) -> &[u8]{
        self.diffuse_image.as_rgba8().unwrap().as_bytes()
    }
}
