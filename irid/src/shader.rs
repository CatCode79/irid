
//= FNS ============================================================================================

/*
/**
 * SHADER'S COMPILING AT RUNTIME
 */
fn _create_shader_module() {
    let _vs_src = include_str!("assets.shaders/shader.vert");
    let _fs_src = include_str!("assets.shaders/shader.frag");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();
    let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();
    let vs_data = wgpu::util::make_spirv(vs_spirv.as_binary_u8());
    let fs_data = wgpu::util::make_spirv(fs_spirv.as_binary_u8());
    let vs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: vs_data,
        flags: wgpu::ShaderFlags::default(),
    });
    let fs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: fs_data,
        flags: wgpu::ShaderFlags::default(),
    });
}
*/


/**
 *
 */
#[inline(always)]
pub fn create_module(
    renderer: &crate::renderer::Renderer,
    descriptor: &wgpu::ShaderModuleDescriptor
) -> wgpu::ShaderModule {
    renderer.device.create_shader_module(descriptor)
}
