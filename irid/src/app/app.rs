
//= APP STRUCT =====================================================================================

pub struct App {
}


impl App {

    /**
     *
     */
    pub fn new(_app_builder: Option<()>) -> App {
        env_logger::init();

        App {

        }
    }

    /**
     *
     */
    pub fn start(self) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = crate::renderer::Renderer::new(&window);

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                winit::event::WindowEvent::CloseRequested => self.on_window_close_requested_event(control_flow),
                winit::event::WindowEvent::KeyboardInput { input, .. } => match input {
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = winit::event_loop::ControlFlow::Exit,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        });
    }

    //- Default Event's Methods --------------------------------------------------------------------

    fn on_window_close_requested_event(&self, control_flow: &mut winit::event_loop::ControlFlow) {
        *control_flow = winit::event_loop::ControlFlow::Exit;
    }
}


//= WINDOW EVENT'S TRAITS ==========================================================================

pub trait CloseRequestedEvent {
}

//impl CloseRequestedEvent for App {}
