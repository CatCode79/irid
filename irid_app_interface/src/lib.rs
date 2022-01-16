#![warn(
absolute_paths_not_starting_with_crate,
box_pointers,
elided_lifetimes_in_paths,
explicit_outlives_requirements,
keyword_idents,
macro_use_extern_crate,
meta_variable_misuse,
missing_abi,
missing_copy_implementations,
missing_debug_implementations,
//missing_docs,
non_ascii_idents,
noop_method_call,
pointer_structural_match,
rust_2021_incompatible_closure_captures,
rust_2021_incompatible_or_patterns,
rust_2021_prefixes_incompatible_syntax,
rust_2021_prelude_collisions,
single_use_lifetimes,
trivial_casts,
trivial_numeric_casts,
unreachable_pub,
unsafe_code,
unsafe_op_in_unsafe_fn,
unstable_features,
unused_crate_dependencies,
unused_extern_crates,
unused_import_braces,
unused_lifetimes,
unused_qualifications,
unused_results,
variant_size_differences,
// We don't match on a reference, unless required.
clippy::pattern_type_mismatch,
)]

//= WINDOW BUILDER TRAIT ===========================================================================

///
pub trait WindowBuilder {
    //- Associated Types ---------------------------------------------------------------------------

    /// **Associated type** regarding the build result.
    type BuildOutput;

    //- Method Signatures --------------------------------------------------------------------------

    /// Initializes a new `WindowBuilder` with default values.
    fn new() -> Self;

    /// Requests the window to be of specific dimensions.
    ///
    /// See [`Window::set_inner_size`] for details.
    ///
    /// [`Window::set_inner_size`]: winit::window::Window::set_inner_size
    fn with_inner_size<S: Into<winit::dpi::Size>>(self, size: S) -> Self;

    /// Sets a minimum dimension size for the window.
    ///
    /// See [`Window::set_min_inner_size`] for details.
    ///
    /// [`Window::set_min_inner_size`]: winit::window::Window::set_min_inner_size
    fn with_min_inner_size<S: Into<winit::dpi::Size>>(self, min_size: S) -> Self;

    /// Sets a maximum dimension size for the window.
    ///
    /// See [`Window::set_max_inner_size`] for details.
    ///
    /// [`Window::set_max_inner_size`]: winit::window::Window::set_max_inner_size
    fn with_max_inner_size<S: Into<winit::dpi::Size>>(self, max_size: S) -> Self;

    /// Sets a desired initial position for the window.
    ///
    /// See [`WindowAttributes::position`] for details.
    ///
    /// [`WindowAttributes::position`]: crate::window::WindowAttributes::position
    fn with_position<P: Into<winit::dpi::Position>>(self, position: P) -> Self;

    /// Sets whether the window is resizable or not.
    ///
    /// See [`Window::set_resizable`] for details.
    ///
    /// [`Window::set_resizable`]: crate::window::Window::set_resizable
    fn with_resizable(self, resizable: bool) -> Self;

    /// Requests a specific title for the window.
    ///
    /// See [`Window::set_title`] for details.
    ///
    /// [`Window::set_title`]: crate::window::Window::set_title
    fn with_title<T: Into<String>>(self, title: T) -> Self;

    /// Sets the window fullscreen state.
    ///
    /// See [`Window::set_fullscreen`] for details.
    ///
    /// [`Window::set_fullscreen`]: crate::window::Window::set_fullscreen
    fn with_fullscreen(self, fullscreen: Option<winit::window::Fullscreen>) -> Self;

    /// Requests maximized mode.
    ///
    /// See [`Window::set_maximized`] for details.
    ///
    /// [`Window::set_maximized`]: crate::window::Window::set_maximized
    fn with_maximized(self, maximized: bool) -> Self;

    /// Sets whether the window will be initially hidden or visible.
    ///
    /// See [`Window::set_visible`] for details.
    ///
    /// [`Window::set_visible`]: crate::window::Window::set_visible
    fn with_visible(self, visible: bool) -> Self;

    /// Sets whether the background of the window should be transparent.
    fn with_transparent(self, transparent: bool) -> Self;

    /// Sets whether the window should have a border, a title bar, etc.
    ///
    /// See [`Window::set_decorations`] for details.
    ///
    /// [`Window::set_decorations`]: crate::window::Window::set_decorations
    fn with_decorations(self, decorations: bool) -> Self;

