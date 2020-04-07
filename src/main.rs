#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vertex_derive;

mod app;
mod engine;

use std::sync::atomic::{Ordering};
use engine::window::{Window, WINDOW};

fn main() {
    app::AppWindow::init();

    while WINDOW
        .lock()
        .unwrap()
        .as_ref()
        .map(|w| w.window_inner().running.load(Ordering::Relaxed))
        .unwrap_or(false)
    {
        WINDOW.lock().unwrap().as_mut().map(|w| w.broadcast());
    }
}
