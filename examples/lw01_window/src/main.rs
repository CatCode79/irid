//= USES ===========================================================================================

use irid::{ApplicationBuilder, ColorVertex, Listener};

//= LISTENER =======================================================================================

struct GameListener;

impl Listener for GameListener {
}

//= MAIN ===========================================================================================

fn main() {
    log::set_max_level(log::LevelFilter::Debug);
    env_logger::init();

    let listener = GameListener {};

    let application = ApplicationBuilder::<'_, _, ColorVertex>::new(listener).build();

    let _ = application.start();
}
