
//= USES ===========================================================================================

use std::collections::HashMap;
use std::fs::read_to_string;

use irid::app::{Application, ApplicationBuilder, ConfigBuilder, Listener};
use irid::renderer::VertexColor;
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

    const SHADER_WGSL_FILENAME: &str = "shader.wgsl";
    const SHADER_WGSL_FILEPATH: &str = "D:/_BLACK_ABYSS_DUNGEON/_BAD/shaded_sun/lw_examples/lw04_buffers_indices/assets/shader.wgsl";

    // TODO: Passare solo la path o il nome file
    let mut shaders: HashMap<String, String> = HashMap::new();
    let frag_wgsl = match read_to_string(SHADER_WGSL_FILEPATH) {
        Ok(file) => file.clone(),  // TODO: Cercare di rimuovere il clone
        Err(why) => panic!("couldn't open {} file: {}", SHADER_WGSL_FILENAME, why),
    };
    shaders.insert(SHADER_WGSL_FILENAME.to_string(), frag_wgsl);

    // We arrange the vertices in counter clockwise order: top, bottom left, bottom right.
    // We do it this way partially out of tradition, but mostly because we specified in the
    // rasterization_state of the render_pipeline that we want the front_face of our triangle
    // to be wgpu::FrontFace::Ccw so that we cull the back face.
    const VERTICES: &[VertexColor] = &[
        VertexColor { position: [-0.08682410,  0.49240386, 0.0], color: [0.5, 0.0, 0.5] },
        VertexColor { position: [-0.49513406,  0.06958647, 0.0], color: [0.5, 0.0, 0.5] },
        VertexColor { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] },
        VertexColor { position: [ 0.35966998, -0.34732910, 0.0], color: [0.5, 0.0, 0.5] },
        VertexColor { position: [ 0.44147372,  0.23473590, 0.0], color: [0.5, 0.0, 0.5] },
    ];

    const INDICES: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        /* padding */ 0,
    ];

    let app = ApplicationBuilder::new_with_config(config).build();
    app.start(listener);
}
