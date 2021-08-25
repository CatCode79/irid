//= CONSTS =========================================================================================

const DATA_LAYOUT: wgpu::ImageDataLayout = wgpu::ImageDataLayout {
    offset: 0,
    bytes_per_row: std::num::NonZeroU32::new(4 * crate::renderer::DEFAULT_TEXTURE_WIDTH),
    rows_per_image: std::num::NonZeroU32::new(crate::renderer::DEFAULT_TEXTURE_HEIGHT),
};


//= Texture ========================================================================================

pub struct Texture<'a> {
    diffuse_image: image::DynamicImage,
    //pub data: &'a [u8],
    pub texture: wgpu::ImageCopyTexture<'a>,
    pub data_layout: wgpu::ImageDataLayout,
//    pub view: wgpu::TextureView,
}


// TODO: per via di device questa funzione è meglio spostarla in irid::Device o quello che sarà
impl<'a> Texture<'a> {
    pub fn new(device: &'a crate::renderer::Device, filepath: &str) -> Self{
        let diffuse_image = image::io::Reader::open(filepath).unwrap().decode().unwrap();

        // TODO: potrebbe servirmi ancora per controllare che la diffuse_image sia effettivamente
        //  grande come le struct di default create da Device, prob. tale check è fatto da wgpu
        /*let image_dimensions = {
            use image::GenericImageView;
            diffuse_image.dimensions()
        };*/

        //let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            diffuse_image,
            texture: wgpu::ImageCopyTexture {
                texture: &device.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            data_layout: DATA_LAYOUT,
//            view,
        }
    }

    pub fn get_data(&self) -> &[u8]{
        use image::EncodableLayout;
        self.diffuse_image.as_rgba8().unwrap().as_bytes()
    }
}
