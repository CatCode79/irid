
//= USES ===========================================================================================

use std::collections::HashMap;

use irid::app::{Application, Config, Listener};
use wgpu::Color;
use winit::dpi::PhysicalSize;


//= GAME LOGIC =====================================================================================

struct GameListener {}

impl Listener for GameListener {
    fn on_suspend(&self) -> bool {
        true
    }

    fn on_resume(&self) -> bool {
        true
    }

    fn on_redraw(&self) -> bool {
        true
    }

    fn on_destroy(&self) -> bool {
        true
    }

    fn on_window_resize(&self, _new_size: PhysicalSize<u32>) -> bool {
        true
    }
}


//= MAIN ===========================================================================================

fn main() {
    env_logger::init();
    log::set_max_level(LevelFilter::Debug);

    let listener: &'static GameListener = &GameListener { };

    let mut config = Config::default();
    config.clear_color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    };

    const SHADER_WGSL_FILENAME: &str = "shader.wgsl";
    let mut shaders: HashMap<String, String> = HashMap::new();
    shaders.insert(SHADER_WGSL_FILENAME.to_string(), include_str!(SHADER_WGSL_FILENAME).to_string());

    let app = Application::new(config, shaders);
    app.start(listener);
}
