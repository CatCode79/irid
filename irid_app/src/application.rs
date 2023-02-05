//= USES =====================================================================

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::PathBuf,
};

use irid_assets::Vertex;
use irid_render::{PerspectiveCamera, Renderer, RendererConfig, RendererError};
use winit::event::{
    DeviceId, ElementState, Event, KeyboardInput, ModifiersState, MouseButton, MouseScrollDelta,
    StartCause, Touch, TouchPhase, VirtualKeyCode, WindowEvent,
};

use crate::{IridWindowConfig, Listener};

//= APPLICATION BUILDER ======================================================

/// Build a new [Application] with wanted values.
#[derive(Clone, Debug, Default)]
pub struct ApplicationBuilder<'a, L: Listener, V: Vertex> {
    listener: L,
    window_config: Option<IridWindowConfig>,
    renderer_config: Option<RendererConfig<'a, PerspectiveCamera, &'a str, &'a str, V, u16>>,
}

impl<'a, L, V> ApplicationBuilder<'a, L, V>
where
    L: Listener,
    V: Vertex + bytemuck::Pod,
{
    //- Constructors ---------------------------------------------------------

    ///
    pub fn new(listener: L) -> Self {
        Self {
            listener,
            window_config: None,
            renderer_config: None,
        }
    }

    //- Setters --------------------------------------------------------------

    ///
    #[inline]
    pub fn with_listener(mut self, listener: L) -> Self {
        self.listener = listener;
        self
    }

    ///
    #[inline]
    pub fn with_window_config(mut self, window_config: IridWindowConfig) -> Self {
        self.window_config = Some(window_config);
        self
    }

    ///
    #[inline]
    pub fn with_renderer_config(
        mut self,
        renderer_config: RendererConfig<'a, PerspectiveCamera, &'a str, &'a str, V, u16>,
    ) -> Self {
        self.renderer_config = Some(renderer_config);
        self
    }

    //- Build ----------------------------------------------------------------

    /// Build a new [Application] with given values.
    pub fn build(self) -> Application<'a, L, V> {
        Application {
            listener: self.listener,
            window_config: self.window_config.unwrap_or_else(IridWindowConfig::new),
            renderer_config: self.renderer_config.unwrap_or_else(RendererConfig::new),
        }
    }
}

//= APPLICATION ==============================================================

/// Manages the whole game setup and logic.
#[derive(Debug)]
pub struct Application<'a, L: Listener, V: Vertex> {
    listener: L,
    window_config: IridWindowConfig,
    renderer_config: RendererConfig<'a, PerspectiveCamera, &'a str, &'a str, V, u16>,
}

