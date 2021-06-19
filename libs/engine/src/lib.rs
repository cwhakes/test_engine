#![allow(clippy::single_match)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate vertex_derive;

#[macro_use]
pub mod error;

pub mod prelude;

pub mod components;
pub mod graphics;
pub mod input;
pub mod math;
pub mod physics;
pub mod time;
pub mod util;
pub mod window;