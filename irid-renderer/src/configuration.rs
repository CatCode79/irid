
//= RENDERER CONFIG BUILDER ========================================================================

/// Build a new [RendererConfig] with wanted values.
#[derive(Clone, Debug)]
pub struct RendererConfigBuilder {
    clear_color: Option<wgpu::Color>,
}


/// Application configuration.
impl RendererConfigBuilder {
    pub const DEFAULT_CLEAR_COLOR: Option<wgpu::Color> = Some(wgpu::Color::WHITE);

    /// Create it to build new [RendererConfig].
    pub fn new() -> Self {
        Self {
            clear_color: None,
            ..Default::default()
        }
    }

    /// Color used by a [render pass color attachment](wgpu::RenderPassColorAttachment)
    /// to perform a [clear operation](wgpu::LoadOp).
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    /// Build a new [Config] with the set values.
    pub fn build(self) -> RendererConfig {
        RendererConfig {
            clear_color: if self.clear_color
                .is_some() { self.clear_color.unwrap() }
            else { RendererConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap() },
        }
    }
}


impl Default for RendererConfigBuilder {
    fn default() -> Self {
        Self {
            clear_color: RendererConfigBuilder::DEFAULT_CLEAR_COLOR,
        }
    }
}


//= RENDERER CONFIG OBJECT =========================================================================

/// The [Renderer](irid-renderer::Renderer) configuration, TODO: readable by file with
/// [serde](https://crates.io/crates/serde).
#[derive(Clone, Debug)]
pub struct RendererConfig {
    clear_color: wgpu::Color,
}


impl RendererConfig {

    //- Constructors -------------------------------------------------------------------------------

    /// Create a Config struct by reading the values from given file path.
    pub fn new(_filepath: &std::path::Path) -> Self {
        RendererConfig::default()
    }

    //- Getters ------------------------------------------------------------------------------------

    /// Returns the clear color used in a
    /// [render pass color attachment](wgpu::RenderPassColorAttachment).
    pub fn clear_color(&self) -> wgpu::Color {
        self.clear_color
    }
}


impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            clear_color: RendererConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap(),
        }
    }
}
