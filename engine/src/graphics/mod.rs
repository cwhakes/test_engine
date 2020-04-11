pub mod render;

use render::Render;

use crate::error;
use crate::window::Hwnd;

use std::sync::Mutex;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Option<Graphics>> = Mutex::new(None);
}

pub struct Graphics {
    pub render: Render,
}

impl Graphics {
    pub fn new(hwnd: &Hwnd) -> error::Result<Graphics> {
        let render = Render::new(hwnd)?;
        Ok(Graphics { render })
    }
}
