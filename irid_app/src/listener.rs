//= LISTENER =======================================================================================

/// The Listeners to be implemented for the game logic.
///
/// For more information about the events see
/// [`enum Event`](winit::event::Event) and [`enum WindowEvent`](winit::event::WindowEvent).
pub trait Listener {
    /// Emitted when new events arrive from the OS to be processed.
    ///
    /// This event type is useful as a place to put code that should be done before you start
    /// processing events, such as updating frame timing information for benchmarking or checking
    /// the [`StartCause`][winit::event::StartCause] to see if a timer set by
    /// [`ControlFlow::WaitUntil`](winit::event_loop::ControlFlow::WaitUntil) has elapsed.
    #[allow(unused_variables)]
    fn on_new_events(&self, start_cause: winit::event::StartCause) -> bool {
        true
    }

    /// Emitted when an event is sent from
    /// [`EventLoopProxy::send_event`](winit::event_loop::EventLoopProxy::send_event).
    #[allow(unused_variables)]
    fn on_user_event<T>(&self, event: &T) -> bool {
        true
    }

    /// Emitted when the application has been suspended.
    fn on_suspend(&self) -> bool {
        true
    }

    /// Emitted when the application has been resumed.
    fn on_resume(&self) -> bool {
        true
    }

    /// Emitted when all of the event loop's input events have been processed and redraw
    /// processing is about to begin.
    ///
    /// This event is useful as a place to put your code that should be run after all
    /// state-changing events have been handled and you want to do stuff (updating state,
    /// performing calculations, etc) that happens as the "main body" of your event loop.
    /// If your program only draws graphics when something changes, it's usually better
    /// to do it in response to [`Event::RedrawRequested`](winit::event::Event::RedrawRequested),
    /// which gets emitted immediately after this event.
    ///
    /// Programs that draw graphics continuously, like most games, can render here
    /// unconditionally for simplicity.
    fn on_redraw(&self) -> bool;

    /// Emitted after `on_redraw_begin` when a window should be redrawn.
    ///
    /// This gets triggered in two scenarios:
    /// - The OS has performed an operation that's invalidated the window's contents (such as
    ///   resizing the window).
    /// - The application has explicitly requested a redraw via
    ///   [`Window::request_redraw`](winit::window::Window::request_redraw).
    ///
    /// During each iteration of the event loop, Winit will aggregate duplicate redraw requests
    /// into a single event, to help avoid duplicating rendering work.
    ///
    /// Mainly of interest to applications with mostly-static graphics that avoid redrawing unless
    /// something changes, like most non-game GUIs.
    #[allow(unused_variables)]
    fn on_redraw_request(&self, window_id: &winit::window::WindowId) -> bool {
        true
    }

    /// Emitted after all `on_redraw_request` events have been processed and control flow is about
    /// to be taken away from the program. If there are no `on_redraw_request` events, it is emitted
    /// immediately after `on_redraw`.
    ///
    /// This event is useful for doing any cleanup or bookkeeping work after all the rendering
    /// tasks have been completed.
    fn on_redraw_clear(&self) -> bool {
        true
    }

    /// Emitted when the event loop is being shut down.
    ///
    /// This is irreversible - if this event is emitted, it is guaranteed to be the last event that
    /// gets emitted. You generally want to treat this as an "do on quit" event.
    fn on_destroy(&self) -> bool {
        true
    }

    //- Window Events ------------------------------------------------------------------------------

    /// The size of the window has changed.
    ///
    /// * `new_size` - Contains the client area's new dimensions.
    #[allow(unused_variables)]
    fn on_window_resize(&self, new_size: winit::dpi::PhysicalSize<u32>) -> bool {
        true
    }

    /// The position of the window has changed.
    ///
    /// * `physical_position` - Contains the window's new position.
    #[allow(unused_variables)]
    fn on_window_move(&self, physical_position: winit::dpi::PhysicalPosition<i32>) -> bool {
        true
    }

    /// Triggered then an user try to close the window.
    fn on_window_close(&self) -> bool {
        true
    }

    /// The window has been destroyed.
    fn on_window_destroy(&self) -> bool {
        true
    }

    /// A file has been dropped into the window.
    ///
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    #[allow(unused_variables)]
    fn on_window_drop_file(&self, path: std::path::PathBuf) -> bool {
        true
    }

    /// A file is being hovered over the window.
    ///
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    #[allow(unused_variables)]
    fn on_window_hover_file(&self, path: std::path::PathBuf) -> bool {
        true
    }

