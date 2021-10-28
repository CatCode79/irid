
//= APPLICATION CONFIG BUILDER =====================================================================

/// Build a new [Config] with wanted values.
#[derive(Debug)]
pub struct ConfigBuilder {
    clear_color: Option<wgpu::Color>,
    window_inner_width: Option<std::num::NonZeroU32>,
    window_inner_height: Option<std::num::NonZeroU32>,
    window_starts_maximized: bool,
}


/// Application's configuration.
impl ConfigBuilder {
    pub const DEFAULT_CLEAR_COLOR: Option<wgpu::Color> =
        Some(wgpu::Color::WHITE);
    pub const DEFAULT_WINDOW_INNER_WIDTH: Option<std::num::NonZeroU32> =
        std::num::NonZeroU32::new(1920 / 2);
    pub const DEFAULT_WINDOW_INNER_HEIGHT: Option<std::num::NonZeroU32> =
        std::num::NonZeroU32::new(1080 / 2);
    pub const DEFAULT_WINDOW_STARTS_MAXIMIZED: bool =
        true;  // false value as default can gives less starting problems

    /// Create it to build new [Config]s.
    pub fn new() -> Self {
        Self {
            clear_color: None,
            window_inner_width: None,
            window_inner_height: None,
            ..Default::default()
        }
    }

    /// Color used by a [render pass color attachment](wgpu::RenderPassColorAttachment)
    /// to perform a [clear operation](wgpu::LoadOp).
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    /// Set the width window inner size.
    pub fn with_window_inner_width(mut self, window_inner_width: u32) -> Self {
        self.window_inner_width = match window_inner_width {
            0 => {
                log::warn!("A value equal to zero has been given to window_width, the default value of {} will be set instead",
                    ConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap());
                ConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH
            },
            _ => std::num::NonZeroU32::new(window_inner_width),
        };
        self
    }

    /// Set the height window inner size.
    pub fn with_window_inner_height(mut self, window_inner_height: u32) -> Self {
        self.window_inner_height = match window_inner_height {
            0 => {
                log::warn!("A value equal to zero has been given to window_height, the default value of {} will be set instead",
                    ConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap());
                ConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT
            },
            _ => std::num::NonZeroU32::new(window_inner_height),
        };
        self
    }

    pub fn with_window_starts_maximized(mut self, window_starts_maximized: bool) -> Self {
        self.window_starts_maximized = window_starts_maximized;
        self
    }

    /// Build a new [Config] with the set values.
    pub fn build(self) -> Config {
        Config {
            clear_color: if self.clear_color
                .is_some() { self.clear_color.unwrap() }
                else { ConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap() },
            window_inner_width: if self.window_inner_width
                .is_some() { self.window_inner_width.unwrap() }
                else { ConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap() },
            window_inner_height: if self.window_inner_height
                .is_some() { self.window_inner_height.unwrap() }
                else { ConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap() },
            window_starts_maximized: self.window_starts_maximized,
        }
    }
}


impl Default for ConfigBuilder {
    fn default() -> ConfigBuilder {
        ConfigBuilder {
            clear_color: ConfigBuilder::DEFAULT_CLEAR_COLOR,
            window_inner_width: ConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH,
            window_inner_height: ConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT,
            window_starts_maximized: ConfigBuilder::DEFAULT_WINDOW_STARTS_MAXIMIZED,
        }
    }
}


//= APPLICATION CONFIG OBJECT ======================================================================

/// The [Application](crate::app::Application)'s configuration, TODO: readable by file with
/// [serde](https://crates.io/crates/serde).
#[derive(Debug)]
pub struct Config {
    clear_color: wgpu::Color,
    window_inner_width: std::num::NonZeroU32,
    window_inner_height: std::num::NonZeroU32,
    window_starts_maximized: bool,
}


impl Config {
    /// Create a Config struct by reading the values from given file path.
    pub fn new(_filepath: &std::path::Path) -> Self {
        Config::default()
    }

    /// Returns the clear color used in a
    /// [render pass color attachment](wgpu::RenderPassColorAttachment).
    pub fn clear_color(&self) -> wgpu::Color {
        self.clear_color
    }

    /// Returns the window inner width (used when the window is not maximized).
    ///
    /// The returned value is a NonZeroU32 to avoid division by zero on computing the
    /// [display aspect ratio](https://en.wikipedia.org/wiki/Display_aspect_ratio).
    pub fn window_inner_width(&self) -> std::num::NonZeroU32 {
        self.window_inner_width
    }

    /// Returns the window inner height (used when the window is not maximized).
    ///
    /// The returned value is a NonZeroU32 to avoid division by zero on computing the
    /// [display aspect ratio](https://en.wikipedia.org/wiki/Display_aspect_ratio).
    pub fn window_inner_height(&self) -> std::num::NonZeroU32 {
        self.window_inner_height
    }

    /// Checks if the game's window starts maximized.
    pub fn window_starts_maximized(&self) -> bool {
        self.window_starts_maximized
    }

    /// Returns the window inner size.
    pub fn window_inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize {
            width: self.window_inner_width.get(),
            height: self.window_inner_height.get(),
        }
    }

    /// Returns the minimum window inner size. You cannot resize the window below these values.
    pub fn window_min_inner_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize {
            width: self.window_inner_width.get() / 2,
            height: self.window_inner_height.get() / 2,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: ConfigBuilder::DEFAULT_CLEAR_COLOR.unwrap(),
            window_inner_width: ConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap(),
            window_inner_height: ConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap(),
            window_starts_maximized: ConfigBuilder::DEFAULT_WINDOW_STARTS_MAXIMIZED,
        }
    }
}
