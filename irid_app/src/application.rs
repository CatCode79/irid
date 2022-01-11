//= USES ===========================================================================================

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use bytemuck::Pod;
use thiserror::Error;

use irid_app_interface::{Window, WindowBuilder};
use irid_assets::{DiffuseImageSize, DiffuseTexture};
use irid_assets_interface::{Index, Vertex};
use irid_renderer::{Renderer, RendererBuilder, RendererError};

use crate::Listener;

//= ERRORS =========================================================================================

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("the OS cannot perform the requested operation")]
    WindowOsError {
        #[from]
        source: winit::error::OsError,
    },
    #[error("the Renderer cannot be built")]
    RendererError {
        #[from]
        source: RendererError,
    },
}

//= APPLICATION BUILDER ============================================================================

/// Build a new [Application] with wanted values.
#[derive(Clone, Debug, Default)]
pub struct ApplicationBuilder<
    'a,
    L: Listener,
    B: WindowBuilder,
    PS: AsRef<Path>,
    PT: AsRef<Path>,
    V: Vertex,
    I: Index,
> {
    listener: L,
    window_builder: Option<B>,

    // Renderer stuff
    shader_paths: Option<Vec<PS>>,
    texture_path: Option<PT>,
    vertices: Option<&'a [V]>,
    indices: Option<&'a [I]>,

    // Renderer specific options
    clear_color: Option<wgpu::Color>,
}

impl<'a, L, B, PS, PT, V, I> ApplicationBuilder<'a, L, B, PS, PT, V, I>
where
    L: Listener,
    B: WindowBuilder,
    PS: AsRef<std::path::Path>,
    PT: AsRef<std::path::Path>,
    V: Vertex,
    I: Index,
{
    //- Constructors -------------------------------------------------------------------------------

    ///
    pub fn new(listener: L) -> Self {
        Self {
            listener,
            window_builder: None,
            shader_paths: None,
            texture_path: None,
            vertices: None,
            indices: None,
            clear_color: None,
        }
    }

    //- Setters ------------------------------------------------------------------------------------

    ///
    pub fn with_listener(mut self, listener: L) -> Self {
        self.listener = listener;
        self
    }

    ///
    pub fn with_window_builder(mut self, window_builder: B) -> Self {
        self.window_builder = Some(window_builder);
        self
    }

    /*
    ///
    pub fn with_window(mut self, window: W) -> Self {
        self.window = Some(window);
        self
    }

    ///
    pub fn with_event_loop(mut self, event_loop: winit::event_loop::EventLoop<()>) -> Self {
        self.event_loop = Some(event_loop);
        self
    }
    */

    ///
    pub fn with_shader_paths(mut self, shader_paths: Vec<PS>) -> Self {
        self.shader_paths = Some(shader_paths);
        self
    }

    ///
    pub fn with_texture_path(mut self, texture_path: PT) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    ///
    pub fn with_vertices(mut self, vertices: &'a [V]) -> Self {
        self.vertices = Some(vertices);
        self
    }

    ///
    pub fn with_indices(mut self, indices: &'a [I]) -> Self {
        self.indices = Some(indices);
        self
    }

    /// Color used by a [render pass color attachment](wgpu::RenderPassColorAttachment)
    /// to perform a [clear operation](wgpu::LoadOp).
    pub fn with_clear_color(mut self, clear_color: wgpu::Color) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    //- Build --------------------------------------------------------------------------------------

    /// Build a new [Application] with given values.
    pub fn build(self) -> Application<'a, L, B, PS, PT, V, I> {
        Application {
            listener: self.listener,
            window_builder: self.window_builder.unwrap_or_else(B::new),
            shader_paths: self.shader_paths,
            texture_path: self.texture_path,
            vertices: self.vertices,
            indices: self.indices,
            clear_color: self.clear_color,
        }
    }
}

//= APPLICATION ====================================================================================

/// Manages the whole game setup and logic.
#[derive(Clone, Debug, Default)]
pub struct Application<
    'a,
    L: Listener,
    B: WindowBuilder,
    PS: AsRef<Path>,
    PT: AsRef<Path>,
    V: Vertex,
    I: Index,
> {
    listener: L,
    window_builder: B,

    // Renderer stuffs
    shader_paths: Option<Vec<PS>>,
    texture_path: Option<PT>,
    vertices: Option<&'a [V]>,
    indices: Option<&'a [I]>,

    // Renderer specific options
    clear_color: Option<wgpu::Color>,
}

