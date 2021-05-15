//= USES ===========================================================================================

use anyhow::*;


//= CONSTS =========================================================================================

pub const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= STRUCTS ========================================================================================

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

//= IMPLS ==========================================================================================

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label_text: &str
    ) -> Result<Self> {
        let img = image::load_from_memory_with_format(
            bytes,
            image::ImageFormat::Png
        )?;
        Self::from_image(device, queue, &img, label_text)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label_text: &str
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap();
        let dimensions = {
            use image::GenericImageView;
            img.dimensions()
        };

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture by setting depth to 1
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &create_texture_desc(label_text, size)
        );

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(
            &create_texture_view_desc(label_text)
        );

        let sampler = device.create_sampler(
            &create_sampler_desc(label_text)
        );

        Ok(Self { texture, view, sampler })
    }
}


//= FNS ============================================================================================

fn create_texture_desc(label_text: &str, size: wgpu::Extent3d) -> wgpu::TextureDescriptor {
    wgpu::TextureDescriptor {
        label: Some(label_text),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: PREFERRED_TEXTURE_FORMAT,
        // SAMPLED tells wgpu that we want to use this texture in shaders
        // COPY_DST means that we want to copy data to this texture
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    }
}

fn create_texture_view_desc(label_text: &str) -> wgpu::TextureViewDescriptor {
    wgpu::TextureViewDescriptor {
        label: Some(label_text),
        ..Default::default()
    }
}

fn create_sampler_desc(label_text: &str) -> wgpu::SamplerDescriptor {
    wgpu::SamplerDescriptor {
        label: Some(label_text),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    }
}
