//= USES ===========================================================================================

//use anyhow::*;


//= CONSTS =========================================================================================

pub const DEFAULT_TEXTURE_WIDTH: u32 = 256;
pub const DEFAULT_TEXTURE_HEIGHT: u32 = 256;


//= TEXTURE META DATAS =============================================================================

pub struct TextureMetaDatasBuilder<'a> {
    image_copy: Option<wgpu::ImageCopyTexture<'a>>,
    image_data_layout: Option<wgpu::ImageDataLayout>,
    image_size: Option<wgpu::Extent3d>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    bind_group: Option<wgpu::BindGroup>,
}
/*
impl<'a> TextureMetaDatasBuilder<'a> {

}*/


/// Struct containing values used by queue.write_texture()
pub struct TextureMetaDatas<'a> {
    //texture: wgpu::Texture,
    pub image_copy: wgpu::ImageCopyTexture<'a>,
    pub image_data_layout: wgpu::ImageDataLayout,
    pub image_size: wgpu::Extent3d,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl<'a> TextureMetaDatas<'a> {
    pub fn new(
        device: &crate::renderer::Device,
        width: u32,
        height: u32
    ) -> Self {
        let wgpu_device = device.expose_wgpu_device();

        let image_size = wgpu::Extent3d {
            width,
            height,
            // All textures are stored as 3D, we represent our 2D texture by setting depth to 1
            depth_or_array_layers: 1,
        };

        let texture = wgpu_device.create_texture(
            &wgpu::TextureDescriptor {
                size: image_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("Diffuse Texture"),
            }
        );

        let image_copy = wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };

        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * width),
            rows_per_image: std::num::NonZeroU32::new(height),
        };

        let bind_group_layout = TextureMetaDatas::create_bind_group_layout(wgpu_device);

        let bind_group = {
            let diffuse_texture_view = image_copy.texture.create_view(
                &wgpu::TextureViewDescriptor {
                    label: Some("Diffuse Texture View"),
                    format: None,
                    dimension: None,
                    aspect: wgpu::TextureAspect::All,
                    base_mip_level: 0,
                    mip_level_count: None,
                    base_array_layer: 0,
                    array_layer_count: None
                }
            );

            let diffuse_sampler = TextureMetaDatas::create_sampler(wgpu_device);

            wgpu_device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                        }
                    ],
                    label: Some("Diffuse Bind Group"),
                }
            )
        };

        Self {
            //texture,
            image_copy,
            image_data_layout,
            image_size,
            bind_group_layout,
            bind_group,
        }
    }

    #[inline]
    pub fn new_default_size(device: &crate::renderer::Device) -> Self {
        Self::new(device, DEFAULT_TEXTURE_WIDTH, DEFAULT_TEXTURE_HEIGHT)
    }

    fn create_bind_group_layout(
        wgpu_device: &std::rc::Rc<wgpu::Device>
    ) -> wgpu::BindGroupLayout {
        wgpu_device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            // This is only for TextureSampleType::Depth
                            comparison: false,
                            // This should be true if the sample_type of the texture is:
                            //     TextureSampleType::Float { filterable: true }
                            // Otherwise you'll get an error.
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("Texture Bind Group Layout"),
            }
        )
    }

    fn create_sampler(wgpu_device: &std::rc::Rc<wgpu::Device>) -> wgpu::Sampler {
        wgpu_device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,  // TODO: probabilmente meglio utilizzare MirrorRepeated per evitare le Bleeding Textures, anche sotto
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: 0.0,
                lod_max_clamp: 0.0,
                compare: None,
                anisotropy_clamp: None,
                border_color: None
            }
        )
    }
}


//= TEXTURE HANDLER ================================================================================

pub struct Texture {
    diffuse_image: image::DynamicImage,
}

impl Texture {
    pub fn new(filepath: &str) -> Self {
        // TODO: potrebbe servirmi ancora per controllare che la diffuse_image sia effettivamente
        //  grande come le struct di default create in TextureMEtaDatas, comunque probabilmente
        //  tale check viene fatto da wgpu
        /*let image_dimensions = {
            use image::GenericImageView;
            diffuse_image.dimensions()
        };*/

        // TODO: controllare l'esistenza del file senza questi beceri unwrap?
        Self {
            diffuse_image: image::io::Reader::open(filepath).unwrap().decode().unwrap(),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        use image::EncodableLayout;
        self.diffuse_image.as_rgba8().unwrap().as_bytes()
    }
}
