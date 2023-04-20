//= USES ===========================================================================================

use irid::{ApplicationBuilder, ColorVertex, Listener, RendererConfig};

//= GAME LOGIC =====================================================================================

struct GameListener {}

impl Listener for GameListener {
}

//= MAIN ===========================================================================================

fn main() {
    log::set_max_level(log::LevelFilter::Debug);
    env_logger::init();

    let listener = GameListener {};

    let renderer_config = RendererConfig::<'_, ColorVertex>::new()
        .with_clear_color_rgb(0.1, 0.2, 0.3)
        .with_shader_path("examples/lw03_pipeline/assets/shader.wgsl");

    let application = ApplicationBuilder::new(listener)
        .with_renderer_config(renderer_config)
        .build();

    let _ = application.start();
}
