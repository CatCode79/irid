//= USES ===========================================================================================

use irid::{ApplicationConfig, Listener, PerspectiveCamera, RendererConfig};
use irid_assets::TextCoordsVertex;
use irid_renderer_interface::Camera;

//= GAME LOGIC =====================================================================================

struct GameListener {}

impl Listener for GameListener {
    fn on_redraw(&self) -> bool {
        true
    }
}

//= MAIN ===========================================================================================

fn main() {
    log::set_max_level(log::LevelFilter::Debug);
    env_logger::init();

    let listener = GameListener { };

    let shader_paths = vec!["examples/lw09_model_loading/assets/shader.wgsl"];

    let texture_path = "examples/lw09_model_loading/assets/happy-tree.png";

    #[rustfmt::skip]
    let vertices = &[irid::assets::ModelVertex] = &[];

    #[rustfmt::skip]
    let indices = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4_u16,
    ];

    // TODO: the correct way is: window_size.width as f32, window_size.height as f32
    // TODO: ignore the proportion incorrecteness until renderer-builder-config refact
    let camera = PerspectiveCamera::new(1920.0 / 2.0, 1080.0 / 2.0);

    let renderer_config: RendererConfig<TextCoordsVertex> = RendererConfig::new()
        .with_clear_color_rgb(0.1, 0.2, 0.3)
        .with_shader_path("lw09_model_loading/assets/shader.wgsl")
        .with_texture_path("lw09_model_loading/assets/happy-tree.png")
        .with_vertices(vertices)
        .with_indices(indices)
        .with_camera(camera);

    let application = ApplicationConfig::new(listener)
        .with_renderer_config(renderer_config)
        .build();

    let _ = application.start();
}
