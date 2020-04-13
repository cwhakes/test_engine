#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vertex_derive;

mod app;

use engine::window::Window;

fn main() {
    Window::<app::AppWindow>::init();

    while app::WINDOW.broadcast() {}
}
