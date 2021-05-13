
//= CONSTS =========================================================================================

pub const PREFERRED_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


//= FNS ============================================================================================

pub fn create_texture_desc(size: wgpu::Extent3d, label_text: &str) -> wgpu::TextureDescriptor {
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

pub fn create_texture_view_desc(label_text: &str) -> wgpu::TextureViewDescriptor {
    wgpu::TextureViewDescriptor {
        label: Some(label_text),
        ..Default::default()
    }
}
