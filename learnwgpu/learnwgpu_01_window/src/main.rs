
//= USES ===========================================================================================

use irid;


//= GAME LOGIC =====================================================================================

struct GameListener {}

impl irid::window::EventListener for GameListener {
    fn on_redraw(&self) -> bool {
        true
    }
}

impl irid::window::WindowListener for GameListener { }


//= MAIN ===========================================================================================

fn main() {
    env_logger::init();

    let listener: &'static GameListener = &GameListener { };

    let app = irid::app::App::default();
    app.start(listener);
}
