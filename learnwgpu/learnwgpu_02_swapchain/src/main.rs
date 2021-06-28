
//= USES ===========================================================================================

use irid;


//= GAME LOGIC =====================================================================================

struct GameListener {}

impl irid::app::EventListener for GameListener {
    fn on_redraw(&self) -> bool {
        true
    }
}

impl irid::app::WindowListener for GameListener { }


//= MAIN ===========================================================================================

fn main() {
    let listener: &'static GameListener = &GameListener { };

    let app = irid::app::App::default();
    app.start(listener);
}