    /// Sets whether or not the window will always be on top of other windows.
    ///
    /// See [`Window::set_always_on_top`] for details.
    ///
    /// [`Window::set_always_on_top`]: crate::window::Window::set_always_on_top
    fn with_always_on_top(self, always_on_top: bool) -> Self;

    /// Sets the window icon.
    ///
    /// See [`Window::set_window_icon`] for details.
    ///
    /// [`Window::set_window_icon`]: crate::window::Window::set_window_icon
    fn with_window_icon(self, window_icon: Option<winit::window::Icon>) -> Self;

    /// Builds the window.
    ///
    /// Possible causes of error include denied permission, incompatible system, and lack of memory.
    fn build(
        self,
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Self::BuildOutput, winit::error::OsError>;
}

//= WINDOW TRAIT ===================================================================================

pub trait Window {
    //- Associated Types ---------------------------------------------------------------------------

    type Output;

    //- Base Window Functions ----------------------------------------------------------------------

    /// Creates a new Window for platforms where this is appropriate.
    ///
    /// This function is equivalent to [`WindowBuilder::new().build(event_loop)`].
    ///
    /// Error should be very rare and only occur in case of permission denied, incompatible system,
    ///  out of memory, etc.
    ///
    /// [`WindowBuilder::new().build(event_loop)`]: crate::window::WindowBuilder::build
    fn new(
        event_loop: &winit::event_loop::EventLoop<()>,
    ) -> Result<Self::Output, winit::error::OsError>;

    /// Returns an identifier unique to the window.
    fn id(&self) -> winit::window::WindowId;

    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    ///
    /// See the [`dpi`](crate::dpi) module for more information.
    ///
    /// Note that this value can change depending on user action (for example if the window is
    /// moved to another screen); as such, tracking `WindowEvent::ScaleFactorChanged` events is
    /// the most robust way to track the DPI you need to use to draw.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** This respects Xft.dpi, and can be overridden using the `WINIT_X11_SCALE_FACTOR` environment variable.
    /// - **Android:** Always returns 1.0.
    /// - **iOS:** Can only be called on the main thread. Returns the underlying `UIView`'s
    ///   [`contentScaleFactor`].
    ///
    /// [`contentScaleFactor`]: https://developer.apple.com/documentation/uikit/uiview/1622657-contentscalefactor?language=objc
    fn scale_factor(&self) -> f64;

    /// Emits a `WindowEvent::RedrawRequested` event in the associated event loop after all OS
    /// events have been processed by the event loop.
    ///
    /// This is the **strongly encouraged** method of redrawing windows, as it can integrate with
    /// OS-requested redraws (e.g. when a window gets resized).
    ///
    /// This function can cause `RedrawRequested` events to be emitted after `Event::MainEventsCleared`
    /// but before `Event::NewEvents` if called in the following circumstances:
    /// * While processing `MainEventsCleared`.
    /// * While processing a `RedrawRequested` event that was sent during `MainEventsCleared` or any
    ///   directly subsequent `RedrawRequested` event.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread.
    /// - **Android:** Subsequent calls after `MainEventsCleared` are not handled.
    fn request_redraw(&self);

    //- Position and Size Functions ----------------------------------------------------------------

    /// Returns the position of the top-left hand corner of the window's client area relative to the
    /// top-left hand corner of the desktop.
    ///
    /// The same conditions that apply to `outer_position` apply to this method.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the top left coordinates of the
    ///   window's [safe area] in the screen space coordinate system.
    /// - **Android / Wayland:** Always returns [`NotSupportedError`].
    ///
    /// [safe area]: https://developer.apple.com/documentation/uikit/uiview/2891103-safeareainsets?language=objc
    fn inner_position(
        &self,
    ) -> Result<winit::dpi::PhysicalPosition<i32>, winit::error::NotSupportedError>;

