//= APPLICATION CONFIG BUILDER =====================================================================

/// Build a new [AppConfig] with wanted values.
#[derive(Clone, Debug)]
pub struct AppConfigBuilder {
    window_inner_width: Option<std::num::NonZeroU32>,
    window_inner_height: Option<std::num::NonZeroU32>,
    window_starts_maximized: bool,
}

/// Application configuration.
impl AppConfigBuilder {
    pub const DEFAULT_WINDOW_INNER_WIDTH: Option<std::num::NonZeroU32> =
        std::num::NonZeroU32::new(1920 / 2);
    pub const DEFAULT_WINDOW_INNER_HEIGHT: Option<std::num::NonZeroU32> =
        std::num::NonZeroU32::new(1080 / 2);
    pub const DEFAULT_WINDOW_STARTS_MAXIMIZED: bool =
        true;  // false value as default can gives less starting problems

    /// Create it to build new [AppConfig].
    pub fn new() -> Self {
        Self {
            window_inner_width: None,
            window_inner_height: None,
            ..Default::default()
        }
    }

    /// Set the width window inner size.
    pub fn with_window_inner_width(mut self, window_inner_width: u32) -> Self {
        self.window_inner_width = match window_inner_width {
            0 => {
                log::warn!("A value equal to zero has been given to window_width, the default value of {} will be set instead",
                    AppConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap());
                AppConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH
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
                    AppConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap());
                AppConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT
            },
            _ => std::num::NonZeroU32::new(window_inner_height),
        };
        self
    }

    pub fn with_window_starts_maximized(mut self, window_starts_maximized: bool) -> Self {
        self.window_starts_maximized = window_starts_maximized;
        self
    }

    /// Build a new [AppConfig] with the set values.
    pub fn build(self) -> ApplicationConfig {
        ApplicationConfig {
            window_inner_width: if self.window_inner_width
                .is_some() { self.window_inner_width.unwrap() }
                else { AppConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap() },
            window_inner_height: if self.window_inner_height
                .is_some() { self.window_inner_height.unwrap() }
                else { AppConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap() },
            window_starts_maximized: self.window_starts_maximized,
        }
    }
}

impl Default for AppConfigBuilder {
    fn default() -> Self {
        Self {
            window_inner_width: AppConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH,
            window_inner_height: AppConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT,
            window_starts_maximized: AppConfigBuilder::DEFAULT_WINDOW_STARTS_MAXIMIZED,
        }
    }
}

//= APPLICATION CONFIG =============================================================================

/// The [Application](crate::app::Application) configuration, TODO: readable by file with
/// [serde](https://crates.io/crates/serde).
#[derive(Clone, Debug)]
pub struct ApplicationConfig {
    window_inner_width: std::num::NonZeroU32,
    window_inner_height: std::num::NonZeroU32,
    window_starts_maximized: bool,
}

impl ApplicationConfig {
    //- Constructors -------------------------------------------------------------------------------

    /// Create a Config struct by reading the values from given file path.
    pub fn new<P: AsRef<std::path::Path>>(#[allow(unused)] filepath: P) -> Self {
        ApplicationConfig::default()
    }

    //- Getters ------------------------------------------------------------------------------------

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

    //- Windows Inner Size -------------------------------------------------------------------------

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

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            window_inner_width: AppConfigBuilder::DEFAULT_WINDOW_INNER_WIDTH.unwrap(),
            window_inner_height: AppConfigBuilder::DEFAULT_WINDOW_INNER_HEIGHT.unwrap(),
            window_starts_maximized: AppConfigBuilder::DEFAULT_WINDOW_STARTS_MAXIMIZED,
        }
    }
}
