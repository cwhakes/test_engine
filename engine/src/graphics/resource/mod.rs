pub mod manager;
pub mod mesh;
pub mod texture;

pub use manager::ResourceManager;
pub use mesh::Mesh;
pub use texture::Texture;

use crate::error;
use crate::graphics::render::Device;

use std::path::Path;

pub trait Resource: Clone {
    fn load_resource_from_file(device: &Device, path: impl AsRef<Path>) -> error::Result<Self>;
}
