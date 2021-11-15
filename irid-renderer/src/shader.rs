//= USES ===========================================================================================

use irid_assets_traits::Image;
use irid_renderer_traits::Vertex;

use crate::{device::Device};

//= SHADER MODULE ==================================================================================

/// [ShaderModule](wgpu::ShaderModule)'s Builder.
pub struct ShaderModuleBuilder<'a> {
    label: Option<&'a str>,
    source: wgpu::ShaderSource<'static>,  // TODO: avoid this static lifetime
}

impl<'a> ShaderModuleBuilder<'a> {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new ShaderModuleBuilder.
    pub fn new(source: wgpu::ShaderSource<'static>) -> Self {
        Self {
            label: Some("Render Pipeline Descriptor Default Label"),
            source,
        }
    }

    //- Setters ------------------------------------------------------------------------------------

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

    //- Build --------------------------------------------------------------------------------------

    /// Build the shader module.
    pub fn build<I: Image, V: Vertex>(self, device: &Device<I, V>) -> wgpu::ShaderModule {
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
    //- Constants ----------------------------------------------------------------------------------

    /// This is the default vertex state entry point name that will be used in which case
    /// one will not be passed.
    pub const DEFAULT_ENTRY_POINT: &'static str = "vs_main";

    //- Constructors -------------------------------------------------------------------------------

    /// Create a new VertexStateBuilder with a [ShaderModule](wgpu::ShaderModule).
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        VertexStateBuilder {
            module,
            entry_point: None,
            buffers: None,
        }
    }

    //- Setters ------------------------------------------------------------------------------------

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

    //- Build --------------------------------------------------------------------------------------

    /// Build a new Vertex State.
    pub fn build(self) -> wgpu::VertexState<'a> {
        wgpu::VertexState {
            module: self.module,
            entry_point: self.entry_point.unwrap_or(VertexStateBuilder::DEFAULT_ENTRY_POINT),
            buffers: self.buffers.unwrap_or(&[]),
        }
    }
}

//= FRAGMENT STATE BUILDER =========================================================================

/// [FragmentState](wgpu::FragmentState)'s Builder.
#[derive(Clone, Debug)]
pub struct FragmentStateBuilder<'a, I: Image, V: Vertex> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    targets: Option<&'a [wgpu::ColorTargetState]>,
}

impl<'a, I, V> FragmentStateBuilder<'a, I, V> {
    //- Constants ----------------------------------------------------------------------------------

    /// This is the default fragment state entry point name that will be used in which case
    /// one will not be passed.
    pub const DEFAULT_ENTRY_POINT: &'static str = "fs_main";

    //- Constructors -------------------------------------------------------------------------------

    /// Create a new FragmentStateBuilder with a [ShaderModule](wgpu::ShaderModule).
    pub fn new(module: &'a wgpu::ShaderModule) -> Self {
        Self {
            module,
            entry_point: None,
            targets: None,
        }
    }

    //- Setters ------------------------------------------------------------------------------------

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

    //- Build --------------------------------------------------------------------------------------

    /// Build a new Fragment State.
    pub fn build(self, color_target_state: wgpu::ColorTargetState) -> wgpu::FragmentState<'a> {
        wgpu::FragmentState {
            module: self.module,
            entry_point: self.entry_point.unwrap_or(FragmentStateBuilder::DEFAULT_ENTRY_POINT),
            targets: self.targets.unwrap_or(&[color_target_state]),
        }
    }
}
