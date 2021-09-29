
//= APPLICATION BUILDER ============================================================================

#[derive(Default)]
pub struct ApplicationBuilder<'a> {
    config: crate::app::Config,
    shaders: Option<std::collections::HashMap<String, String>>,
    texture_path: Option<&'a std::path::Path>,
    vertices: Option<&'a [crate::assets::ModelVertex]>,
    indices: Option<&'a [u32]>
}


impl<'a> ApplicationBuilder<'a> {
    /// Create a new plain AppBuilder struct.
    /// After that you can add the necessary fields or build the app and starts it.
    pub fn new(config: crate::app::Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn shaders(mut self, shaders: std::collections::HashMap<String, String>) -> Self {
        self.shaders = Some(shaders);
        self
    }

    pub fn texture_path(mut self, texture_path: &'a std::path::Path) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    pub fn vertices(mut self, vertices: &'a [crate::assets::ModelVertex]) -> Self {
        self.vertices = Some(vertices);
        self
    }

    pub fn indices(mut self, indices: &'a [u32]) -> Self {
        self.indices = Some(indices);
        self
    }

    // TODO: gestirò poi i None a seconda dell'uso che farò con le applicazioni
    pub fn build(self) -> Application<'a> {
        Application {
            config: self.config,
            shaders: self.shaders.unwrap(),
            texture_path: self.texture_path.unwrap(),
            vertices: self.vertices.unwrap(),
            indices: self.indices.unwrap(),
        }
    }
}


//= APPLICATION STRUCT =============================================================================

pub struct Application<'a> {
    config: crate::app::Config,
    shaders: std::collections::HashMap<String, String>,
    texture_path: &'a std::path::Path,
    vertices: &'a [crate::assets::ModelVertex],
    indices: &'a [u32]
}


