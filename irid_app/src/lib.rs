//= MODS =====================================================================

mod application;
mod listener;

//= RE-EXPORTS ===============================================================

pub use self::application::*;
pub use self::listener::*;

pub type Window = winit::window::Window;
pub type WindowConfig = winit::window::WindowBuilder;
