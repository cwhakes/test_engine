#![allow(
    clippy::missing_safety_doc,
    clippy::single_match,
    clippy::uninlined_format_args
)]

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
