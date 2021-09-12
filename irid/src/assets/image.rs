
//= DYNAMIC IMAGE ==================================================================================

//use image::{ImageBuffer, Rgba};

pub struct DynamicImage(image::DynamicImage);

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
            0: image::io::Reader::open(filepath).unwrap().decode().unwrap(),  // TODO: cercare di fare a meno degli unwrap
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        use image::EncodableLayout;
        match self.0.as_rgba8() {
            None => { None }
            Some(rgba8) => { Some(rgba8.as_bytes()) }
        }
    }

    pub fn dimensions(&self) -> Option<(u32, u32)> {
        match self.0.as_rgba8() {
            None => { None }
            Some(rgba8) => { Some(rgba8.dimensions()) }
        }
    }
}
