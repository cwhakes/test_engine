#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vertex_derive;

mod app;

use engine::input::INPUT;
use engine::window::Window;

fn main() {
    app::AppWindow::init();

    while app::WINDOW
        .lock()
        .unwrap()
        .as_mut()
        .map(|w| w.window_inner().running)
        .unwrap_or(false)
    {
        INPUT.lock().unwrap().update();
        app::WINDOW
            .lock()
            .unwrap()
            .as_mut()
            .map(|w| w.broadcast());
    }
}
