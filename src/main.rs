//= MODS ===========================================================================================

mod state;
mod irid;


//= USES ===========================================================================================

use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

use self::state::State;


//= MAIN ===========================================================================================

fn main() {
	env_logger::init();
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new()
		.with_title(&[env!("CARGO_PKG_DESCRIPTION"), " v", env!("CARGO_PKG_VERSION")].join(""))
		.build(&event_loop)
		.unwrap();

	// Since main can't be async, we're going to need to block
	let mut state: State = State::new(&window);

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent { ref event, window_id, } if window_id == window.id() => {
				if !state.input(event) {
					// UPDATED!
					match event {
						WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
						WindowEvent::KeyboardInput { input, .. } => match input {
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							} => *control_flow = ControlFlow::Exit,
							_ => {}
						},
						WindowEvent::Resized(physical_size) => {
							state.resize(*physical_size);
						}
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
							// new_inner_size is &&mut so w have to dereference it twice
							state.resize(**new_inner_size);
						}
						_ => {}
					}
				}
			}
			Event::RedrawRequested(_) => {
				state.update();
				match state.render() {
					Ok(_) => {}
					// Recreate the swap_chain if lost
					Err(wgpu::SwapChainError::Lost) => state.refresh_size(),
					// The system is out of memory, we should probably quit
					Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					// All other errors (Outdated, Timeout) should be resolved by the next frame
					Err(e) => eprintln!("{:?}", e),
				}
			}
			Event::MainEventsCleared => {
				// RedrawRequested will only trigger once, unless we manually
				// request it.
				window.request_redraw();
			}
			_ => {}
		}
	});
}
