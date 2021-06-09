/**
 *
 todo link to example
 */

//= APP STRUCT =====================================================================================

pub struct App {
}


impl App {

    /**
     * Create a new plain App struct.
     todo: different from ::default
     todo: after configured the App must be started with start method
     */
    pub fn new() -> App {
        env_logger::init();

        App {

        }
    }

    /**
     * Starts the event loop.
     * The event loop is winit based.
     todo: parameter explication
     */
    pub fn start<WL: crate::window::WindowListener>(self, window_listener: &'static WL) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
        let _renderer = crate::renderer::Renderer::new(&window);

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {  // todo multi-monitor support

                // Triggered then an user try to close the window
                winit::event::WindowEvent::CloseRequested => {
                    let use_default_behaviour = window_listener.on_close_requested();
                    if use_default_behaviour {
                        self.on_close_requested_default_behaviour(control_flow)
                    }
                },

                // Triggered then an user press a key upon this active window
                winit::event::WindowEvent::KeyboardInput {
                    device_id,
                    input,
                    is_synthetic,
                } => {
                    // Currently it's preferred to avoid the synthetic event's keys because they
                    // works only on Windows and X11 OS; also because the API it's easier and less
                    // breakable if used on listener's events.
                    // Also then the window is restored (from minimize state) where are
                    // input.virtual_keycode KeyboardInput events equals to None.
                    //debug_assert!(!*is_synthetic, "Why I have to use ya, is_synthetic? {:?}", event);
                    //debug_assert!(input.virtual_keycode.is_some(), "Why I have to use ya, virtual_keycode.is_none()? {:?}", event);
                    if !*is_synthetic && input.virtual_keycode.is_some() {
                        // First call a generic method to manage the key events
                        let use_default_behaviour = window_listener.on_keyboard_input(
                            &device_id as &crate::window::DeviceId,
                            input.state as crate::window::ElementState,
                            input.virtual_keycode.unwrap() as crate::window::VirtualKeycode,
                        );

                        if use_default_behaviour {
                            // Then check the input's type for default behaviours
                            match input {

                                // Esc key pressed
                                winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                    ..
                                } => self.on_close_requested_default_behaviour(control_flow),

                                // The other keys are ignored
                                _ => {}
                            }
                        }
                    }
                },

                // The other window's events are ignored
                _ => {},
            },
            _ => {}
        });
    }

    //- Default Events Methods ---------------------------------------------------------------------

    fn on_close_requested_default_behaviour(&self, control_flow: &mut winit::event_loop::ControlFlow) {
        *control_flow = winit::event_loop::ControlFlow::Exit;
    }
}
