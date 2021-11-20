/// Refer to the documentation of the individual signatures in a generic way
/// because the individual implementations may vary in detail.

//= IMAGE ==========================================================================================

/// Trait that describes the generic behavior of an image object.
///
/// # Known Implementations:
///
/// - [irid-assets::DiffuseImage](irid-assets::DiffuseImage)
pub trait GenericImage {
    /// **Associated type** regarding the implementation of this trait.
    type Img;

    /// **Associated type** regarding the implementation of the [ImageSize] trait.
    type ImgSz;

    /// Open and decode a file to read, format will be guessed from path.
    fn load(filepath: &std::path::Path) -> image::ImageResult<Self::Img>;  // TODO utilise anyhow instead, also below

    /// Open and decode a file to read, format will be guessed from content.
    fn load_with_guessed_format(filepath: &std::path::Path) -> image::ImageResult<Self::Img>;

    /// Returns a value that implements the [ImageSize](ImageSize) trait.
    fn size(&self) -> Self::ImgSz;

    /// Get the bytes from the image as 8bit RGBA.
    fn as_rgba8_bytes(&self) -> Option<&[u8]>;
}

//= IMAGE SIZE =====================================================================================

/// Trait that describes the generic behavior of an image size info object.
///
/// # Known Implementations:
///
/// - [irid-assets::DiffuseImageSize](irid-assets::DiffuseImageSize)
pub trait GenericSize: From<(u32, u32)> + From<[u32; 2]> {
    ///
    fn new(width: u32, height: u32) -> Self;

    /// Returns the [Image] width.
    fn width(&self) -> u32;

    /// Returns the [Image] height.
    fn height(&self) -> u32;

    /// Returns the [Image] width and height (in that order) as tuple.
    fn as_tuple(&self) -> (u32, u32);
}

//= MODEL ==========================================================================================

///
pub trait GenericModel {
    type Mdl;

    fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self::Mdl>;
}

//= TEXTURE ========================================================================================

///
pub trait GenericTexture {
    type Txtr;
    type ImgSz;

    ///
    fn load(filepath: &std::path::Path) -> anyhow::Result<Self::Txtr>;

    ///
    fn load_with_guessed_format(filepath: &std::path::Path) -> anyhow::Result<Self::Txtr>;

    ///
    fn as_bytes(&self) -> Option<&[u8]>;

    ///
    fn size(&self) -> Self::ImgSz;
}

//= VERTEX =========================================================================================

///
pub trait GenericVertex {
    ///
    fn new() -> Self;

    ///
    fn position(&mut self, position: [f32; 3]);

    ///
    fn colors(&mut self, colors: [f32; 3]);

    ///
    fn tex_coords(&mut self, tex_coords: [f32; 2]);

    ///
    fn normal(&mut self, normal: [f32; 3]);
}

// TODO da togliere da qui e mettere in renderer
/*            let vertex_buffer = device.create_vertex_buffer_init(
                &format!("{:?} Vertex Buffer", path.as_ref()),
                vertices.as_slice(),
            );

            let index_buffer = device.create_indices_buffer_init(
                &format!("{:?} Index Buffer", path.as_ref()),
                obj_model.mesh.indices.as_slice(),
            );*/
