
//= USES ===========================================================================================

use irid;


//= GAME LOGIC =====================================================================================

struct GameListener {}

impl irid::window::EventListener for GameListener {
    fn on_redraw_begin(&self) -> bool {
        true
    }
}

impl irid::window::WindowListener for GameListener { }


//= MAIN ===========================================================================================

fn main() {
    let listener: &'static GameListener = &GameListener { };

    let app = irid::app::App::new();
    app.start(listener);
}
