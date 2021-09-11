
//= DYNAMIC IMAGE ==================================================================================

pub struct DynamicImage(image::DynamicImage);

// TODO: cercare di fare a meno degli unwrap
impl DynamicImage {
    pub fn new(filepath: &str) -> Self {
        // TODO: potrebbe servirmi ancora per controllare che la diffuse_image sia effettivamente
        //  grande come le struct di default create in TextureMEtaDatas, comunque probabilmente
        //  tale check viene fatto da wgpu
        /*let image_dimensions = {
            use image::GenericImageView;
            diffuse_image.dimensions()
        };*/

        Self {
            0: image::io::Reader::open(filepath).unwrap().decode().unwrap(),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        use image::EncodableLayout;
        self.0.as_rgba8().unwrap().as_bytes()
    }
}
