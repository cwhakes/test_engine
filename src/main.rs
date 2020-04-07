#[macro_use]
extern crate vertex_derive;

mod app;

use engine::window::{Window, WINDOW};

fn main() {
    app::AppWindow::init();

    while WINDOW
        .lock()
        .unwrap()
        .as_ref()
        .map(|w| w.window_inner().running)
        .unwrap_or(false)
    {
        WINDOW.lock().unwrap().as_mut().map(|w| w.broadcast());
    }
}
