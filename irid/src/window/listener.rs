/// The Listeners to be implemented for the game logic.
///
/// todo: usage's example

//= SUPER TRAITS ===================================================================================

/// Super trait for all the listeners to use as alias.
pub trait Listener: EventListener + WindowListener {}
impl<T: EventListener + WindowListener> Listener for T {}


//= LISTENER TRAITS ================================================================================

/// Listener for a generic event.
///
/// For more information see [`enum Event`](winit::event::Event).
pub trait EventListener {
    /// Emitted when new events arrive from the OS to be processed.
    #[allow(unused_variables)]
    fn on_new_events(&self, start_cause: crate::window::event::StartCause) -> bool {
        true
    }

    /// Emitted when an event is sent from [`EventLoopProxy::send_event`](winit::event_loop::EventLoopProxy::send_event).
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

    /// Emitted when all of the event loop's input events have been processed and redraw processing
    /// is about to begin.
    ///
    /// This event is useful as a place to put your code that should be run after all
    /// state-changing events have been handled and you want to do stuff (updating state,
    /// performing calculations, etc) that happens as the "main body" of your event loop.
    ///
    /// Programs that draw graphics continuously, like most games, can render here unconditionally
    /// for simplicity.
    fn on_redraw_begin(&self) -> bool;

    /// Emitted after `on_redraw_begin` when a window should be redrawn.
    ///
    /// Mainly of interest to applications with mostly-static graphics that avoid redrawing unless
    /// something changes, like most non-game GUIs.
    #[allow(unused_variables)]
    fn on_redraw_request(&self, window_id: &crate::window::WindowId) -> bool {
        true
    }

    /// Emitted after all `on_redraw_request` events have been processed and control flow is about
    /// to be taken away from the program. If there are no `on_redraw_request` events, it is emitted
    /// immediately after `on_redraw_begin`.
    ///
    /// This event is useful for doing any cleanup or bookkeeping work after all the rendering
    /// tasks have been completed.
    fn on_redraw_end(&self) -> bool {
        true
    }

    /// Emitted when the event loop is being shut down.
    ///
    /// This is irreversible - if this event is emitted, it is guaranteed to be the last event that
    /// gets emitted. You generally want to treat this as an "do on quit" event.
    fn on_destroy(&self) -> bool {
        true
    }
}


/// Listener for a window event.
///
/// For more information see [`enum Event`](winit::event::Event).
pub trait WindowListener {
    /// The size of the window has changed.
    #[allow(unused_variables)]
    fn on_window_resize(&self, new_size: crate::window::PhysicalSize) -> bool {
        true
    }

    /// The position of the window has changed. Contains the window's new position.
    #[allow(unused_variables)]
    fn on_window_move(&self, physical_position: crate::window::PhysicalPosition) -> bool {
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
    #[allow(unused_variables)]
    fn on_window_drop_file(&self, path: &std::path::PathBuf) -> bool {
        true
    }

    /// A file is being hovered over the window.
    #[allow(unused_variables)]
    fn on_window_hover_file(&self, path: &std::path::PathBuf) -> bool {
        true
    }

    /// A file was hovered, but has exited the window.
    #[allow(unused_variables)]
    fn on_window_hover_file_cancelled(&self) -> bool {
        true
    }

    /// The window received a unicode character.
    #[allow(unused_variables)]
    fn on_window_receive_character(c: char) -> bool  {
        true
    }

    /// The window gained or lost the focus.
    #[allow(unused_variables)]
    fn on_window_focus(gained_focus: bool) -> bool {
        true
    }

    /// An event from the keyboard has been received.
    #[allow(unused_variables)]
    fn on_window_keyboard_input(
        &self,
        device_id: &crate::window::DeviceId,
        state: crate::window::ElementState,
        virtual_keycode: crate::window::VirtualKeyCode
    ) -> bool {
        true
    }

    /// The keyboard modifiers have changed.
    #[allow(unused_variables)]
    fn on_window_modifiers_change(state: winit::event::ModifiersState) -> bool {
        true
    }

    /// The cursor has moved on the window.
    #[allow(unused_variables)]
    fn on_window_cursor_move(
        &self,
        device_id: crate::window::DeviceId,
        /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range
        /// of this data is limited by the display area and it may have been transformed by the OS
        /// to implement effects such as cursor acceleration, it should not be used to implement
        /// non-cursor-like interactions such as 3D camera control.
        position:crate::window::PhysicalPosition
    ) -> bool {
        true
    }

    /// The cursor has entered the window.
    #[allow(unused_variables)]
    fn on_window_cursor_enter(
        &self,
        device_id: crate::window::DeviceId
    ) -> bool {
        true
    }

    /// The cursor has left the window.
    #[allow(unused_variables)]
    fn on_window_cursor_exit(
        &self,
        device_id: crate::window::DeviceId
    ) -> bool {
        true
    }

    /// A mouse wheel movement or touchpad scroll occurred.
    #[allow(unused_variables)]
    fn on_window_mouse_wheel(
        &self,
        device_id: crate::window::DeviceId,
        delta: crate::window::MouseScrollDelta,
        phase: crate::window::TouchPhase
    ) -> bool {
        true
    }

    /// A mouse button press has been received.
    #[allow(unused_variables)]
    fn on_window_mouse_input(
        &self,
        device_id: crate::window::DeviceId,
        delta: crate::window::ElementState,
        phase: crate::window::MouseButton
    ) -> bool {
        true
    }

    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    #[allow(unused_variables)]
    fn on_window_touchpad_pressure(
        &self,
        device_id: crate::window::DeviceId,
        /// Value between 0 and 1 representing how hard the touchpad is being pressed.
        pressure: f32,
        /// Integer representing the click level.
        stage: i64
    ) -> bool {
        true
    }

    /// Motion on some analog axis. May report data redundant to other, more specific events.
    #[allow(unused_variables)]
    fn on_window_axis_motion(
        &self,
        device_id: crate::window::DeviceId,
        axis: crate::window::AxisId,
        value: f64
    ) -> bool {
        true
    }

    /// Touch event has been received.
    #[allow(unused_variables)]
    fn on_window_touch(
        &self,
        Touch
    ) -> bool {
        true
    }

    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    /// * Changing the display's resolution.
    /// * Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different scale factor.
    #[allow(unused_variables)]
    fn on_window_scale_change(
        &self,
        scale_factor: f64,
        new_inner_size: crate::window::PhysicalSize
    ) -> bool {
        true
    }
}


/*
pub trait DeviceListener {

}
*/
