//= USES ===========================================================================================

use winit::dpi::{PhysicalPosition, PhysicalSize, Position, Size};
use winit::error::{ExternalError, NotSupportedError, OsError};
use winit::monitor::MonitorHandle;
use winit::window::{CursorIcon, Fullscreen, Icon, UserAttentionType, WindowId};

use irid_app_interface::{Window, WindowBuilder};

//= IRID WINDOW BUILDER ============================================================================

///
#[derive(Clone, Debug)]
pub struct IridWindowBuilder {
    winit_builder: winit::window::WindowBuilder,
    postponed_visibility: bool,
}

impl Default for IridWindowBuilder {
    fn default() -> Self {
        IridWindowBuilder {
            winit_builder: winit::window::WindowBuilder::default(),
            postponed_visibility: true,
        }
        /*.with_inner_size(winit::dpi::PhysicalSize {
            width: 1980 / 2,
            height: 720 / 2,
        });*/
        .with_min_inner_size(winit::dpi::PhysicalSize {
            width: 1980 / 4,
            height: 720 / 4,
        })
        .with_resizable(true)
        .with_title("Irid Application")
        .with_visible(true)
    }
}

impl WindowBuilder for IridWindowBuilder {
    //- Associated Types ---------------------------------------------------------------------------

    type BuildOutput = IridWindow;

    //- Constructors -------------------------------------------------------------------------------

    fn new() -> Self {
        Self::default()
    }

    //- Setters ------------------------------------------------------------------------------------

    fn with_inner_size<S: Into<winit::dpi::Size>>(mut self, size: S) -> Self {
        self.winit_builder.window.inner_size = Some(size.into());
        self
    }

    fn with_min_inner_size<S: Into<winit::dpi::Size>>(mut self, min_size: S) -> Self {
        self.winit_builder.window.min_inner_size = Some(min_size.into());
        self
    }

    fn with_max_inner_size<S: Into<winit::dpi::Size>>(mut self, max_size: S) -> Self {
        self.winit_builder.window.max_inner_size = Some(max_size.into());
        self
    }

    fn with_position<P: Into<winit::dpi::Position>>(mut self, position: P) -> Self {
        self.winit_builder.window.position = Some(position.into());
        self
    }

    fn with_resizable(mut self, resizable: bool) -> Self {
        self.winit_builder.window.resizable = resizable;
        self
    }

    fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.winit_builder.window.title = title.into();
        self
    }

    fn with_fullscreen(mut self, fullscreen: Option<winit::window::Fullscreen>) -> Self {
        self.winit_builder.window.fullscreen = fullscreen;
        self
    }

    fn with_maximized(mut self, maximized: bool) -> Self {
        self.winit_builder.window.maximized = maximized;
        self
    }

    fn with_visible(mut self, visible: bool) -> Self {
        self.winit_builder.window.visible = visible;
        self
    }

    fn with_transparent(mut self, transparent: bool) -> Self {
        self.winit_builder.window.transparent = transparent;
        self
    }

    fn with_decorations(mut self, decorations: bool) -> Self {
        self.winit_builder.window.decorations = decorations;
        self
    }

    fn with_always_on_top(mut self, always_on_top: bool) -> Self {
        self.winit_builder.window.always_on_top = always_on_top;
        self
    }

    fn with_window_icon(mut self, window_icon: Option<winit::window::Icon>) -> Self {
        self.winit_builder.window.window_icon = window_icon;
        self
    }

    //- Building -----------------------------------------------------------------------------------

    fn build(self) -> Result<(Self::BuildOutput, winit::event_loop::EventLoop<()>), OsError> {
        let event_loop = winit::event_loop::EventLoop::new();

        // TODO: it could be considered questionable to give a different behavior than usual, probably remove this part
        /*
        // In this particular case the borderless fullscreen is forced instead of maximization
        if self.winit_builder.window.maximized && self.winit_builder.window.fullscreen.is_none() {
            // Searching for primary monitor on Wayland returns always None
            if let Some(primary_monitor) = event_loop.primary_monitor() {
                self.with_fullscreen(
                    Some(winit::window::Fullscreen::Borderless(Some(primary_monitor)))
                );
            };
        }
        */

        Ok((IridWindow {
            winit_window: self.winit_builder.build(&event_loop)?,
            visible: self.postponed_visibility,
        }, winit::event_loop::EventLoop::new()))
    }
}

//= IRID WINDOW ====================================================================================

pub struct IridWindow {
    winit_window: winit::window::Window,
    visible: bool,
}

impl Window for IridWindow {
    //- Associated Types ---------------------------------------------------------------------------

    type Output = IridWindow;

    //- Base Window Functions ----------------------------------------------------------------------

    #[inline]
    fn new() -> Result<(Self::Output, winit::event_loop::EventLoop<()>), OsError> {
        IridWindowBuilder::default().build()
    }

