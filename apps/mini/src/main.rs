#![allow(clippy::single_match)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate listener_derive;
//#[macro_use]
//extern crate vertex_derive;

mod app;
mod minigame;

use engine::window::Window;
use log::info;

fn main() {
    env_logger::init();
    info!("Starting up..");

    Window::<app::AppWindow>::init();
    Window::<minigame::AppWindow>::init();

    while app::WINDOW.broadcast() | minigame::WINDOW.broadcast() {
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
