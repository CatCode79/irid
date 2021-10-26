//= USES ===========================================================================================

use crate::renderer::{Device, Surface};


//= SHADER MODULE ==================================================================================

/// [ShaderModule](wgpu::ShaderModule)'s Builder.
pub struct ShaderModuleBuilder<'a> {
    label: Option<&'a str>,
    source: wgpu::ShaderSource<'static>,  //TODO avoid this static lifetime
}


impl<'a> ShaderModuleBuilder<'a> {

    //- Constructor Methods ------------------------------------------------------------------------

    /// Create a new ShaderModuleBuilder.
    pub fn new(source: wgpu::ShaderSource<'static>) -> Self {
        Self {
            label: Some("Render Pipeline Descriptor Default Label"),
            source,
        }
    }

    //- Setter Methods -----------------------------------------------------------------------------

    /// Debug label of the shader module. This will show up in graphics debuggers
    /// for easy identification.
    pub fn with_label(&mut self, label_text: &'a str) -> &mut Self {
        self.label = if label_text.is_empty() {
            wgpu::Label::default()
        } else {
            Some(label_text)
        };
        self
    }

    /// Source code for the shader.
    pub fn with_source(&mut self, source: wgpu::ShaderSource<'static>) -> &mut Self {
        self.source = source;
        self
    }

    //- Build Methods ------------------------------------------------------------------------------

    /// Build the shader module.
    pub fn build(self, device: &Device) -> wgpu::ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(self.label.unwrap()),  // TODO mancano dei check
            source: self.source,
        })
    }
}


//= VERTEX STATE BUILDER ===========================================================================

/// [VertexState](wgpu::VertexState)'s Builder.
#[derive(Clone, Debug)]
pub struct VertexStateBuilder<'a> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    buffers: Option<&'a [wgpu::VertexBufferLayout<'a>]>,
}


impl<'a> VertexStateBuilder<'a> {
    /// This is the default vertex state entry point name that will be used in which case
    /// one will not be passed.
    pub const DEFAULT_ENTRY_POINT: &'static str = "vs_main";

    //- Constructor Methods ------------------------------------------------------------------------

    /// Create a new VertexStateBuilder with a [ShaderModule](wgpu::ShaderModule).
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        VertexStateBuilder {
            module,
            entry_point: None,
            buffers: None,
        }
    }

    //- With-Setter Methods ------------------------------------------------------------------------

    /// The compiled shader module for this vertex stage.
    pub fn with_module(&mut self, module: &'a wgpu::ShaderModule) -> &mut Self {
        self.module = module;
        self
    }

    /// The name of the vertex entry point in the compiled shader.
    /// There must be a function that returns void with this name in the shader.
    pub fn with_entry_point(&mut self, entry_point: &'a str) -> &mut Self {
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

    /// The format of any vertex buffers used with this pipeline.
    pub fn with_buffers(mut self, buffers: &'a [wgpu::VertexBufferLayout]) -> Self {
        self.buffers = Some(buffers);
        self
    }

    //- Build Methods ------------------------------------------------------------------------------

    /// Build a new Vertex State.
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

/// [FragmentState](wgpu::FragmentState)'s Builder.
#[derive(Clone, Debug)]
pub struct FragmentStateBuilder<'a> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    targets: Option<&'a [wgpu::ColorTargetState]>,
}


impl<'a> FragmentStateBuilder<'a> {
    /// This is the default fragment state entry point name that will be used in which case
    /// one will not be passed.
    pub const DEFAULT_ENTRY_POINT: &'static str = "fs_main";

    //- Constructor Methods ------------------------------------------------------------------------

    /// Create a new FragmentStateBuilder with a [ShaderModule](wgpu::ShaderModule).
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            module,
            entry_point: None,
            targets: None,
        }
    }

    //- With-Setter Methods ------------------------------------------------------------------------

    /// The compiled shader module for this fragment stage.
    pub fn with_module(mut self, module: &'a wgpu::ShaderModule) -> Self {
        self.module = module;
        self
    }

    /// The name of the fragment entry point in the compiled shader.
    /// There must be a function that returns void with this name in the shader.
    pub fn with_entry_point(mut self, entry_point: &'a str) -> Self {
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

    /// The color state of the render targets.
    pub fn with_targets(mut self, targets: &'a [wgpu::ColorTargetState]) -> Self {
        self.targets = Some(targets);
        self
    }

    //- Build Methods ------------------------------------------------------------------------------

    /// Build a new Fragment State.
    pub fn build(self, surface: &'a Surface) -> wgpu::FragmentState<'a> {
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
                surface.color_target_states()
            },
        }
    }
}
