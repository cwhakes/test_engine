pub mod color;
pub mod material;
pub mod render;
pub mod resource;
pub mod vertex;

use material::Material;
use render::Render;
use resource::mesh::{Mesh, MeshManager};
use resource::shader::{Pixel, Shader, ShaderManager, Vertex};
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
    pub vs_manager: ShaderManager<Vertex>,
    pub ps_manager: ShaderManager<Pixel>,
}

impl Graphics {
    pub fn new() -> error::Result<Self> {
        Ok(Self {
            render: Render::new()?,
            mesh_manager: MeshManager::new(),
            texture_manager: TextureManager::new(),
            vs_manager: ShaderManager::new(),
            ps_manager: ShaderManager::new(),
        })
    }

    pub fn get_texture_from_file(&mut self, path: impl AsRef<Path>) -> error::Result<Texture> {
        self.texture_manager
            .get_resource_from_file(self.render.device(), path)
    }

    pub fn get_mesh_from_file(&mut self, path: impl AsRef<Path>) -> error::Result<Mesh> {
        self.mesh_manager
            .get_resource_from_file(self.render.device(), path)
    }

    pub fn get_vertex_shader_from_file(
        &mut self,
        path: impl AsRef<Path>,
    ) -> error::Result<Shader<Vertex>> {
        self.vs_manager
            .get_resource_from_file(self.render.device(), path)
    }

    pub fn get_pixel_shader_from_file(
        &mut self,
        path: impl AsRef<Path>,
    ) -> error::Result<Shader<Pixel>> {
        self.ps_manager
            .get_resource_from_file(self.render.device(), path)
    }

    pub fn new_material(
        &mut self,
        vs: impl AsRef<Path>,
        ps: impl AsRef<Path>,
    ) -> error::Result<Material> {
        Material::new(self, vs, ps)
    }
}
