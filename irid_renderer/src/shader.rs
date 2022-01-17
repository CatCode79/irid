//= USES ===========================================================================================

use crate::device::Device;

//= CONSTS =========================================================================================

/// This is the default vertex state entry point name that will be used in which case
/// one will not be passed.
pub const DEFAULT_VERTEX_ENTRY_POINT: &str = "vs_main";

/// This is the default fragment state entry point name that will be used in which case
/// one will not be passed.
pub const DEFAULT_FRAGMENT_ENTRY_POINT: &str = "fs_main";

//= SHADER MODULE BUILDER ==========================================================================

/// [ShaderModule](wgpu::ShaderModule)'s Builder.
//#[derive(Debug)] TODO: cannot derive it because of wgpu::ShaderSource
pub struct ShaderModuleBuilder<'a> {
    label: Option<&'a str>,
    source: wgpu::ShaderSource<'a>,
}

impl<'a> ShaderModuleBuilder<'a> {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a new ShaderModuleBuilder.
    pub fn new(source: wgpu::ShaderSource<'a>) -> Self {
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
    pub fn with_source(&mut self, source: wgpu::ShaderSource<'a>) -> &mut Self {
        self.source = source;
        self
    }

    //- Build --------------------------------------------------------------------------------------

    /// Build the shader module.
    pub fn build(self, device: &Device) -> wgpu::ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: self.label,
            source: self.source,
        })
    }
}