    #[inline]
    fn id(&self) -> WindowId {
        self.winit_window.id()
    }

    #[inline]
    fn scale_factor(&self) -> f64 {
        self.winit_window.scale_factor()
    }

    #[inline]
    fn request_redraw(&self) {
        self.winit_window.request_redraw()
    }

    //- Position and Size Functions ----------------------------------------------------------------

    #[inline]
    fn inner_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.winit_window.inner_position()
    }

    #[inline]
    fn outer_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        self.winit_window.outer_position()
    }

    #[inline]
    fn set_outer_position<P: Into<Position>>(&self, position: P) {
        self.winit_window.set_outer_position(position)
    }

    #[inline]
    fn inner_size(&self) -> PhysicalSize<u32> {
        self.winit_window.inner_size()
    }

    #[inline]
    fn set_inner_size<S: Into<Size>>(&self, size: S) {
        self.winit_window.set_inner_size(size)
    }

    #[inline]
    fn outer_size(&self) -> PhysicalSize<u32> {
        self.winit_window.outer_size()
    }

    #[inline]
    fn set_min_inner_size<S: Into<Size>>(&self, min_size: Option<S>) {
        self.winit_window.set_min_inner_size(min_size)
    }

    #[inline]
    fn set_max_inner_size<S: Into<Size>>(&self, max_size: Option<S>) {
        self.winit_window.set_max_inner_size(max_size)
    }

    //- Misc. Attribute Functions ------------------------------------------------------------------

    #[inline]
    fn set_title(&self, title: &str) {
        self.winit_window.set_title(title)
    }

    #[inline]
    fn set_visible(&self, visible: bool) {
        self.winit_window.set_visible(visible)
    }

    #[inline]
    fn set_resizable(&self, resizable: bool) {
        self.winit_window.set_resizable(resizable)
    }

    #[inline]
    fn set_minimized(&self, minimized: bool) {
        self.winit_window.set_minimized(minimized)
    }

    #[inline]
    fn set_maximized(&self, maximized: bool) {
        self.winit_window.set_maximized(maximized)
    }

    #[inline]
    fn is_maximized(&self) -> bool {
        self.winit_window.is_maximized()
    }

    #[inline]
    fn set_fullscreen(&self, fullscreen: Option<Fullscreen>) {
        self.winit_window.set_fullscreen(fullscreen)
    }

    #[inline]
    fn fullscreen(&self) -> Option<Fullscreen> {
        self.winit_window.fullscreen()
    }

    #[inline]
    fn set_decorations(&self, decorations: bool) {
        self.winit_window.set_decorations(decorations)
    }

    #[inline]
    fn set_always_on_top(&self, always_on_top: bool) {
        self.winit_window.set_always_on_top(always_on_top)
    }

    #[inline]
    fn set_window_icon(&self, window_icon: Option<Icon>) {
        self.winit_window.set_window_icon(window_icon)
    }

    #[inline]
    fn set_ime_position<P: Into<Position>>(&self, position: P) {
        self.winit_window.set_ime_position(position)
    }

    /*#[inline]
    fn focus_window(&self) {
        self.winit_window.focus_window()
    }*/

    #[inline]
    fn request_user_attention(&self, request_type: Option<UserAttentionType>) {
        self.winit_window.request_user_attention(request_type)
    }

    //- Cursor Functions ---------------------------------------------------------------------------

    #[inline]
    fn set_cursor_icon(&self, cursor: CursorIcon) {
        self.winit_window.set_cursor_icon(cursor)
    }

    #[inline]
    fn set_cursor_position<P: Into<Position>>(&self, position: P) -> Result<(), ExternalError> {
        self.winit_window.set_cursor_position(position)
    }

    #[inline]
    fn set_cursor_grab(&self, grab: bool) -> Result<(), ExternalError> {
        self.winit_window.set_cursor_grab(grab)
    }

    #[inline]
    fn set_cursor_visible(&self, visible: bool) {
        self.winit_window.set_cursor_visible(visible)
    }

    //- Monitor Info Functions ---------------------------------------------------------------------

    #[inline]
    fn drag_window(&self) -> Result<(), ExternalError> {
        self.winit_window.drag_window()
    }

    #[inline]
    fn current_monitor(&self) -> Option<MonitorHandle> {
        self.winit_window.current_monitor()
    }

    /*
    #[inline]
    fn available_monitors(&self) -> Box<(dyn Iterator<Item=winit::monitor::MonitorHandle> +'static)> {
        self.winit_window.window.available_monitors()
            .into_iter()
            .map(|inner| MonitorHandle { inner })
    }
     */

    #[inline]
    fn primary_monitor(&self) -> Option<MonitorHandle> {
        self.winit_window.primary_monitor()
    }

    //- Wrapper Functions --------------------------------------------------------------------------

    #[inline]
    fn expose_inner_window(&self) -> &winit::window::Window {
        &self.winit_window
    }
}
