/// Application's configuration.

//= CONFIG BUILDER =================================================================================

#[derive(Debug)]
pub struct ConfigBuilder {
    clear_color: Option<wgpu::Color>,
    window_width: Option<std::num::NonZeroU32>,
    window_height: Option<std::num::NonZeroU32>,
}


impl ConfigBuilder {
    pub const DEFAULT_CLEAR_COLOR: Option<wgpu::Color> = Some(wgpu::Color::WHITE);
    pub const DEFAULT_WINDOW_WIDTH: Option<std::num::NonZeroU32> = std::num::NonZeroU32::new(1024);
    pub const DEFAULT_WINDOW_HEIGHT: Option<std::num::NonZeroU32> = std::num::NonZeroU32::new(768);

    ///
    pub fn new() -> Self {
        Self {
            clear_color: None,
            window_width: None,
            window_height: None,
        }
    }

    ///
    pub fn clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    ///
    pub fn window_width(mut self, window_width: u32) -> Self {
        self.window_width = match window_width {
            0 => {
                log::warn!("A value equal to zero has been given to window_width, the default value of {} will be set instead",
                    ConfigBuilder::DEFAULT_WINDOW_WIDTH.unwrap());
                ConfigBuilder::DEFAULT_WINDOW_WIDTH
            },
            _ => std::num::NonZeroU32::new(window_width),
        };
        self
    }

    ///
    pub fn window_height(mut self, window_height: u32) -> Self {
        self.window_height = match window_height {
            0 => {
                log::warn!("A value equal to zero has been given to window_height, the default value of {} will be set instead",
                    ConfigBuilder::DEFAULT_WINDOW_HEIGHT.unwrap());
                ConfigBuilder::DEFAULT_WINDOW_HEIGHT
            },
            _ => std::num::NonZeroU32::new(window_height),
        };
        self
    }

    ///
    pub fn build(self) -> Config {
        Config {
            clear_color: if self.clear_color.is_some() { self.clear_color.unwrap() }
                else { ConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap() },
            window_width: if self.window_width.is_some() { self.window_width.unwrap() }
                else { ConfigBuilder::DEFAULT_WINDOW_WIDTH.unwrap() },
            window_height: if self.window_height.is_some() { self.window_height.unwrap() }
                else { ConfigBuilder::DEFAULT_WINDOW_HEIGHT.unwrap() },
        }
    }
}


impl Default for ConfigBuilder {
    fn default() -> ConfigBuilder {
        ConfigBuilder {
            clear_color: ConfigBuilder::DEFAULT_CLEAR_COLOR,
            window_width: ConfigBuilder::DEFAULT_WINDOW_WIDTH,
            window_height: ConfigBuilder::DEFAULT_WINDOW_HEIGHT,
        }
    }
}


//= CONFIG STRUCT ==================================================================================

#[derive(Debug)]
pub struct Config {
    clear_color: wgpu::Color,
    window_width: std::num::NonZeroU32,
    window_height: std::num::NonZeroU32,
}


// TODO: serializzare con serde
impl Config {
    /// Create a new config by reading the values from given file path.
    pub fn new(_filepath: &std::path::Path) -> Self {
        Config::default()
    }

    /// Return the clear color (used in render pass color attachment).
    pub fn clear_color(&self) -> wgpu::Color {
        self.clear_color
    }

    /// Return the window width resolution in pixels (used when the window is not maximized).
    ///
    /// The returned value is a NonZeroU32 to avoid division by zero on computing the screen's ratio.
    pub fn window_width(&self) -> std::num::NonZeroU32 {
        self.window_width
    }

    /// Return the window height resolution in pixels (used when the window is not maximized).
    ///
    /// The returned value is a NonZeroU32 to avoid division by zero on computing the screen's ratio.
    pub fn window_height(&self) -> std::num::NonZeroU32 {
        self.window_height
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: ConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap(),
            window_width: ConfigBuilder::DEFAULT_WINDOW_WIDTH.unwrap(),
            window_height: ConfigBuilder::DEFAULT_WINDOW_HEIGHT.unwrap(),
        }
    }
}
