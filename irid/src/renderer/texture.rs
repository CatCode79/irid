//= CONSTS =========================================================================================

const DATA_LAYOUT: wgpu::ImageDataLayout = wgpu::ImageDataLayout {
    offset: 0,
    bytes_per_row: std::num::NonZeroU32::new(4 * crate::renderer::DEFAULT_TEXTURE_WIDTH),
    rows_per_image: std::num::NonZeroU32::new(crate::renderer::DEFAULT_TEXTURE_HEIGHT),
};


//= Texture ========================================================================================

pub struct Texture<'a> {
    diffuse_image: image::DynamicImage,
    pub texture: wgpu::ImageCopyTexture<'a>,
    pub data_layout: wgpu::ImageDataLayout,
}


impl<'a> Texture<'a> {
    pub fn new(device: &'a crate::renderer::Device, filepath: &str) -> Self{
        // TODO: potrebbe servirmi ancora per controllare che la diffuse_image sia effettivamente
        //  grande come le struct di default create da Device, prob. tale check è fatto da wgpu
        /*let image_dimensions = {
            use image::GenericImageView;
            diffuse_image.dimensions()
        };*/

        Self {
            diffuse_image: image::io::Reader::open(filepath).unwrap().decode().unwrap(),  // TODO: controllare l'esistenza del file
            // TODO: cambiare con as_image_copy nella versione 10 e provare a spostarla in Device
            texture: wgpu::ImageCopyTexture {
                texture: &device.diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data_layout: DATA_LAYOUT,  // TODO: potrebbe non essere il caso di salvarselo nella struct
        }
    }

    pub fn get_data(&self) -> &[u8] {
        use image::EncodableLayout;
        self.diffuse_image.as_rgba8().unwrap().as_bytes()
    }
}
