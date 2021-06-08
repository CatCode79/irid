
//= CONSTS =========================================================================================

// TODO farlo come funzione per ricavare, anche solo per debug, che cosa è preferito dal device
pub(crate) const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= STRUCTS ========================================================================================

pub struct Texture {
    pub handle: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

//= IMPLS ==========================================================================================

impl Texture {
    // TODO pub fn from_path

    pub fn from_bytes(
        renderer: &crate::renderer::Renderer,
        bytes: &[u8],
        label_text: &str
    ) -> anyhow::Result<Self> {
        let img = image::load_from_memory_with_format(
            bytes,
            image::ImageFormat::Png  // TODO Configurabile o passabile come argomento
        )?;  // TODO anche qui controllare il funzionamento di anyhow
        Self::from_image(renderer, &img, label_text)
    }


    pub fn from_image(
        renderer: &crate::renderer::Renderer,
        img: &image::DynamicImage,
        label_text: &str
    ) -> anyhow::Result<Self> {
        let rgba = img.as_rgba8().unwrap();  // TODO panic? in realtà penso ci pensi anyhow

        let dimensions = {
            use image::GenericImageView;
            img.dimensions()
        };

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture by setting depth to 1
            depth_or_array_layers: 1,
        };

        let handle = renderer.device.create_texture(
            &create_handle_desc(label_text, texture_size)
        );

        renderer.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTextureBase {
                texture: &handle,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );

        let view = handle.create_view(
            &create_view_desc(label_text)
        );

        let sampler = renderer.device.create_sampler(
            &create_sampler_desc(label_text)
        );

        Ok(Self { handle, view, sampler })
    }


    /**
     * Convenience method to create a BindGroup for textures.
     */
    pub fn create_bind_group(
        &self,
        renderer: &crate::renderer::Renderer,
        label_text: &str,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        renderer.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some(label_text),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&self.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    }
                ],
            }
        )
    }
}


//= FNS ============================================================================================

//- Texture Handle Creation Functions --------------------------------------------------------------

fn create_handle_desc(label_text: &str, size: wgpu::Extent3d) -> wgpu::TextureDescriptor {
    wgpu::TextureDescriptor {
        label: Some(label_text),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: PREFERRED_TEXTURE_FORMAT,
        // SAMPLED tells wgpu that we want to use this texture in assets.shaders
        // COPY_DST means that we want to copy data to this texture
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    }
}


//- Texture View Creation Functions ----------------------------------------------------------------

fn create_view_desc(label_text: &str) -> wgpu::TextureViewDescriptor {
    wgpu::TextureViewDescriptor {
        label: Some(label_text),
        ..Default::default()
    }
}


//- Texture Sampler Creation Functions -------------------------------------------------------------

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


//- Bind Group Creation Functions ------------------------------------------------------------------

/**
 * Convenience method to create a typical BindGroupLayout for textures.
 */
#[inline(always)]
pub fn create_bind_group_layout(
    renderer: &crate::renderer::Renderer,
    label_text: &str
)-> wgpu::BindGroupLayout {
    renderer.device.create_bind_group_layout(
        &create_bind_group_layout_desc(label_text)
    )
}


fn create_bind_group_layout_desc(label_text: &str) -> wgpu::BindGroupLayoutDescriptor {
    wgpu::BindGroupLayoutDescriptor {
        label: Some(label_text),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                count: None,
            },
        ],
    }
}
