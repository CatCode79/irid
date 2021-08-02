/**
*
todo link to examples
 */

//= USES ===========================================================================================

use std::collections::HashMap;
use wgpu::ShaderSource;


//= APPLICATION STRUCT =============================================================================

#[derive(Default)]
pub struct Application {
    pub config: std::rc::Rc<crate::app::Config>,
    pub shaders: HashMap<String, Box<wgpu::ShaderSource<'static>>>,
}


impl Application {
    /// Create a new plain App struct.
    // todo: different from ::default
    // todo: after configured the App must be started with start method
    pub fn new(config: crate::app::Config, shaders: HashMap<String, Box<wgpu::ShaderSource<'static>>>) -> Self {
        Self {
            config: std::rc::Rc::new(config),
            shaders,
        }
    }

    /// Starts the event loop.
    /// The event loop is winit based.
    // todo: parameter explication
    pub fn start<L: crate::app::Listener>(self, listener: &'static L) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .build(&event_loop)// TODO check oserror
            .unwrap();

        let mut renderer = crate::renderer::Renderer::new(&window, &self.config);
        let pipeline = crate::renderer::RenderPipeline::new(
            &renderer.device,
            self.shaders.get("shader.wgsl").unwrap()  // TODO: bug! uso hardcoded del filename
        );
        renderer.add_pipeline(pipeline);

        event_loop.run(move |event, _, control_flow| match event {
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
                        self.on_window_keyboard_input(listener, control_flow, device_id, input);
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
            match renderer.redraw() {
                Ok(_) => {},
                Err(error) => match error {
                    // These errors should be resolved by the next frame
                    wgpu::SwapChainError::Timeout | wgpu::SwapChainError::Outdated =>
                        eprintln!("{:?}", error),  // todo

                    // Recreate the swap chain if lost
                    wgpu::SwapChainError::Lost => renderer.refresh_current_size(),

                    // The system is out of memory, we should probably quit
                    wgpu::SwapChainError::OutOfMemory =>
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
        input: winit::event::KeyboardInput
    ) {
        // First call a generic method to manage the key events
        let use_default_behaviour = listener.on_window_keyboard_input(
            device_id,
            input.state,
            input.virtual_keycode.unwrap(),
        );

        // Then check the input's type for default behaviours
        if use_default_behaviour {
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