    /// Returns the position of the top-left hand corner of the window relative to the
    ///  top-left hand corner of the desktop.
    ///
    /// Note that the top-left hand corner of the desktop is not necessarily the same as
    ///  the screen. If the user uses a desktop with multiple monitors, the top-left hand corner
    ///  of the desktop is the top-left hand corner of the monitor at the top-left of the desktop.
    ///
    /// The coordinates can be negative if the top-left hand corner of the window is outside
    ///  of the visible screen region.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the top left coordinates of the
    ///   window in the screen space coordinate system.
    /// - **Android / Wayland:** Always returns [`NotSupportedError`].
    fn outer_position(
        &self,
    ) -> Result<winit::dpi::PhysicalPosition<i32>, winit::error::NotSupportedError>;

    /// Modifies the position of the window.
    ///
    /// See `outer_position` for more information about the coordinates. This automatically un-maximizes the
    /// window if it's maximized.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalPosition, PhysicalPosition};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the position in logical dimensions like this:
    /// window.set_outer_position(LogicalPosition::new(400.0, 200.0));
    ///
    /// // Or specify the position in physical dimensions like this:
    /// window.set_outer_position(PhysicalPosition::new(400, 200));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Sets the top left coordinates of the
    ///   window in the screen space coordinate system.
    /// - **Android / Wayland:** Unsupported.
    fn set_outer_position<P: Into<winit::dpi::Position>>(&self, position: P);

    /// Returns the physical size of the window's client area.
    ///
    /// The client area is the content of the window, excluding the title bar and borders.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the `PhysicalSize` of the window's
    ///   [safe area] in screen space coordinates.
    ///
    /// [safe area]: https://developer.apple.com/documentation/uikit/uiview/2891103-safeareainsets?language=objc
    fn inner_size(&self) -> winit::dpi::PhysicalSize<u32>;

    /// Modifies the inner size of the window.
    ///
    /// See `inner_size` for more information about the values. This automatically un-maximizes the
    /// window if it's maximized.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalSize, PhysicalSize};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the size in logical dimensions like this:
    /// window.set_inner_size(LogicalSize::new(400.0, 200.0));
    ///
    /// // Or specify the size in physical dimensions like this:
    /// window.set_inner_size(PhysicalSize::new(400, 200));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_inner_size<S: Into<winit::dpi::Size>>(&self, size: S);

    /// Returns the physical size of the entire window.
    ///
    /// These dimensions include the title bar and borders.
    /// If you don't want that (and you usually don't), use `inner_size` instead.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread. Returns the `PhysicalSize` of the window
    ///   in screen space coordinates.
    fn outer_size(&self) -> winit::dpi::PhysicalSize<u32>;

    /// Sets a minimum dimension size for the window.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalSize, PhysicalSize};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the size in logical dimensions like this:
    /// window.set_min_inner_size(Some(LogicalSize::new(400.0, 200.0)));
    ///
    /// // Or specify the size in physical dimensions like this:
    /// window.set_min_inner_size(Some(PhysicalSize::new(400, 200)));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_min_inner_size<S: Into<winit::dpi::Size>>(&self, min_size: Option<S>);

    /// Sets a maximum dimension size for the window.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalSize, PhysicalSize};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the size in logical dimensions like this:
    /// window.set_max_inner_size(Some(LogicalSize::new(400.0, 200.0)));
    ///
    /// // Or specify the size in physical dimensions like this:
    /// window.set_max_inner_size(Some(PhysicalSize::new(400, 200)));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_max_inner_size<S: Into<winit::dpi::Size>>(&self, max_size: Option<S>);

    //- Misc. Attribute Functions ------------------------------------------------------------------

    /// Modifies the title of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_title(&self, title: &str);

    /// Modifies the window's visibility.
    ///
    /// If `false`, this will hide the window. If `true`, this will show the window.
    /// ## Platform-specific
    ///
    /// - **Android / Wayland:** Unsupported.
    /// - **iOS:** Can only be called on the main thread.
    fn set_visible(&self, visible: bool);

    /// Sets whether the window is resizable or not.
    ///
    /// Note that making the window unresizable doesn't exempt you from handling `Resized`, as that event can still be
    /// triggered by DPI scaling, entering fullscreen mode, etc.
    ///
    /// ## Platform-specific
    ///
    /// This only has an effect on desktop platforms.
    ///
    /// Due to a bug in XFCE, this has no effect on Xfwm.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_resizable(&self, resizable: bool);

    /// Sets the window to minimized or back
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    /// - **Wayland:** Un-minimize is unsupported.
    fn set_minimized(&self, minimized: bool);