impl<'a> Application<'a> {
    /// Starts the event loop (the event loop is winit based).
    // todo: parameter explication
    pub fn start<L: crate::app::Listener>(self, listener: &'static L) {
        let mut event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap();  // TODO check OsError

        let mut renderer = crate::renderer::Renderer::new(
            &window,
            self.shaders.get("shader.wgsl").unwrap().clone(),// TODO: controllare poi come togliere il clone (forse con un iteratore)
            self.texture_path,
            self.vertices,
            self.indices
        ).unwrap();

        // I know I should use run method instead of de run_return,
        // but all those static variables are a bore to handle.
        // To remember that the resize is not managed perfectly with run_return:
        // https://docs.rs/winit/0.25.0/winit/platform/run_return/trait.EventLoopExtRunReturn.html
        use winit::platform::run_return::EventLoopExtRunReturn;
        event_loop.run_return(move |event, _, control_flow| match event {
            winit::event::Event::NewEvents(start_cause) => {
                self.on_new_events(listener, start_cause);
            },

            winit::event::Event::WindowEvent {
                event: window_event,
                window_id,
            } => if window_id == window.id() {  // todo multi-monitor support
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
                        self.on_window_drop_file(listener, path);
                    },

                    winit::event::WindowEvent::HoveredFile(path) => {
                        self.on_window_hover_file(listener, path);
                    },

                    winit::event::WindowEvent::HoveredFileCancelled => {
                        self.on_window_hover_file_cancelled(listener);
                    },

                    winit::event::WindowEvent::ReceivedCharacter(c) => {
                        self.on_window_receive_character(listener, c);
                    },

                    winit::event::WindowEvent::Focused(gained_focus) => {
                        self.on_window_focus(listener, gained_focus);
                    },

                    winit::event::WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => if !is_synthetic && input.virtual_keycode.is_some() {
                        self.on_window_keyboard_input(
                            listener, control_flow, device_id, &mut renderer, &input
                        );
                    },

                    winit::event::WindowEvent::ModifiersChanged(state) => {
                        self.on_window_modifiers_change(listener, state);
                    },

                    winit::event::WindowEvent::CursorMoved {
                        device_id,
                        position ,
                        ..
                    } => {
                        self.on_window_cursor_move(listener, device_id, position);
                    },

                    winit::event::WindowEvent::CursorEntered { device_id } => {
                        self.on_window_cursor_enter(listener, device_id);
                    },

                    winit::event::WindowEvent::CursorLeft { device_id } => {
                        self.on_window_cursor_left(listener, device_id);
                    },

                    winit::event::WindowEvent::MouseWheel {
                        device_id,
                        delta,
                        phase,
                        ..
                    } => {
                        self.on_window_mouse_wheel(listener, device_id, delta, phase);
                    },

                    winit::event::WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        self.on_window_mouse_input(listener, device_id, state, button);
                    },

                    winit::event::WindowEvent::TouchpadPressure {
                        device_id,
                        pressure,
                        stage
                    } => {
                        self.on_window_touchpad_pressure(listener, device_id, pressure, stage);
                    },

                    winit::event::WindowEvent::AxisMotion {
                        device_id,
                        axis,
                        value
                    } => {
                        self.on_window_axis_motion(listener, device_id, axis, value);
                    },

                    winit::event::WindowEvent::Touch(touch) => {
                        self.on_window_touch(listener, touch);
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

                    winit::event::WindowEvent::ThemeChanged(theme) => {
                        self.on_window_theme_change(listener, theme);
                    },
                }
            },

            winit::event::Event::DeviceEvent { device_id: _device_id, event: ref _device_event } => {
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
                self.on_redraw(listener, &mut renderer, control_flow);
            },

            winit::event::Event::RedrawRequested(window_id) => {
                self.on_redraw_request(listener, &window_id);
            },

            winit::event::Event::RedrawEventsCleared => {
                self.on_redraw_clear(listener);
            },

            winit::event::Event::LoopDestroyed => {
                self.on_destroy(listener);
            },
        });
    }

    //- Generic Events Methods ---------------------------------------------------------------------

    fn on_new_events<L: crate::app::Listener>(
        &self,
        listener: &L,
        start_cause: winit::event::StartCause
    ) {
        /*let use_default_behaviour: bool =*/ listener.on_new_events(start_cause);
    }

    fn on_user_event<L: crate::app::Listener>(&self, listener: &L, event: &()) {
        /*let use_default_behaviour: bool =*/ listener.on_user_event(event);
    }

    fn on_suspend<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_suspend();
    }

    fn on_resume<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_resume();
    }

    // This method is probably one of the few that must always be inline.
    #[inline(always)]
    fn on_redraw<L: crate::app::Listener>(
        &self,
        listener: &L,
        renderer: &mut crate::renderer::Renderer,
        control_flow: &mut winit::event_loop::ControlFlow
    ) {
        let use_default_behaviour: bool = listener.on_redraw();
        if use_default_behaviour {
            match renderer.redraw(&self.config) {
                Ok(_) => {},
                Err(error) => match error {
                    // These errors should be resolved by the next frame
                    wgpu::SurfaceError::Timeout | wgpu::SurfaceError::Outdated =>
                        eprintln!("{:?}", error),  // todo

                    // Recreate the swap chain if lost
                    wgpu::SurfaceError::Lost => renderer.refresh_current_size(),

                    // The system is out of memory, we should probably quit
                    wgpu::SurfaceError::OutOfMemory =>
                        *control_flow = winit::event_loop::ControlFlow::Exit,
                }
            }
        }
    }

    fn on_redraw_request<L: crate::app::Listener>(
        &self,
        listener: &L,
        window_id: &winit::window::WindowId
    ) {
        /*let use_default_behaviour: bool =*/ listener.on_redraw_request(window_id);
    }

    fn on_redraw_clear<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_redraw_clear();
    }

    fn on_destroy<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour: bool =*/ listener.on_destroy();
    }

    //- Window Events Methods ----------------------------------------------------------------------

    fn on_window_resize<L: crate::app::Listener>(
        &self,
        listener: &L,
        renderer: &mut crate::renderer::Renderer,
        physical_size: winit::dpi::PhysicalSize<u32>
    ) {
        let use_default_behaviour = listener.on_window_resize(physical_size);
        if use_default_behaviour {
            // todo Pensare se il metodo resize lo devo spostare qui, in window o tenerlo così
            renderer.resize(physical_size);
        }
    }

    fn on_window_move<L: crate::app::Listener>(
        &self,
        listener: &L,
        physical_position: winit::dpi::PhysicalPosition<i32>
    ) {
        /*let use_default_behaviour =*/ listener.on_window_move(physical_position);
    }

    fn on_window_close<L: crate::app::Listener>(
        &self,
        listener: &L,
        control_flow: &mut winit::event_loop::ControlFlow
    ) {
        let use_default_behaviour = listener.on_window_close();
        if use_default_behaviour {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    }

    fn on_window_destroy<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour =*/ listener.on_window_destroy();
    }

    fn on_window_drop_file<L: crate::app::Listener>(
        &self,
        listener: &L,
        path: std::path::PathBuf
    ) {
        /*let use_default_behaviour =*/ listener.on_window_drop_file(path);
    }

    fn on_window_hover_file<L: crate::app::Listener>(
        &self,
        listener: &L,
        path: std::path::PathBuf
    ) {
        /*let use_default_behaviour =*/ listener.on_window_hover_file(path);
    }

    fn on_window_hover_file_cancelled<L: crate::app::Listener>(&self, listener: &L) {
        /*let use_default_behaviour =*/ listener.on_window_hover_file_cancelled();
    }

    fn on_window_receive_character<L: crate::app::Listener>(&self, listener: &L, c: char) {
        /*let use_default_behaviour =*/ listener.on_window_receive_character(c);
    }

    fn on_window_focus<L: crate::app::Listener>(&self, listener: &L, gained_focus: bool) {
        /*let use_default_behaviour =*/ listener.on_window_focus(gained_focus);
    }

    // Triggered then an user press a key upon this active window.
    // Currently it's preferred to avoid the synthetic event's keys because they
    // works only on Windows and X11 OS; also because the API it's easier and less
    // breakable if used on listener's events.
    // Also then the window is restored (from minimize state) where are
    // input.virtual_keycode KeyboardInput events equals to None.
    fn on_window_keyboard_input<L: crate::app::Listener>(
        &self,
        listener: &L,
        control_flow: &mut winit::event_loop::ControlFlow,
        device_id: winit::event::DeviceId,
        renderer: &mut crate::renderer::Renderer,
        input: &winit::event::KeyboardInput
    ) {
        // First call a generic method to manage the key events
        let use_default_behaviour = listener.on_window_keyboard_input(
            device_id,
            input.state,
            input.virtual_keycode.unwrap(),
        );

        // Then check the input's type for default behaviours
        if use_default_behaviour {
            // Check the camera controller
            renderer.process_camera_events(input);  // TODO: migliorare il sistema, per ora lo sto facendo solo funzionare

            match input {
                // Esc key pressed
                winit::event::KeyboardInput {
                    state: winit::event::ElementState::Pressed,
                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,

                // The other keys are ignored
                _ => {}
            }
        }
    }

    fn on_window_modifiers_change<L: crate::app::Listener>(
        &self,
        listener: &L,
        state: winit::event::ModifiersState
    ) {
        /*let use_default_behaviour =*/ listener.on_window_modifiers_change(state);
    }

    fn on_window_cursor_move<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId,
        position: winit::dpi::PhysicalPosition<f64>
    ) {
        /*let use_default_behaviour =*/ listener.on_window_cursor_move(device_id, position);
    }

    fn on_window_cursor_enter<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId
    ) {
        /*let use_default_behaviour =*/ listener.on_window_cursor_enter(device_id);
    }

    fn on_window_cursor_left<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId
    ) {
        /*let use_default_behaviour =*/ listener.on_window_cursor_left(device_id);
    }

    fn on_window_mouse_wheel<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId,
        delta: winit::event::MouseScrollDelta,
        phase: winit::event::TouchPhase
    ) {
        /*let use_default_behaviour =*/ listener.on_window_mouse_wheel(device_id, delta, phase);
    }

    fn on_window_mouse_input<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId,
        state: winit::event::ElementState,
        button: winit::event::MouseButton
    ) {
        /*let use_default_behaviour =*/ listener.on_window_mouse_input(device_id, state, button);
    }

    fn on_window_touchpad_pressure<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId,
        pressure: f32,
        stage: i64
    ) {
        /*let use_default_behaviour =*/ listener.on_window_touchpad_pressure(device_id, pressure, stage);
    }

    fn on_window_axis_motion<L: crate::app::Listener>(
        &self,
        listener: &L,
        device_id: winit::event::DeviceId,
        axis: u32,
        value: f64
    ) {
        /*let use_default_behaviour =*/ listener.on_window_axis_motion(device_id, axis, value);
    }

    fn on_window_touch<L: crate::app::Listener>(
        &self,
        listener: &L,
        touch: winit::event::Touch
    ) {
        /*let use_default_behaviour =*/ listener.on_window_touch(touch);
    }

    fn on_window_scale_change<L: crate::app::Listener>(
        &self,
        listener: &L,
        renderer: &mut crate::renderer::Renderer,
        scale_factor: f64,
        new_inner_size: &mut winit::dpi::PhysicalSize<u32>
    ) {
        let use_default_behaviour = listener.on_window_scale_change(
            scale_factor,
            new_inner_size  // TODO prob qui e giù devo passarla senza deref
        );

        if use_default_behaviour {
            renderer.resize(*new_inner_size);
        }
    }

    fn on_window_theme_change<L: crate::app::Listener>(
        &self,
        listener: &L,
        // theme is copied because an enumeration
        theme: winit::window::Theme
    ) {
        /*let use_default_behaviour =*/ listener.on_window_theme_change(theme);
    }
}
