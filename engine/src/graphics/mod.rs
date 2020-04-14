pub mod render;
pub mod resource;

use render::Render;
use resource::mesh::{Mesh, MeshManager};
use resource::texture::{Texture, TextureManager};

use crate::error;

use std::path::Path;
use std::sync::Mutex;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Graphics> = Mutex::new(Graphics::new().unwrap());
}

pub struct Graphics {
    pub render: Render,
    pub mesh_manager: MeshManager,
    pub texture_manager: TextureManager,
}

impl Graphics {
    pub fn new() -> error::Result<Graphics> {
        Ok(Graphics {
            render: Render::new()?,
            mesh_manager: MeshManager::new(),
            texture_manager: TextureManager::new(),
        })
    }

    pub fn get_texture_from_file(&mut self, path: &Path) -> error::Result<Texture> {
        self.texture_manager.get_resource_from_file(self.render.device(), path)
    }

    pub fn get_mesh_from_file(&mut self, path: &Path) -> error::Result<Mesh> {
        self.mesh_manager.get_resource_from_file(self.render.device(), path)
    }
}