impl<'a, L, V> Application<'a, L, V>
where
    L: Listener,
    V: Vertex + bytemuck::Pod,
{
    /// Starts the
    /// [event loop](https://docs.rs/winit/0.25.0/winit/event_loop/struct.EventLoop.html).
    ///
    /// # Notes:
    ///
    /// The event loop uses the winit
    /// [run_return](https://docs.rs/winit/0.25.0/winit/platform/run_return/trait.EventLoopExtRunReturn.html#tymethod.run_return)
    /// method, which has some caveats.
    pub fn start(self) -> Result<(), ApplicationError> {
        let mut event_loop = winit::event_loop::EventLoop::new();
        let window = self
            .window_config
            .to_owned()
            .build(&event_loop)
            .map_err(|e| ApplicationError::WindowOsError { source: e })?;

        let renderer = &mut self
            .renderer_config
            .build(window.expose_inner_window())
            .map_err(|e| ApplicationError::RendererError { source: e })?;

        use winit::platform::run_return::EventLoopExtRunReturn;
        event_loop.run_return(move |event, _, control_flow| {
            match event {
                Event::NewEvents(start_cause) => {
                    self.on_new_events(start_cause);
                }

                Event::WindowEvent {
                    event: window_event,
                    window_id,
                } => {
                    if window_id == window.id() {
                        match window_event {
                            WindowEvent::Resized(physical_size) => {
                                self.on_window_resize(renderer, physical_size);
                            }

                            WindowEvent::Moved(physical_position) => {
                                self.on_window_move(physical_position);
                            }

                            WindowEvent::CloseRequested => {
                                self.on_window_close(control_flow);
                            }

                            WindowEvent::Destroyed => {
                                self.on_window_destroy();
                            }

                            WindowEvent::DroppedFile(path) => {
                                self.on_window_drop_file(path);
                            }

                            WindowEvent::HoveredFile(path) => {
                                self.on_window_hover_file(path);
                            }

                            WindowEvent::HoveredFileCancelled => {
                                self.on_window_hover_file_cancelled();
                            }

                            WindowEvent::ReceivedCharacter(c) => {
                                self.on_window_receive_character(c);
                            }

                            WindowEvent::Focused(gained_focus) => {
                                self.on_window_focus(gained_focus);
                            }

                            WindowEvent::KeyboardInput {
                                device_id,
                                input,
                                is_synthetic,
                            } => {
                                if !is_synthetic && input.virtual_keycode.is_some() {
                                    self.on_window_keyboard_input(
                                        control_flow,
                                        device_id,
                                        renderer,
                                        input,
                                    );
                                }
                            }

                            WindowEvent::ModifiersChanged(state) => {
                                self.on_window_modifiers_change(state);
                            }

                            WindowEvent::CursorMoved {
                                device_id,
                                position,
                                ..
                            } => {
                                self.on_window_cursor_move(device_id, position);
                            }

                            WindowEvent::CursorEntered { device_id } => {
                                self.on_window_cursor_enter(device_id);
                            }

                            WindowEvent::CursorLeft { device_id } => {
                                self.on_window_cursor_left(device_id);
                            }

                            WindowEvent::MouseWheel {
                                device_id,
                                delta,
                                phase,
                                ..
                            } => {
                                self.on_window_mouse_wheel(device_id, delta, phase);
                            }

                            WindowEvent::MouseInput {
                                device_id,
                                state,
                                button,
                                ..
                            } => {
                                self.on_window_mouse_input(device_id, state, button);
                            }

                            WindowEvent::TouchpadPressure {
                                device_id,
                                pressure,
                                stage,
                            } => {
                                self.on_window_touchpad_pressure(device_id, pressure, stage);
                            }

                            WindowEvent::AxisMotion {
                                device_id,
                                axis,
                                value,
                            } => {
                                self.on_window_axis_motion(device_id, axis, value);
                            }

                            WindowEvent::Touch(touch) => {
                                self.on_window_touch(touch);
                            }

                            // The window's scale factor has changed.
                            WindowEvent::ScaleFactorChanged {
                                scale_factor,
                                new_inner_size,
                            } => {
                                self.on_window_scale_change(renderer, scale_factor, new_inner_size);
                            }

                            WindowEvent::ThemeChanged(theme) => {
                                self.on_window_theme_change(theme);
                            }
                            _ => {}
                        }
                    }
                }

                Event::DeviceEvent {
                    device_id: _device_id,
                    event: ref _device_event,
                } => {
                    // Currently we don't have to manage it
                }

                Event::UserEvent(event) => {
                    self.on_user_event(&event);
                }

                Event::Suspended => {
                    self.on_suspend();
                }

                Event::Resumed => {
                    self.on_resume();
                }

                Event::MainEventsCleared => {
                    self.on_redraw(renderer, control_flow);
                }

                Event::RedrawRequested(window_id) => {
                    self.on_redraw_request(&window_id);
                }

                Event::RedrawEventsCleared => {
                    self.on_redraw_clear();
                }

                Event::LoopDestroyed => {
                    self.on_destroy();
                }
            }
        });

        Ok(())
    }

    //- Generic Events -------------------------------------------------------

    fn on_new_events(&self, start_cause: StartCause) {
        let _use_default_behaviour = self.listener.on_new_events(start_cause);
    }

    fn on_user_event(&self, event: &()) {
        let _use_default_behaviour = self.listener.on_user_event(event);
    }

    fn on_suspend(&self) {
        let _use_default_behaviour = self.listener.on_suspend();
    }

    fn on_resume(&self) {
        let _use_default_behaviour = self.listener.on_resume();
    }

    fn on_redraw(
        &self,
        renderer: &mut Renderer<PerspectiveCamera>,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        let use_default_behaviour = self.listener.on_redraw();
        if use_default_behaviour {
            match renderer.redraw() {
                Ok(_) => {}
                Err(error) => match error {
                    // These errors should be resolved by the next frame
                    wgpu::SurfaceError::Timeout | wgpu::SurfaceError::Outdated => {
                        log::error!("{:?}", error)
                    }

                    // Recreate the swap chain if lost
                    wgpu::SurfaceError::Lost => renderer.refresh_current_size(),

                    // The system is out of memory, we should probably quit
                    wgpu::SurfaceError::OutOfMemory => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                },
            }
        }
    }

    fn on_redraw_request(&self, window_id: &winit::window::WindowId) {
        let _use_default_behaviour = self.listener.on_redraw_request(window_id);
    }

    fn on_redraw_clear(&self) {
        let _use_default_behaviour = self.listener.on_redraw_clear();
    }

    fn on_destroy(&self) {
        let _use_default_behaviour = self.listener.on_destroy();
    }

    //- Window Events --------------------------------------------------------

    fn on_window_resize(
        &self,
        renderer: &mut Renderer<PerspectiveCamera>,
        physical_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let use_default_behaviour = self.listener.on_window_resize(physical_size);
        if use_default_behaviour {
            renderer.resize(physical_size);
        }
    }

    fn on_window_move(&self, physical_position: winit::dpi::PhysicalPosition<i32>) {
        let _use_default_behaviour = self.listener.on_window_move(physical_position);
    }

    fn on_window_close(&self, control_flow: &mut winit::event_loop::ControlFlow) {
        let use_default_behaviour = self.listener.on_window_close();
        if use_default_behaviour {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    }

    fn on_window_destroy(&self) {
        let _use_default_behaviour = self.listener.on_window_destroy();
    }

    fn on_window_drop_file(&self, path: PathBuf) {
        let _use_default_behaviour = self.listener.on_window_drop_file(path);
    }

    fn on_window_hover_file(&self, path: PathBuf) {
        let _use_default_behaviour = self.listener.on_window_hover_file(path);
    }

    fn on_window_hover_file_cancelled(&self) {
        let _use_default_behaviour = self.listener.on_window_hover_file_cancelled();
    }

    fn on_window_receive_character(&self, c: char) {
        let _use_default_behaviour = self.listener.on_window_receive_character(c);
    }

    fn on_window_focus(&self, gained_focus: bool) {
        let _use_default_behaviour = self.listener.on_window_focus(gained_focus);
    }

    // Triggered then an user press a key upon this active window.
    // Currently it's preferred to avoid the synthetic event's keys because they
    // works only on Windows and X11 OS; also because the API it's easier and less
    // breakable if used on listener's events.
    // Also then the window is restored (from minimize state) where are
    // input.virtual_keycode KeyboardInput events equals to None.
    fn on_window_keyboard_input(
        &self,
        control_flow: &mut winit::event_loop::ControlFlow,
        device_id: DeviceId,
        renderer: &mut Renderer<PerspectiveCamera>,
        input: KeyboardInput,
    ) {
        // First call a generic method to manage the key events
        let use_default_behaviour = self.listener.on_window_keyboard_input(
            device_id,
            input.state,
            input.virtual_keycode.unwrap(),
        );

        // Then check the input's type for default behaviours
        if use_default_behaviour {
            // Check the camera controller
            let _ = renderer.process_camera_events(input);

            if let KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
            } = input
            {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
        }
    }

    fn on_window_modifiers_change(&self, state: ModifiersState) {
        let _use_default_behaviour = self.listener.on_window_modifiers_change(state);
    }

    fn on_window_cursor_move(
        &self,
        device_id: DeviceId,
        position: winit::dpi::PhysicalPosition<f64>,
    ) {
        let _use_default_behaviour = self.listener.on_window_cursor_move(device_id, position);
    }

    fn on_window_cursor_enter(&self, device_id: DeviceId) {
        let _use_default_behaviour = self.listener.on_window_cursor_enter(device_id);
    }

    fn on_window_cursor_left(&self, device_id: DeviceId) {
        let _use_default_behaviour = self.listener.on_window_cursor_left(device_id);
    }

    fn on_window_mouse_wheel(
        &self,
        device_id: DeviceId,
        delta: MouseScrollDelta,
        phase: TouchPhase,
    ) {
        let _use_default_behaviour = self.listener.on_window_mouse_wheel(device_id, delta, phase);
    }

    fn on_window_mouse_input(&self, device_id: DeviceId, state: ElementState, button: MouseButton) {
        let _use_default_behaviour = self
            .listener
            .on_window_mouse_input(device_id, state, button);
    }

    fn on_window_touchpad_pressure(&self, device_id: DeviceId, pressure: f32, stage: i64) {
        let _use_default_behaviour = self
            .listener
            .on_window_touchpad_pressure(device_id, pressure, stage);
    }

    fn on_window_axis_motion(&self, device_id: DeviceId, axis: u32, value: f64) {
        let _use_default_behaviour = self.listener.on_window_axis_motion(device_id, axis, value);
    }

    fn on_window_touch(&self, touch: Touch) {
        let _use_default_behaviour = self.listener.on_window_touch(touch);
    }

    fn on_window_scale_change(
        &self,
        renderer: &mut Renderer<PerspectiveCamera>,
        scale_factor: f64,
        new_inner_size: &mut winit::dpi::PhysicalSize<u32>,
    ) {
        let use_default_behaviour = self
            .listener
            .on_window_scale_change(scale_factor, new_inner_size);

        if use_default_behaviour {
            // listener.on_window_scale_change may change the new_inner_size values,
            // renderer.resize no
            let new_inner_size_copy = *new_inner_size;
            renderer.resize(new_inner_size_copy);
        }
    }

    fn on_window_theme_change(&self, theme: winit::window::Theme) {
        let _use_default_behaviour = self.listener.on_window_theme_change(theme);
    }
}

//= ERRORS ===================================================================

#[derive(Debug)]
pub enum ApplicationError {
    WindowOsError { source: winit::error::OsError },
    RendererError { source: RendererError },
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::WindowOsError { source } => write!(
                f,
                "The OS cannot perform the requested operation: {}",
                source
            ),
            ApplicationError::RendererError { source } => {
                write!(f, "The Renderer cannot be built: {}", source)
            }
        }
    }
}

impl Error for ApplicationError {}
