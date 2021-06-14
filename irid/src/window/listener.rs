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
    ///
    #[allow(unused_variables)]
    fn on_resized(&self, new_size: crate::window::PhysicalSize) -> bool {
        true
    }

    ///
    fn on_close_requested(&self) -> bool {
        true
    }

    ///
    #[allow(unused_variables)]
    fn on_keyboard_input(
        &self,
        device_id: &crate::window::DeviceId,
        state: crate::window::ElementState,
        virtual_keycode: crate::window::VirtualKeyCode
    ) -> bool {
        true
    }

    /// The window's scale factor has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's scale factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different scale factor.
    #[allow(unused_variables)]
    fn on_scale_factor_changed(
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
