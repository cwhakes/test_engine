pub mod render;
pub mod resource;

use render::Render;
use resource::texture::{Texture, TextureManager};

use crate::error;

use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Graphics> = Mutex::new(Graphics::new().unwrap());
}

pub struct Graphics {
    pub render: Render,
    pub texture_manager: TextureManager,
}

impl Graphics {
    pub fn new() -> error::Result<Graphics> {
        let render = Render::new()?;
        let texture_manager = TextureManager::new();
        Ok(Graphics { render, texture_manager })
    }

    pub fn get_texture_from_file(&mut self, path: &Path) -> error::Result<Texture> {
        self.texture_manager.get_resource_from_file(self.render.device(), path)
    }
}
