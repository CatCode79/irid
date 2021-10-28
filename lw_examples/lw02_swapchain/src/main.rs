
//= USES ===========================================================================================

use irid::app::{ApplicationBuilder, Config, ConfigBuilder, Listener};
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
    log::set_max_level(log::LevelFilter::Error);
    env_logger::init();

    let config = ConfigBuilder::new()
        .with_clear_color(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        })
        .build();

    let listener: &GameListener = &GameListener { };

    let app = ApplicationBuilder::new_with_config(config).build();
    app.start(listener);
}
