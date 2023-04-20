//= USES ===========================================================================================

use irid::{
    ApplicationBuilder, Camera, Listener, PerspectiveCamera, RendererConfig, TextCoordsVertex,
};

//= GAME LOGIC =====================================================================================

struct GameListener {}

impl Listener for GameListener {
}

//= MAIN ===========================================================================================

fn main() {
    log::set_max_level(log::LevelFilter::Debug);
    env_logger::init();

    let listener = GameListener {};

    #[rustfmt::skip]
    let vertices = &[
        TextCoordsVertex { position: [-0.086824,  0.492403, 0.0], tex_coords: [0.413175, 0.007596], },
        TextCoordsVertex { position: [-0.495134,  0.069586, 0.0], tex_coords: [0.004865, 0.430413], },
        TextCoordsVertex { position: [-0.219185, -0.449397, 0.0], tex_coords: [0.280814, 0.949397], },
        TextCoordsVertex { position: [ 0.359669, -0.347329, 0.0], tex_coords: [0.859670, 0.847329], },
        TextCoordsVertex { position: [ 0.441473,  0.234735, 0.0], tex_coords: [0.941473, 0.265264], },
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
        .with_shader_path("examples/lw07_instancing/assets/shader.wgsl")
        .with_texture_path("examples/lw07_instancing/assets/happy-tree.png")
        .with_vertices(vertices)
        .with_indices(indices)
        .with_camera(camera);

    let application = ApplicationBuilder::new(listener)
        .with_renderer_config(renderer_config)
        .build();

    let _ = application.start();
}