    /// Sets the window to maximized or back.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_maximized(&self, maximized: bool);

    /// Gets the window's current maximized state.
    ///
    /// ## Platform-specific
    ///
    /// - **Wayland / X11:** Not implemented.
    /// - **iOS / Android:** Unsupported.
    fn is_maximized(&self) -> bool;

    /// Sets the window to fullscreen or back.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** `Fullscreen::Exclusive` provides true exclusive mode with a
    ///   video mode change. *Caveat!* macOS doesn't provide task switching (or
    ///   spaces!) while in exclusive fullscreen mode. This mode should be used
    ///   when a video mode change is desired, but for a better user experience,
    ///   borderless fullscreen might be preferred.
    ///
    ///   `Fullscreen::Borderless` provides a borderless fullscreen window on a
    ///   separate space. This is the idiomatic way for fullscreen games to work
    ///   on macOS. See `WindowExtMacOs::set_simple_fullscreen` if
    ///   separate spaces are not preferred.
    ///
    ///   The dock and the menu bar are always disabled in fullscreen mode.
    /// - **iOS:** Can only be called on the main thread.
    /// - **Wayland:** Does not support exclusive fullscreen mode and will no-op a request.
    /// - **Windows:** Screen saver is disabled in fullscreen mode.
    /// - **Android:** Unsupported.
    fn set_fullscreen(&self, fullscreen: Option<winit::window::Fullscreen>);

    /// Gets the window's current fullscreen state.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS:** Can only be called on the main thread.
    /// - **Android:** Will always return `None`.
    /// - **Wayland:** Can return `Borderless(None)` when there are no monitors.
    fn fullscreen(&self) -> Option<winit::window::Fullscreen>;

    /// Turn window decorations on or off.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    ///
    /// [`setPrefersStatusBarHidden`]: https://developer.apple.com/documentation/uikit/uiviewcontroller/1621440-prefersstatusbarhidden?language=objc
    fn set_decorations(&self, decorations: bool);

    /// Change whether or not the window will always be on top of other windows.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Wayland:** Unsupported.
    fn set_always_on_top(&self, always_on_top: bool);

    /// Sets the window icon. On Windows and X11, this is typically the small icon in the top-left
    /// corner of the titlebar.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Wayland / macOS:** Unsupported.
    ///
    /// On Windows, this sets `ICON_SMALL`. The base size for a window icon is 16x16, but it's
    /// recommended to account for screen scaling and pick a multiple of that, i.e. 32x32.
    ///
    /// X11 has no universal guidelines for icon sizes, so you're at the whims of the WM. That
    /// said, it's usually in the same ballpark as on Windows.
    fn set_window_icon(&self, window_icon: Option<winit::window::Icon>);

    /// Sets location of IME candidate box in client area coordinates relative to the top left.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalPosition, PhysicalPosition};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the position in logical dimensions like this:
    /// window.set_ime_position(LogicalPosition::new(400.0, 200.0));
    ///
    /// // Or specify the position in physical dimensions like this:
    /// window.set_ime_position(PhysicalPosition::new(400, 200));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_ime_position<P: Into<winit::dpi::Position>>(&self, position: P);

    /*
    /// Brings the window to the front and sets input focus. Has no effect if the window is
    /// already in focus, minimized, or not visible.
    ///
    /// This method steals input focus from other applications. Do not use this method unless
    /// you are certain that's what the user wants. Focus stealing can cause an extremely disruptive
    /// user experience.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Wayland:** Unsupported.
    fn focus_window(&self);
    */

    /// Requests user attention to the window, this has no effect if the application
    /// is already focused. How requesting for user attention manifests is platform dependent,
    /// see `UserAttentionType` for details.
    ///
    /// Providing `None` will unset the request for user attention. Unsetting the request for
    /// user attention might not be done automatically by the WM when the window receives input.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    /// - **macOS:** `None` has no effect.
    /// - **X11:** Requests for user attention must be manually cleared.
    /// - **Wayland:** Requires `xdg_activation_v1` protocol, `None` has no effect.
    fn request_user_attention(&self, request_type: Option<winit::window::UserAttentionType>);

    //- Cursor Functions ---------------------------------------------------------------------------