impl<'a, L, B, PS, PT, V, I> Application<'a, L, B, PS, PT, V, I>
where
    L: Listener,
    B: WindowBuilder + Clone,
    PS: AsRef<Path> + Clone + Debug,
    PT: AsRef<Path> + Clone + Debug,
    V: Vertex + Pod,
    I: Index + Pod,
{
    /// Starts the
    /// [event loop](https://docs.rs/winit/0.25.0/winit/event_loop/struct.EventLoop.html).
    ///
    /// # Notes:
    ///
    /// The event loop uses the winit
    /// [run_return](https://docs.rs/winit/0.25.0/winit/platform/run_return/trait.EventLoopExtRunReturn.html#tymethod.run_return)
    /// method.
    ///
    /// I know I should use winit
    /// [run](https://docs.rs/winit/0.25.0/winit/event_loop/struct.EventLoop.html#method.run)
    /// method instead but all those static variables are a bore to handle.
    ///
    /// To remember that the resize is not managed perfectly with run_return.
    pub fn start(self) -> Result<(), ApplicationError>
    where
        <B as irid_app_interface::WindowBuilder>::BuildOutput: irid_app_interface::Window,
    {
        let mut event_loop = winit::event_loop::EventLoop::new();
        // TODO: remove the clone
        let mut window = self.window_builder.clone().build(&event_loop)?;

        let mut renderer_builder = RendererBuilder::<
            <B as WindowBuilder>::BuildOutput,
            PS,
            PT,
            V,
            I,
            DiffuseImageSize,
            DiffuseTexture<DiffuseImageSize>,
        >::new(&window);
        if self.clear_color.is_some() {
            // TODO: we have to have with_clear_color method only on RenderBuilder
            //  and not also in ApplicationBuilder, so we can ride with this unwrap
            renderer_builder = renderer_builder.with_clear_color(self.clear_color.unwrap());
        }
        if self.shader_paths.is_some() {
            // TODO: remove unwrap and clone
            renderer_builder = renderer_builder.with_shader_path(
                self.shader_paths
                    .as_ref()
                    .map(|v| v.as_slice()[0].clone())
                    .unwrap(),
            );
        }
        if self.texture_path.is_some() {
            // TODO: remove the clone
            renderer_builder =
                renderer_builder.with_texture_path(self.texture_path.as_ref().unwrap().clone());
        }
        if self.vertices.is_some() {
            renderer_builder = renderer_builder.with_vertices(self.vertices.unwrap());
        }
        if self.indices.is_some() {
            renderer_builder = renderer_builder.with_indices(self.indices.unwrap());
        }
        let mut renderer = renderer_builder.build()?;

        // Now is a good time to make the window visible: after the renderer has been initialized,
        // in this way we avoid a slight visible/invisible toggling effect of the window
        window.conclude_visibility_delay();

        use winit::platform::run_return::EventLoopExtRunReturn;
        event_loop.run_return(move |event, _, control_flow| match event {
            winit::event::Event::NewEvents(start_cause) => {
                self.on_new_events(start_cause);
            }

            winit::event::Event::WindowEvent {
                event: window_event,
                window_id,
            } => {
                // TODO: add a multi-monitor support
                if window_id == window.id() {
                    match window_event {
                        winit::event::WindowEvent::Resized(physical_size) => {
                            self.on_window_resize(&mut renderer, physical_size);
                        }

                        winit::event::WindowEvent::Moved(physical_position) => {
                            self.on_window_move(physical_position);
                        }

                        winit::event::WindowEvent::CloseRequested => {
                            self.on_window_close(control_flow);
                        }

                        winit::event::WindowEvent::Destroyed => {
                            self.on_window_destroy();
                        }

                        winit::event::WindowEvent::DroppedFile(path) => {
                            self.on_window_drop_file(path);
                        }

                        winit::event::WindowEvent::HoveredFile(path) => {
                            self.on_window_hover_file(path);
                        }

                        winit::event::WindowEvent::HoveredFileCancelled => {
                            self.on_window_hover_file_cancelled();
                        }

                        winit::event::WindowEvent::ReceivedCharacter(c) => {
                            self.on_window_receive_character(c);
                        }

                        winit::event::WindowEvent::Focused(gained_focus) => {
                            self.on_window_focus(gained_focus);
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        } => {
                            if !is_synthetic && input.virtual_keycode.is_some() {
                                self.on_window_keyboard_input(
                                    control_flow,
                                    device_id,
                                    &mut renderer,
                                    &input,
                                );
                            }
                        }

                        winit::event::WindowEvent::ModifiersChanged(state) => {
                            self.on_window_modifiers_change(state);
                        }

                        winit::event::WindowEvent::CursorMoved {
                            device_id,
                            position,
                            ..
                        } => {
                            self.on_window_cursor_move(device_id, position);
                        }

                        winit::event::WindowEvent::CursorEntered { device_id } => {
                            self.on_window_cursor_enter(device_id);
                        }

                        winit::event::WindowEvent::CursorLeft { device_id } => {
                            self.on_window_cursor_left(device_id);
                        }

                        winit::event::WindowEvent::MouseWheel {
                            device_id,
                            delta,
                            phase,
                            ..
                        } => {
                            self.on_window_mouse_wheel(device_id, delta, phase);
                        }

                        winit::event::WindowEvent::MouseInput {
                            device_id,
                            state,
                            button,
                            ..
                        } => {
                            self.on_window_mouse_input(device_id, state, button);
                        }

                        winit::event::WindowEvent::TouchpadPressure {
                            device_id,
                            pressure,
                            stage,
                        } => {
                            self.on_window_touchpad_pressure(device_id, pressure, stage);
                        }

                        winit::event::WindowEvent::AxisMotion {
                            device_id,
                            axis,
                            value,
                        } => {
                            self.on_window_axis_motion(device_id, axis, value);
                        }

                        winit::event::WindowEvent::Touch(touch) => {
                            self.on_window_touch(touch);
                        }

                        // The window's scale factor has changed.
                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            self.on_window_scale_change(
                                &mut renderer,
                                scale_factor,
                                new_inner_size,
                            );
                        }

                        winit::event::WindowEvent::ThemeChanged(theme) => {
                            self.on_window_theme_change(theme);
                        }
                    }
                }
            }

            winit::event::Event::DeviceEvent {
                device_id: _device_id,
                event: ref _device_event,
            } => {
                // TODO: Currently we don't have to manage it
            }

            winit::event::Event::UserEvent(event) => {
                self.on_user_event(&event);
            }

            winit::event::Event::Suspended => {
                self.on_suspend();
            }

            winit::event::Event::Resumed => {
                self.on_resume();
            }

            winit::event::Event::MainEventsCleared => {
                self.on_redraw(&mut renderer, control_flow);
            }

            winit::event::Event::RedrawRequested(window_id) => {
                self.on_redraw_request(&window_id);
            }

            winit::event::Event::RedrawEventsCleared => {
                self.on_redraw_clear();
            }

            winit::event::Event::LoopDestroyed => {
                self.on_destroy();
            }
        });

        Ok(())
    }

    //- Generic Events -----------------------------------------------------------------------------

    fn on_new_events(&self, start_cause: winit::event::StartCause) {
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

    // This method is probably one of the few that must always be inline.
    #[inline(always)]
    fn on_redraw(
        &self,
        renderer: &mut Renderer,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        let use_default_behaviour = self.listener.on_redraw();
        if use_default_behaviour {
            match renderer.redraw() {
                Ok(_) => {}
                Err(error) => match error {
                    // These errors should be resolved by the next frame
                    wgpu::SurfaceError::Timeout | wgpu::SurfaceError::Outdated => {
                        // TODO: better error messages?
                        eprintln!("{:?}", error)
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

    //- Window Events ------------------------------------------------------------------------------

    fn on_window_resize(
        &self,
        renderer: &mut Renderer,
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
        device_id: winit::event::DeviceId,
        renderer: &mut Renderer,
        input: &winit::event::KeyboardInput,
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
            renderer.process_camera_events(input);

            if let winit::event::KeyboardInput {
                state: winit::event::ElementState::Pressed,
                virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                ..
            } = input
            {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
        }
    }

    fn on_window_modifiers_change(&self, state: winit::event::ModifiersState) {
        let _use_default_behaviour = self.listener.on_window_modifiers_change(state);
    }

    fn on_window_cursor_move(
        &self,
        device_id: winit::event::DeviceId,
        position: winit::dpi::PhysicalPosition<f64>,
    ) {
        let _use_default_behaviour = self.listener.on_window_cursor_move(device_id, position);
    }

    fn on_window_cursor_enter(&self, device_id: winit::event::DeviceId) {
        let _use_default_behaviour = self.listener.on_window_cursor_enter(device_id);
    }

    fn on_window_cursor_left(&self, device_id: winit::event::DeviceId) {
        let _use_default_behaviour = self.listener.on_window_cursor_left(device_id);
    }

    fn on_window_mouse_wheel(
        &self,
        device_id: winit::event::DeviceId,
        delta: winit::event::MouseScrollDelta,
        phase: winit::event::TouchPhase,
    ) {
        let _use_default_behaviour = self.listener.on_window_mouse_wheel(device_id, delta, phase);
    }

    fn on_window_mouse_input(
        &self,
        device_id: winit::event::DeviceId,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) {
        let _use_default_behaviour = self
            .listener
            .on_window_mouse_input(device_id, state, button);
    }

    fn on_window_touchpad_pressure(
        &self,
        device_id: winit::event::DeviceId,
        pressure: f32,
        stage: i64,
    ) {
        let _use_default_behaviour = self
            .listener
            .on_window_touchpad_pressure(device_id, pressure, stage);
    }

    fn on_window_axis_motion(&self, device_id: winit::event::DeviceId, axis: u32, value: f64) {
        let _use_default_behaviour = self.listener.on_window_axis_motion(device_id, axis, value);
    }

    fn on_window_touch(&self, touch: winit::event::Touch) {
        let _use_default_behaviour = self.listener.on_window_touch(touch);
    }

    fn on_window_scale_change(
        &self,
        renderer: &mut Renderer,
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
