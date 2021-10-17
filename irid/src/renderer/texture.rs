//= USES ===========================================================================================

use crate::assets::DiffuseImage;
use crate::renderer::{Device, Surface};


//= DIFFUSE TEXTURE ================================================================================

///
#[derive(Debug)]
pub struct DiffuseTexture {
    diffuse_image: DiffuseImage,
    image_metadatas: TextureImageMetadatas,
}

impl DiffuseTexture {
    ///
    // TODO I have to create the metas in static manner and after the surface/device creation, so I can create a texture without use those parameters
    pub fn load(surface: &Surface, device: &crate::renderer::Device, filepath: &std::path::Path) -> anyhow::Result<Self> {
        let diffuse_image = DiffuseImage::new(filepath)?;

        // TODO: da finire
        let image_metadatas = TextureImageMetadatas::new(
            surface,
            device,
            diffuse_image.width(),
            diffuse_image.height()
        );

        Ok(Self {
            diffuse_image,
            image_metadatas,
        })
    }

    // TODO: to be used instead of  dynamic_image.as_rgba8_bytes on queue.create_texture after created the IridQueue
    pub fn as_bytes(&self) -> Option<&[u8]>{
        self.diffuse_image.as_rgba8_bytes()
    }
}


//= TEXTURE IMAGE METADATAS ========================================================================

/// Struct containing values used by queue.write_texture()
#[derive(Debug)]
pub struct TextureImageMetadatas {
    texture: wgpu::Texture,
    image_data_layout: wgpu::ImageDataLayout,
    image_size: wgpu::Extent3d,
}

impl TextureImageMetadatas {
    pub fn new(
        surface: &Surface,
        device: &Device,
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
                format: surface.get_preferred_format(),
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


    //- ImageCopyTexture related metghods ----------------------------------------------------------

    pub fn create_image_copy(&self) -> wgpu::ImageCopyTexture {
        wgpu::ImageCopyTexture {
            texture: &self.texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        }
    }

    //- Getters ------------------------------------------------------------------------------------

    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn image_data_layout(&self) -> &wgpu::ImageDataLayout {
        &self.image_data_layout
    }

    pub fn image_size(&self) -> &wgpu::Extent3d {
        &self.image_size
    }
}


//= TEXTURE BIND GROUP METADATAS ===================================================================

///
#[derive(Debug)]
pub struct TextureBindGroupMetadatas {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl TextureBindGroupMetadatas {
    pub fn new(
        device: &crate::renderer::Device,
        texture: &wgpu::Texture
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
                            &TextureBindGroupMetadatas::create_texture_view(texture)
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

    fn create_texture_view(texture: &wgpu::Texture) -> wgpu::TextureView {
        texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: Some("Diffuse Texture View"),
                ..Default::default()
            }
        )
    }

    fn create_sampler(wgpu_device: &wgpu::Device) -> wgpu::Sampler {
        wgpu_device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Diffuse Texture Sampler"),
                address_mode_u: wgpu::AddressMode::ClampToEdge,  // TODO: probably is better to use MirrorRepeated to avoid bleeding textures, also below
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        )
    }

    //- Getters ------------------------------------------------------------------------------------

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}


//= TEXTURE DEPTH ==================================================================================

///
#[derive(Debug)]
pub struct TextureDepthMetadatas {
    _texture: wgpu::Texture,
    view: wgpu::TextureView,
    _sampler: wgpu::Sampler,
}


impl TextureDepthMetadatas {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Our depth texture needs to be the same size as our screen if we want things
    /// to render correctly so we give to constructor windows_size value.
    pub fn new(
        device: &crate::renderer::Device,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        let wgpu_device = device.expose_wgpu_device();

        let size = wgpu::Extent3d {
            width: window_size.width,
            height: window_size.height,
            depth_or_array_layers: 1,
        };

        let desc = wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };

        let texture = wgpu_device.create_texture(&desc);

        let view = texture.create_view(
            &wgpu::TextureViewDescriptor{
                label: Some("Depth Texture View"),
                ..Default::default()
            }
        );

        // We technically don't need a sampler for a depth texture,
        // but our Texture struct requires it.
        // If we do decide to render our depth texture, we need to use CompareFunction::LessEqual.
        // This is due to how the samplerShadow and sampler2DShadow()
        // interacts with the texture() function in GLSL.
        let sampler = wgpu_device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self {
            _texture: texture,
            view,
            _sampler: sampler,
        }
    }

    //- Getters ------------------------------------------------------------------------------------

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}