    /// Modifies the cursor icon of the window.
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android:** Unsupported.
    fn set_cursor_icon(&self, cursor: winit::window::CursorIcon);

    /// Changes the position of the cursor in window coordinates.
    ///
    /// ```no_run
    /// # use winit::dpi::{LogicalPosition, PhysicalPosition};
    /// # use winit::event_loop::EventLoop;
    /// # use winit::window::Window;
    /// # let mut event_loop = EventLoop::new();
    /// # let window = Window::new(&event_loop).unwrap();
    /// // Specify the position in logical dimensions like this:
    /// window.set_cursor_position(LogicalPosition::new(400.0, 200.0));
    ///
    /// // Or specify the position in physical dimensions like this:
    /// window.set_cursor_position(PhysicalPosition::new(400, 200));
    /// ```
    ///
    /// ## Platform-specific
    ///
    /// - **iOS / Android / Wayland:** Always returns an [`ExternalError::NotSupported`].
    fn set_cursor_position<P: Into<winit::dpi::Position>>(
        &self,
        position: P,
    ) -> Result<(), winit::error::ExternalError>;

    /// Grabs the cursor, preventing it from leaving the window.
    ///
    /// There's no guarantee that the cursor will be hidden. You should
    /// hide it by yourself if you want so.
    ///
    /// ## Platform-specific
    ///
    /// - **macOS:** This locks the cursor in a fixed location, which looks visually awkward.
    /// - **iOS / Android:** Always returns an [`ExternalError::NotSupported`].
    fn set_cursor_grab(&self, grab: bool) -> Result<(), winit::error::ExternalError>;

    /// Modifies the cursor's visibility.
    ///
    /// If `false`, this will hide the cursor. If `true`, this will show the cursor.
    ///
    /// ## Platform-specific
    ///
    /// - **Windows:** The cursor is only hidden within the confines of the window.
    /// - **X11:** The cursor is only hidden within the confines of the window.
    /// - **Wayland:** The cursor is only hidden within the confines of the window.
    /// - **macOS:** The cursor is hidden as long as the window has input focus, even if the cursor
    ///   is outside of the window.
    /// - **iOS / Android:** Unsupported.
    fn set_cursor_visible(&self, visible: bool);

    /// Moves the window with the left mouse button until the button is released.
    ///
    /// There's no guarantee that this will work unless the left mouse button was pressed
    /// immediately before this function is called.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** Un-grabs the cursor.
    /// - **Wayland:** Requires the cursor to be inside the window to be dragged.
    /// - **macOS:** May prevent the button release event to be triggered.
    /// - **iOS / Android:** Always returns an [`ExternalError::NotSupported`].
    fn drag_window(&self) -> Result<(), winit::error::ExternalError>;

    //- Monitor Info Functions ---------------------------------------------------------------------

    /// Returns the monitor on which the window currently resides.
    ///
    /// Returns `None` if current monitor can't be detected.
    ///
    /// ## Platform-specific
    ///
    /// **iOS:** Can only be called on the main thread.
    fn current_monitor(&self) -> Option<winit::monitor::MonitorHandle>;

    // TODO: this is possible to correct using a platform_impl Window (other solutions?)
    /*
    /// Returns the list of all the monitors available on the system.
    ///
    /// This is the same as `EventLoopWindowTarget::available_monitors`, and is provided
    /// for convenience.
    ///
    /// ## Platform-specific
    ///
    /// **iOS:** Can only be called on the main thread.
    fn available_monitors(&self) -> Box<dyn Iterator<Item = winit::monitor::MonitorHandle>>;
     */

    /// Returns the primary monitor of the system.
    ///
    /// Returns `None` if it can't identify any monitor as a primary one.
    ///
    /// This is the same as `EventLoopWindowTarget::primary_monitor`, and is provided for convenience.
    ///
    /// ## Platform-specific
    ///
    /// **iOS:** Can only be called on the main thread.
    /// **Wayland:** Always returns `None`.
    fn primary_monitor(&self) -> Option<winit::monitor::MonitorHandle>;

    //- Wrapper Functions --------------------------------------------------------------------------

    ///
    fn expose_inner_window(&self) -> &winit::window::Window;

    ///
    fn conclude_visibility_delay(&mut self);
}
