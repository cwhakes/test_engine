use super::Resource;

use crate::error;
use crate::graphics::render::Device;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct ResourceManager<R: Resource> {
    map: HashMap<PathBuf, Arc<R>>,
}

impl<R: Resource> ResourceManager<R> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_resource_from_file(
        &mut self,
        device: &Device,
        path: impl AsRef<Path>,
    ) -> error::Result<Arc<R>> {
        let path = path.as_ref().canonicalize()?;
        if let Some(resource) = self.map.get(&path) {
            Ok(resource.clone())
        } else {
            let resource = R::load_resource_from_file(device, &path)?;
            self.map.insert(path, resource.clone());
            Ok(resource)
        }
    }
}

impl<R: Resource> Default for ResourceManager<R> {
    fn default() -> Self {
        Self::new()
    }
}
