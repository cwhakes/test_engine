#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vertex_derive;

mod app;

use engine::input::INPUT;

fn main() {
    app::WINDOW.init();

    while app::WINDOW.is_running() {
        INPUT.lock().unwrap().update();
        app::WINDOW.broadcast();
    }
}