    /// A file was hovered, but has exited the window.
    ///
    /// There will be a single `on_window_hover_file_cancelled` event triggered even if multiple
    /// files were hovered.
    #[allow(unused_variables)]
    fn on_window_hover_file_cancelled(&self) -> bool {
        true
    }

    /// The window received a unicode character.
    #[allow(unused_variables)]
    fn on_window_receive_character(&self, c: char) -> bool {
        true
    }

    /// The window gained or lost the focus.
    ///
    /// * `gained_focus` - True if the window has gained focus, and false if it has lost focus.
    #[allow(unused_variables)]
    fn on_window_focus(&self, gained_focus: bool) -> bool {
        true
    }

    /// An event from the keyboard has been received.
    #[allow(unused_variables)]
    fn on_window_keyboard_input(
        &self,
        device_id: winit::event::DeviceId,
        state: winit::event::ElementState,
        virtual_keycode: winit::event::VirtualKeyCode,
    ) -> bool {
        true
    }

    /// The keyboard modifiers have changed.
    ///
    /// Platform-specific behavior:
    /// - **Web**: This API is currently unimplemented on the web. This isn't by design - it's an
    ///   issue, and it should get fixed - but it's the current state of the API.
    #[allow(unused_variables)]
    fn on_window_modifiers_change(&self, state: winit::event::ModifiersState) -> bool {
        true
    }

    /// The cursor has moved on the window.
    ///
    /// * `position` - (x,y) coords in pixels relative to the top-left corner of the window.
    /// Because the range of this data is limited by the display area and it may have been
    /// transformed by the OS to implement effects such as cursor acceleration,
    /// it should not be used to implement non-cursor-like interactions such as 3D camer control.
    #[allow(unused_variables)]
    fn on_window_cursor_move(
        &self,
        device_id: winit::event::DeviceId,
        position: winit::dpi::PhysicalPosition<f64>,
    ) -> bool {
        true
    }

    /// The cursor has entered the window.
    #[allow(unused_variables)]
    fn on_window_cursor_enter(&self, device_id: winit::event::DeviceId) -> bool {
        true
    }

    /// The cursor has left the window.
    #[allow(unused_variables)]
    fn on_window_cursor_left(&self, device_id: winit::event::DeviceId) -> bool {
        true
    }

    /// A mouse wheel movement or touchpad scroll occurred.
    #[allow(unused_variables)]
    fn on_window_mouse_wheel(
        &self,
        device_id: winit::event::DeviceId,
        delta: winit::event::MouseScrollDelta,
        phase: winit::event::TouchPhase,
    ) -> bool {
        true
    }

    /// A mouse button press has been received.
    #[allow(unused_variables)]
    fn on_window_mouse_input(
        &self,
        device_id: winit::event::DeviceId,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) -> bool {
        true
    }

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    ///
    /// * `pressure` - Value between 0 and 1 representing how hard the touchpad is being pressed.
    /// * `stage` - Integer representing the click level.
    #[allow(unused_variables)]
    fn on_window_touchpad_pressure(
        &self,
        device_id: winit::event::DeviceId,
        pressure: f32,
        stage: i64,
    ) -> bool {
        true
    }

    /// Motion on some analog axis. May report data redundant to other, more specific events.
    #[allow(unused_variables)]
    fn on_window_axis_motion(
        &self,
        device_id: winit::event::DeviceId,
        axis: winit::event::AxisId,
        value: f64,
    ) -> bool {
        true
    }

    /// Touch event has been received.
    #[allow(unused_variables)]
    fn on_window_touch(&self, touch: winit::event::Touch) -> bool {
        true
    }

    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different scale factor.
    ///
    /// After this event callback has been processed, the window will be resized to whatever value
    /// is pointed to by the `new_inner_size` reference. By default, this will contain the size
    /// suggested by the OS, but it can be changed to any value.
    #[allow(unused_variables)]
    fn on_window_scale_change(
        &self,
        scale_factor: f64,
        new_inner_size: &mut winit::dpi::PhysicalSize<u32>,
    ) -> bool {
        true
    }

    /// The system window theme has changed.
    ///
    /// Applications might wish to react to this to change the theme of the content of the window
    /// when the system changes the window theme.
    ///
    /// At the moment this is only supported on Windows.
    #[allow(unused_variables)]
    fn on_window_theme_change(&self, theme: winit::window::Theme) -> bool {
        true
    }

    //- Device Events ------------------------------------------------------------------------------
    // TODO: I don't know exactly why I have to use those events, but maybe for the joypad...
}
