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
    pub fn start<L: crate::window::Listener>(self, listener: &'static L) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();
        let mut renderer = crate::renderer::Renderer::new(&window);

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::NewEvents(start_cause) => {
                self.on_new_events(listener, start_cause);
            },

            winit::event::Event::WindowEvent {
                event: ref window_event,
                ref window_id,
            } => if *window_id == window.id() {  // todo multi-monitor support
                match window_event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        self.on_window_resize(listener, &mut renderer, physical_size);
                    },

                    winit::event::WindowEvent::Moved(physical_position) => {
                        self.on_window_move(listener, physical_position);
                    },

                    winit::event::WindowEvent::CloseRequested => {
                        self.on_window_close(listener, control_flow);
                    },

                    winit::event::WindowEvent::Destroyed => {
                        self.on_window_destroy(listener);
                    },

                    winit::event::WindowEvent::DroppedFile(path) => {
                        self.on_window_dropped_file(listener, path);
                    },

                    winit::event::WindowEvent::HoveredFile(path) => {
                        self.on_hovered_file(listener, path);
                    },

                    winit::event::WindowEvent::HoveredFileCancelled => {
                        self.on_hovered_file_cancelled(listener, path);
                    },

                    winit::event::WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => if !*is_synthetic && input.virtual_keycode.is_some() {
                        self.on_window_keyboard_input(listener, control_flow, device_id, input);
                    },

                    // The window's scale factor has changed.
                    winit::event::WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size
                    } => {
                        self.on_window_scale_change(
                            listener, &mut renderer, scale_factor, new_inner_size
                        );
                    },

                    _ => {},  // todo error, devo averli inseriti tutti
                }
            },

            winit::event::Event::DeviceEvent {device_id: _device_id, event: ref _device_event} => {
                // TODO per ora non vengono utilizzati, li ignoro
            },

            winit::event::Event::UserEvent(event) => {
                self.on_user_event(listener, &event);
            },

            winit::event::Event::Suspended => {
                self.on_suspend(listener);
            },

            winit::event::Event::Resumed => {
                self.on_resume(listener);
            },

            winit::event::Event::MainEventsCleared => {
                self.on_redraw_begin(listener);
            },

            winit::event::Event::RedrawRequested(window_id) => {
                self.on_redraw_request(listener, &(window_id as crate::window::WindowId));
            },

            winit::event::Event::RedrawEventsCleared => {
                self.on_redraw_end(listener);
            },

            winit::event::Event::LoopDestroyed => {
                self.on_destroy(listener);
            },
        });
    }

    //- Generic Events Methods ---------------------------------------------------------------------

    fn on_new_events<L: crate::window::Listener>(
        &self,
        listener: &L,
        start_cause: crate::window::StartCause
    ) {
        /*let use_default_behaviour: bool =*/ listener.on_new_events(start_cause);
    }

    fn on_user_event<L: crate::window::Listener>(&self, listener: &L, event: &()) {
        /*let use_default_behaviour: bool =*/ listener.on_user_event(event);
    }

    fn on_suspend<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_suspend();
    }

    fn on_resume<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_resume();
    }

    // This method is probably one of the few that must always be inline.
    #[inline(always)]
    fn on_redraw_begin<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_redraw_begin();
    }

    fn on_redraw_request<L: crate::window::Listener>(
        &self,
        listener: &L,
        window_id: &crate::window::WindowId
    ) {
        /*let use_default_behaviour: bool =*/ listener.on_redraw_request(window_id);
    }

    fn on_redraw_end<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_redraw_end();
    }

    fn on_destroy<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_destroy();
    }

    //- Window Events Methods ----------------------------------------------------------------------

    fn on_window_resize<L: crate::window::Listener>(
        &self,
        listener: &L,
        renderer: &mut crate::renderer::Renderer,
        physical_size: &winit::dpi::PhysicalSize<u32>
    ) {
        let use_default_behaviour = listener.on_window_resize(*physical_size);
        if use_default_behaviour {
            // todo Pensare se il metodo resize lo devo spostare qui, in window o tenerlo così
            renderer.resize(&physical_size);
        }
    }

    fn on_window_move<L: crate::window::Listener>(
        &self,
        listener: &L,
        physical_position: &winit::dpi::PhysicalPosition<i32>
    ) {
        /*let use_default_behaviour =*/ listener.on_window_move(*physical_position);
    }

    fn on_window_close<L: crate::window::Listener>(
        &self,
        listener: &L,
        control_flow: &mut winit::event_loop::ControlFlow
    ) {
        let use_default_behaviour = listener.on_window_close();
        if use_default_behaviour {
            App::close_window_default_behaviour(control_flow);
        }
    }

    fn on_window_destroy<L: crate::window::Listener>(&self, listener: &L) {
        /*let use_default_behaviour =*/ listener.on_window_destroy();
    }

    // Triggered then an user press a key upon this active window.
    // Currently it's preferred to avoid the synthetic event's keys because they
    // works only on Windows and X11 OS; also because the API it's easier and less
    // breakable if used on listener's events.
    // Also then the window is restored (from minimize state) where are
    // input.virtual_keycode KeyboardInput events equals to None.
    fn on_window_keyboard_input<L: crate::window::Listener>(
        &self,
        listener: &L,
        control_flow: &mut winit::event_loop::ControlFlow,
        device_id: &winit::event::DeviceId,
        input: &winit::event::KeyboardInput
    ) {
        // First call a generic method to manage the key events
        let use_default_behaviour = listener.on_window_keyboard_input(
            &device_id as &crate::window::DeviceId,
            input.state as crate::window::ElementState,
            input.virtual_keycode.unwrap() as crate::window::VirtualKeyCode,
        );

        // Then check the input's type for default behaviours
        if use_default_behaviour {
            match input {
                // Esc key pressed
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed,
                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                    ..
                } => App::close_window_default_behaviour(control_flow),

                // The other keys are ignored
                _ => {}
            }
        }
    }

    fn on_window_scale_change<L: crate::window::Listener>(
        &self,
        listener: &L,
        renderer: &mut crate::renderer::Renderer,
        scale_factor: &f64,
        new_inner_size: &&mut winit::dpi::PhysicalSize<u32>
    ) {
        let use_default_behaviour = listener.on_window_scale_change(
            *scale_factor,
            **new_inner_size
        );

        if use_default_behaviour {
            renderer.resize(*new_inner_size);
        }
    }

    //- Default Behaviours -------------------------------------------------------------------------

    fn close_window_default_behaviour(control_flow: &mut winit::event_loop::ControlFlow) {
        *control_flow = winit::event_loop::ControlFlow::Exit;
    }
}
