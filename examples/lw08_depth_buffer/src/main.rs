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

    let listener = GameListener {};

    #[rustfmt::skip]
    let vertices = &[
        TextCoordsVertex { position: [-0.08682410,  0.49240386, 0.0], tex_coords: [0.4131759000, 0.00759614], },
        TextCoordsVertex { position: [-0.49513406,  0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], },
        TextCoordsVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.2808145300, 0.94939700], },
        TextCoordsVertex { position: [ 0.35966998, -0.34732910, 0.0], tex_coords: [0.8596700000, 0.84732914], },
        TextCoordsVertex { position: [ 0.44147372,  0.23473590, 0.0], tex_coords: [0.9414737000, 0.26526410], },
    ];

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
        .with_shader_path("examples/lw08_depth_buffer/assets/shader.wgsl")
        .with_texture_path("examples/lw08_depth_buffer/assets/happy-tree.png")
        .with_vertices(vertices)
        .with_indices(indices)
        .with_camera(camera);

    let application = ApplicationConfig::new(listener)
        .with_renderer_config(renderer_config)
        .build();

    let _ = application.start();
}
