#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate listener_derive;
//#[macro_use]
//extern crate vertex_derive;

mod app;
mod shaders;

use engine::window::Window;

fn main() {
    Window::<app::AppWindow>::init();

    while app::WINDOW.broadcast() {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
