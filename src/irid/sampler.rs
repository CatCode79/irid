
//= FNS ============================================================================================

pub fn create_sampler_desc(label_text: &str) -> wgpu::SamplerDescriptor {
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
