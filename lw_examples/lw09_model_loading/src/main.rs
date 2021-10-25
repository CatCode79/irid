
//= USES ===========================================================================================

use std::collections::HashMap;
use std::fs::read_to_string;

use wgpu::Color;
use winit::dpi::PhysicalSize;

use irid::app::{ApplicationBuilder, ConfigBuilder, Listener};


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

    const SHADER_WGSL_FILENAME: &str = "shader.wgsl";
    const SHADER_WGSL_FILEPATH: &str = "D:/_BLACK_ABYSS_DUNGEON/_BAD/shaded_sun/lw_examples/lw09_model_loading/assets/shader.wgsl";

    // TODO: Passare solo la path o il nome file
    let mut shaders: HashMap<String, String> = HashMap::new();
    let frag_wgsl = match read_to_string(SHADER_WGSL_FILEPATH) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {} file: {}", SHADER_WGSL_FILENAME, why),
    };
    shaders.insert(SHADER_WGSL_FILENAME.to_string(), frag_wgsl);

    const TREE_FILEPATH: &str = "D:/_BLACK_ABYSS_DUNGEON/_BAD/shaded_sun/lw_examples/lw09_model_loading/assets/happy-tree.png";

    // We arrange the vertices in counter clockwise order: top, bottom left, bottom right.
    // We do it this way partially out of tradition, but mostly because we specified in the
    // rasterization_state of the render_pipeline that we want the front_face of our triangle
    // to be wgpu::FrontFace::Ccw so that we cull the back face.
    const VERTICES: &[irid::assets::ModelVertex] = &[];

    const INDICES: &[u32] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        /* padding */ 0,
    ];

    let app = ApplicationBuilder::new_with_config(config)
        .with_shaders(shaders)
        .with_texture_path(std::path::Path::new(TREE_FILEPATH))
        .with_vertices(VERTICES)
        .with_indices(INDICES)
        .build();

    let _ret = app.start(listener);
}
