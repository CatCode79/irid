
//= SHADER MODULE ==================================================================================

pub(crate) struct ShaderModuleBuilder<'a> {
    shader_module_desc: wgpu::ShaderModuleDescriptor<'a>,
}


impl<'a> ShaderModuleBuilder<'a> {
    pub(crate) fn new(source: Box<wgpu::ShaderSource<'static>>) -> Self {
        #[cfg(feature = "debug_label")]
        let label = Some("Render Pipeline Descriptor Default Label");
        #[cfg(not(feature = "debug_label"))]
        let label = wgpu::Label::default();

        Self {
            shader_module_desc: wgpu::ShaderModuleDescriptor {
                label,
                source: *source,
                flags: wgpu::ShaderFlags::VALIDATION | wgpu::ShaderFlags::EXPERIMENTAL_TRANSLATION
            },
        }
    }

    pub(crate) fn label(&mut self, label_text: &'a str) -> &mut Self {
        self.shader_module_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    pub(crate) fn source(&mut self, source: wgpu::ShaderSource<'static>) -> &mut Self {
        self.shader_module_desc.source = source;
        self
    }

    pub(crate) fn flags(&mut self, flags: wgpu::ShaderFlags) -> &mut Self {
        self.shader_module_desc.flags = flags;
        self
    }

    pub(crate) fn expose_wrapped_desc(&self) -> &wgpu::ShaderModuleDescriptor {
        &self.shader_module_desc
    }

    pub(crate) fn build(self, device: &std::rc::Rc<wgpu::Device>) -> wgpu::ShaderModule {
        device.create_shader_module(&self.shader_module_desc)
    }
}


//= VERTEX STATE ===================================================================================

pub(crate) struct VertexStateBuilder<'a> {
    vertex_state: wgpu::VertexState<'a>
}


impl<'a> VertexStateBuilder<'a> {
    pub(crate) const DEFAULT_ENTRY_POINT: &'static str = "main";  // TODO: configurarlo in un build script

    pub(crate) fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            vertex_state: wgpu::VertexState {
                module,
                entry_point: VertexStateBuilder::DEFAULT_ENTRY_POINT,
                buffers: &[],
            },
        }
    }

    pub(crate) fn module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.vertex_state.module = module;
        self
    }

    pub(crate) fn entry_point(&mut self, entry_point: &'a str) -> &mut Self {
        self.vertex_state.entry_point = if entry_point.is_empty() {
            VertexStateBuilder::DEFAULT_ENTRY_POINT
        } else {
            entry_point
        };
        self
    }

    pub(crate) fn buffers(&mut self, buffers: &'a [wgpu::VertexBufferLayout]) -> &mut Self {
        self.vertex_state.buffers = buffers;
        self
    }

    pub(crate) fn build(self) -> wgpu::VertexState<'a> {
        self.vertex_state
    }
}


//= FRAGMENT STATE ============================================================================

pub(crate) struct FragmentStateBuilder<'a> {
    fragment_state: wgpu::FragmentState<'a>,
}


impl<'a> FragmentStateBuilder<'a> {
    pub(crate) const DEFAULT_ENTRY_POINT: &'static str = "main";  // TODO: configurarlo in un build script

    pub(crate) const DEFAULT_COLOR_TARGET_STATE: wgpu::ColorTargetState = wgpu::ColorTargetState {
        format: crate::texture::PREFERRED_TEXTURE_FORMAT,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrite::ALL,
    };

    pub(crate) fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            fragment_state: wgpu::FragmentState {
                module,
                entry_point: FragmentStateBuilder::DEFAULT_ENTRY_POINT,
                targets: &[FragmentStateBuilder::DEFAULT_COLOR_TARGET_STATE],
            },
        }
    }

    pub(crate) fn module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.fragment_state.module = module;
        self
    }

    pub(crate) fn entry_point(&mut self, entry_point: &'a str) -> &mut Self {
        self.fragment_state.entry_point = if entry_point.is_empty() {
            VertexStateBuilder::DEFAULT_ENTRY_POINT
        } else {
            entry_point
        };
        self
    }

    pub(crate) fn targets(&mut self, targets: &'a [wgpu::ColorTargetState]) -> &mut Self {
        self.fragment_state.targets = targets;
        self
    }

    pub(crate) fn build(self) -> wgpu::FragmentState<'a> {
        self.fragment_state
    }
}
