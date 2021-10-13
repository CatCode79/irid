
//= SHADER MODULE ==================================================================================

pub struct ShaderModuleBuilder<'a> {
    shader_module_desc: wgpu::ShaderModuleDescriptor<'a>,
}


impl<'a> ShaderModuleBuilder<'a> {
    pub fn new(source: wgpu::ShaderSource<'static>) -> Self {
        Self {
            shader_module_desc: wgpu::ShaderModuleDescriptor {
                label: Some("Render Pipeline Descriptor Default Label"),
                source,
            },
        }
    }

    pub fn label(&mut self, label_text: &'a str) -> &mut Self {
        self.shader_module_desc.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    pub fn source(&mut self, source: wgpu::ShaderSource<'static>) -> &mut Self {
        self.shader_module_desc.source = source;
        self
    }

    pub fn expose_wrapped_desc(&self) -> &wgpu::ShaderModuleDescriptor {
        &self.shader_module_desc
    }

    pub fn build(self, device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(&self.shader_module_desc)
    }
}


//= VERTEX STATE ===================================================================================

#[derive(Clone, Debug)]
pub struct VertexStateBuilder<'a> {
    vertex_state: wgpu::VertexState<'a>
}


impl<'a> VertexStateBuilder<'a> {
    pub const DEFAULT_ENTRY_POINT: &'static str = "main";  // TODO: configurarlo in un build script

    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            vertex_state: wgpu::VertexState {
                module,
                entry_point: VertexStateBuilder::DEFAULT_ENTRY_POINT,
                buffers: &[],
            },
        }
    }

    pub fn module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.vertex_state.module = module;
        self
    }

    pub fn entry_point(&mut self, entry_point: &'a str) -> &mut Self {
        self.vertex_state.entry_point = if entry_point.is_empty() {
            VertexStateBuilder::DEFAULT_ENTRY_POINT
        } else {
            entry_point
        };
        self
    }

    pub fn buffers(mut self, buffers: &'a [wgpu::VertexBufferLayout]) -> Self {
        self.vertex_state.buffers = buffers;
        self
    }

    pub fn build(self) -> wgpu::VertexState<'a> {
        self.vertex_state
    }
}


//= FRAGMENT STATE ============================================================================

pub struct FragmentStateBuilder<'a> {
    fragment_state: wgpu::FragmentState<'a>,
}


impl<'a> FragmentStateBuilder<'a> {
    pub const DEFAULT_ENTRY_POINT: &'static str = "main";  // TODO: configurarlo in un build script

    pub const DEFAULT_COLOR_TARGET_STATE: wgpu::ColorTargetState = wgpu::ColorTargetState {
        format: crate::renderer::PREFERRED_TEXTURE_FORMAT,
        blend: Some(wgpu::BlendState {
            color: wgpu::BlendComponent::REPLACE,
            alpha: wgpu::BlendComponent::REPLACE,
        }),
        write_mask: wgpu::ColorWrites::ALL,
    };

    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            fragment_state: wgpu::FragmentState {
                module,
                entry_point: FragmentStateBuilder::DEFAULT_ENTRY_POINT,
                targets: &[FragmentStateBuilder::DEFAULT_COLOR_TARGET_STATE],
            },
        }
    }

    pub fn module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.fragment_state.module = module;
        self
    }

    pub fn entry_point(&mut self, entry_point: &'a str) -> &mut Self {
        self.fragment_state.entry_point = if entry_point.is_empty() {
            VertexStateBuilder::DEFAULT_ENTRY_POINT
        } else {
            entry_point
        };
        self
    }

    pub fn targets(&mut self, targets: &'a [wgpu::ColorTargetState]) -> &mut Self {
        self.fragment_state.targets = targets;
        self
    }

    pub fn build(self) -> wgpu::FragmentState<'a> {
        self.fragment_state
    }
}
