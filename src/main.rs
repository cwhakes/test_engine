#[macro_use]
extern crate lazy_static;

mod app;
mod engine;

use engine::window::{Window, WINDOW};

fn main() {
    app::AppWindow::init();

    while WINDOW
        .lock()
        .unwrap()
        .as_ref()
        .map(|w| w.running())
        .unwrap_or(false)
    {
        WINDOW.lock().unwrap().as_ref().map(|w| w.broadcast());
    }
}
