use super::{Resource, ResourceManager, shader};

use crate::error;
use crate::graphics::render::{Device, IndexBuffer, VertexBuffer};
use crate::graphics::vertex;
use crate::math::Vector3d;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use wavefront_obj::{obj, mtl};

pub type MeshManager = ResourceManager<Mesh>;

#[derive(Clone)]
pub struct Mesh(Arc<Mutex<MeshInner>>);

impl Resource for Mesh {
    fn load_resource_from_file(device: &Device, path: impl AsRef<Path>) -> error::Result<Self> {
        let mut string = String::new();

        let mut file = File::open(path.as_ref())?;
        file.read_to_string(&mut string)?;
        let obj_set = obj::parse(&string)?;

        let mut mtl_map = HashMap::new();
        if let Some(mtl_file) = obj_set.material_library.as_ref() {
            if let Ok(mtl_set) = load_material(path.as_ref().join(mtl_file)) {
                for (index, mtl) in mtl_set.materials.iter().enumerate() {
                    mtl_map.insert(mtl.name.clone(), index);
                }
            } else {
                    println!("Material not found for object: {}", path.as_ref().display());
            }
        }
        let num_mats = mtl_map.len();

        let mut geometries: Vec<_> = obj_set.objects.iter()
            .flat_map(|object| object.geometry.iter().map(move |geometry| (object, geometry)))
            .collect();
        geometries.sort_by_key(|(_, geometry)| {
            geometry.material_name
                .as_ref()
                .and_then(|name| mtl_map.get(name))
                .unwrap_or(&num_mats)
        });

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index = 0;
        let mut material_index = MaterialIndex {
            start_index: 0,
            len: 0,
            material_name: geometries[0].1.material_name.clone()
        };
        let mut material_indices = Vec::new();

        for (object, geometry) in geometries.iter() {
            if geometry.material_name != material_index.material_name {
                material_index.len = index - material_index.start_index;

                let new_material_index = MaterialIndex {
                    start_index: index,
                    len: 0,
                    material_name: geometry.material_name.clone(),
                };

                material_indices.push(
                    mem::replace(&mut material_index, new_material_index)
                );
            }

            for shape in geometry.shapes.iter() {
                match shape.primitive {
                    obj::Primitive::Triangle(a, b, c) => {
                        let normal = calc_normal(object, [&a, &b, &c]);
                        for x in [a, b, c].iter() {
                            let mut mesh_vertex = MeshVertex::from_index(object, x);
                            if x.2.is_none() {
                                mesh_vertex.2 = normal.clone();
                            }
                            vertices.push(mesh_vertex);
                            indices.push(index as u32);
                            index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        material_index.len = index - material_index.start_index;
        material_indices.push(dbg!(material_index));
        println!("{}", indices.len());

        if vertices.is_empty() { return Err(error::Custom("Empty Object".to_string())); }

        let vs = shader::compile_shader(include_bytes!("vertex_mesh_layout.hlsl"), "vsmain", "vs_5_0")?;
        let vertex_buffer = device.new_vertex_buffer(&vertices, &vs)?;
        let index_buffer = device.new_index_buffer(&indices)?;

        Ok( Mesh(Arc::new(Mutex::new(MeshInner {
            vertices,
            vertex_buffer,
            indices,
            index_buffer,
            material_indices,
        }))))
    }
}

impl Mesh {
    pub fn inner(&self) -> MutexGuard<MeshInner> {
        self.0.lock().unwrap()
    }
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Mesh {}

#[derive(Clone, Debug)]
pub struct MaterialIndex {
    pub start_index: usize,
    pub len: usize,
    pub material_name: Option<String>,
}

fn load_material<P: AsRef<Path>>(path: P) -> error::Result<mtl::MtlSet> {
    let mut string = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut string)?;
    Ok(mtl::parse(&string)?)
}

//needed for custom derive
use crate::{self as engine};
#[derive(Debug, Vertex)]
#[repr(C)]
pub struct MeshVertex(vertex::Position, vertex::TexCoord, vertex::Normal);

impl MeshVertex {
    fn from_index(object: &obj::Object, index: &obj::VTNIndex) -> MeshVertex {
        let position = object.vertices[index.0].into();
        let texture = if let Some(tex_index) = index.1 {
            object.tex_vertices[tex_index].into()
        } else {
            [0.0, 0.0].into()
        };
        let normal = if let Some(norm_index) = index.2 {
            object.normals[norm_index].into()
        } else {
            [0.0, 0.0, 0.0].into()
        };

        MeshVertex(position, texture, normal)
    }
}

fn calc_normal(object: &obj::Object, indices: [&obj::VTNIndex;3]) -> vertex::Normal {
    let a: Vector3d = object.vertices[indices[0].0].into();
    let b: Vector3d = object.vertices[indices[1].0].into();
    let c: Vector3d = object.vertices[indices[2].0].into();
    (b-a.clone()).cross(c-a).normalize().into()
}

pub struct MeshInner {
    pub vertices: Vec<MeshVertex>,
    pub vertex_buffer: VertexBuffer<MeshVertex>,
    pub indices: Vec<u32>,
    pub index_buffer:  IndexBuffer,
    pub material_indices: Vec<MaterialIndex>
}

//TODO Verify
unsafe impl Send for MeshInner {}
unsafe impl Sync for MeshInner {}

impl Drop for MeshInner {
    fn drop(&mut self) {

    }
}
