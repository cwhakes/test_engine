pub mod render;

use render::Render;

use crate::error;

use std::sync::Mutex;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Graphics> = Mutex::new(Graphics::new().unwrap());
}

pub struct Graphics {
    pub render: Render,
}

impl Graphics {
    pub fn new() -> error::Result<Graphics> {
        let render = Render::new()?;
        Ok(Graphics { render })
    }
}
