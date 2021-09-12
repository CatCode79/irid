
//= DEFAULT SIZE CONSTS ============================================================================

pub const DEFAULT_TEXTURE_WIDTH: u32 = 256;
pub const DEFAULT_TEXTURE_HEIGHT: u32 = 256;


//= TEXTURE METADATAS ==============================================================================

pub struct TextureMetadatas {
    image_metadatas: TextureImageMetadatas,
    bind_group_metadatas: TextureBindGroupMetadatas,
}

impl TextureMetadatas {
    pub fn new(
        device: &crate::renderer::Device, width: u32, height: u32
    ) -> Self {
        let image_metadatas = TextureImageMetadatas::new(
            device, width, height
        );

        let bind_group_metadatas = TextureBindGroupMetadatas::new(
            device, &image_metadatas
        );

        Self {
            //texture,
            image_metadatas,
            bind_group_metadatas,
        }
    }

    #[inline]
    pub fn new_default_size(device: &crate::renderer::Device) -> Self {
        Self::new(device, DEFAULT_TEXTURE_WIDTH, DEFAULT_TEXTURE_HEIGHT)
    }

    //- Getters ------------------------------------------------------------------------------------

    pub fn new_image_copy(&self) -> wgpu::ImageCopyTexture {
        wgpu::ImageCopyTexture {
            texture: &self.image_metadatas.texture,  // TODO: odio questa &, creare una wgpu personale senza?
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }
    }

    #[inline]
    pub fn clone_image_data_layout(&self) -> wgpu::ImageDataLayout {
        self.image_metadatas.image_data_layout.clone()
    }

    #[inline]
    pub fn clone_image_size(&self) -> wgpu::Extent3d {
        self.image_metadatas.image_size.clone()
    }

    #[inline]
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_metadatas.bind_group_layout
    }

    #[inline]
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group_metadatas.bind_group
    }
}


//= TEXTURE IMAGE METADATAS ========================================================================

/// Struct containing values used by queue.write_texture()
struct TextureImageMetadatas {
    texture: wgpu::Texture,
    image_data_layout: wgpu::ImageDataLayout,
    image_size: wgpu::Extent3d,
}

impl TextureImageMetadatas {
    pub fn new(
        device: &crate::renderer::Device,
        width: u32,
        height: u32,
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

        let image_data_layout = wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * width),
            rows_per_image: std::num::NonZeroU32::new(height),
        };

        Self {
            texture,
            image_data_layout,
            image_size,
        }
    }
}


//= TEXTURE BIND GROUP METADATAS ===================================================================

struct TextureBindGroupMetadatas {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl TextureBindGroupMetadatas {
    pub fn new(
        device: &crate::renderer::Device,
        image_metadatas: &TextureImageMetadatas
    ) -> Self {
        let wgpu_device = device.expose_wgpu_device();

        let bind_group_layout = TextureBindGroupMetadatas::create_bind_group_layout(
            wgpu_device
        );

        let bind_group = wgpu_device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(
                            &TextureBindGroupMetadatas::create_texture_view(image_metadatas)
                        ),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(
                            &TextureBindGroupMetadatas::create_sampler(wgpu_device)
                        ),
                    }
                ],
                label: Some("Diffuse Bind Group"),
            }
        );

        Self {
            bind_group_layout,
            bind_group,
        }
    }

    fn create_bind_group_layout(
        wgpu_device: &wgpu::Device
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

    fn create_texture_view(image_metadatas: &TextureImageMetadatas) -> wgpu::TextureView {
        image_metadatas.texture.create_view(
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
        )
    }

    fn create_sampler(wgpu_device: &wgpu::Device) -> wgpu::Sampler {
        wgpu_device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Diffuse Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,  // TODO: probabilmente meglio utilizzare MirrorRepeated per evitare le Bleeding Textures, idem sotto
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
