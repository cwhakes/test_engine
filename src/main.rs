#[macro_use]
extern crate vertex_derive;

mod app;

use engine::input::INPUT;
use engine::window::{Window, WINDOW};

fn main() {
    app::AppWindow::init();

    while WINDOW
        .lock()
        .unwrap()
        .as_ref()
        .map(|w| w.lock().unwrap().window_inner().running)
        .unwrap_or(false)
    {
        INPUT.lock().unwrap().update();
        WINDOW
            .lock()
            .unwrap()
            .as_ref()
            .map(|w| w.lock().unwrap().broadcast());
    }
}
