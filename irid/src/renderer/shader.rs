//= USES ===========================================================================================

use crate::renderer::Surface;


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


//= VERTEX STATE BUILDER ===========================================================================

///
#[derive(Clone, Debug)]
pub struct VertexStateBuilder<'a> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    buffers: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
}


impl<'a> VertexStateBuilder<'a> {
    ///
    pub const DEFAULT_ENTRY_POINT: &'static str = "vertex_main";

    //- Constructor Methods ------------------------------------------------------------------------

    ///
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        VertexStateBuilder {
            module,
            entry_point: None,
            buffers: None,
        }
    }

    //- With-Setter Methods ------------------------------------------------------------------------

    ///
    pub fn module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.module = module;
        self
    }

    ///
    pub fn entry_point(&mut self, entry_point: &'a str) -> &mut Self {
        self.entry_point = if entry_point.is_empty() {
            log::warn!("An empty entry_point string was passed as argument for VertexStateBuilder, \
            the default value of {} will be set instead",
                VertexStateBuilder::DEFAULT_ENTRY_POINT);
            Some(VertexStateBuilder::DEFAULT_ENTRY_POINT)
        } else {
            Some(entry_point)
        };
        self
    }

    ///
    pub fn buffers(mut self, buffers: &'a [wgpu::VertexBufferLayout]) -> Self {
        self.buffers = Some(buffers);
        self
    }

    //- Build Methods ------------------------------------------------------------------------------

    ///
    pub fn build(self) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module: self.module,

            entry_point: if self.entry_point.is_some() {
                self.entry_point.unwrap()
            } else {
                VertexStateBuilder::DEFAULT_ENTRY_POINT
            },

            buffers: if self.buffers.is_some() {
                self.buffers.unwrap()
            } else {
                &[]
            },
        }
    }
}


//= FRAGMENT STATE BUILDER =========================================================================

///
#[derive(Clone, Debug)]
pub struct FragmentStateBuilder<'a> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    targets: Option<&'a [wgpu::ColorTargetState]>,
    color_target_states: Option<[wgpu::ColorTargetState; 1]>,  // workaround to avoid error[E0716]
}


impl<'a> FragmentStateBuilder<'a> {
    ///
    pub const DEFAULT_ENTRY_POINT: &'static str = "fragment_main";

    //- Constructor Methods ------------------------------------------------------------------------

    ///
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            module,
            entry_point: None,
            targets: None,
            color_target_states: None
        }
    }

    //- With-Setter Methods ------------------------------------------------------------------------

    ///
    pub fn module(mut self, module: &'a wgpu::ShaderModule) -> Self {
        self.module = module;
        self
    }

    ///
    pub fn entry_point(mut self, entry_point: &'a str) -> Self {
        self.entry_point = if entry_point.is_empty() {
            log::warn!("An empty entry_point string was passed as argument for FragmentStateBuilder, \
            the default value of {} will be set instead",
                FragmentStateBuilder::DEFAULT_ENTRY_POINT);
            Some(FragmentStateBuilder::DEFAULT_ENTRY_POINT)
        } else {
            Some(entry_point)
        };
        self
    }

    ///
    pub fn targets(mut self, targets: &'a [wgpu::ColorTargetState]) -> Self {
        self.targets = Some(targets);
        self
    }

    //- Build Methods ------------------------------------------------------------------------------

    ///
    fn create_default_color_target_state(
        &self,
        format: wgpu::TextureFormat
    ) -> wgpu::ColorTargetState {
        wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        }
    }

    ///
    pub fn build(mut self, surface: &'a Surface) -> wgpu::FragmentState<'a> {
        if self.targets.is_none() {
            self.color_target_states = Some([self.create_default_color_target_state(
                surface.get_preferred_format())]
            );
        };
        self.inner_build()
    }

    fn inner_build(self) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module: self.module,

            entry_point: if self.entry_point.is_some() {
                self.entry_point.unwrap()
            } else {
                FragmentStateBuilder::DEFAULT_ENTRY_POINT
            },

            targets: if self.targets.is_some() {
                self.targets.unwrap()
            } else {
                &self.color_target_states.unwrap()
            },
        }
    }
}
