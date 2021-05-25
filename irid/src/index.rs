
//= FNS ============================================================================================

/**
 *
 */
pub fn create_buffer_init(
    renderer: &crate::renderer::Renderer,
    label_text: &str,
    indices: &[u16]
) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    renderer.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some(label_text),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        }
    )
}
