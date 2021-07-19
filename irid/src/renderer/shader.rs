
//= SHADER MODULE ==================================================================================

pub(crate) struct ShaderModuleBuilder<'a> {
    wrapped_shader_module_desc: wgpu::ShaderModuleDescriptor<'a>,
}


impl<'a> ShaderModuleBuilder<'a> {
    pub(crate) fn new(label_text: &str) -> &mut Self {
        &mut Self {
            wrapped_shader_module_desc: wgpu::ShaderModuleDescriptor {
                label: Option::None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from("")),
                flags: wgpu::ShaderFlags::VALIDATION | wgpu::ShaderFlags::EXPERIMENTAL_TRANSLATION
            },
        }.label(label_text)
    }

    pub(crate) fn label(&mut self, label_text: &str) -> &mut Self {
        self.wrapped_shader_module_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    pub(crate) fn source_as_wgsl(&mut self, source_text: &str) -> &mut Self {
        self.wrapped_shader_module_desc.source = wgpu::ShaderSource::Wgsl(
            std::borrow::Cow::from(source_text)
        );
        self
    }

    pub(crate) fn source_as_spirv(&mut self, source_text: &[u32]) -> &mut Self {
        self.wrapped_shader_module_desc.source = wgpu::ShaderSource::SpirV(
            std::borrow::Cow::from(source_text)
        );
        self
    }

    pub(crate) fn flags(&mut self, values: wgpu::ShaderFlags) -> &mut Self {
        self.wrapped_shader_module_desc.flags = values;
        self
    }

    pub(crate) fn build(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(&self.wrapped_shader_module_desc)
    }
}
