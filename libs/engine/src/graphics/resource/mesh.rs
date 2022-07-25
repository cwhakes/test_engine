use super::{shader, Resource, ResourceManager};

use crate::error;
use crate::graphics::render::{Device, IndexBuffer, VertexBuffer};
use crate::graphics::vertex;
use crate::math::{Matrix, Vector2d, Vector3d};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use log::warn;
use wavefront_obj::{mtl, obj};

pub type MeshManager = ResourceManager<Mesh>;

#[derive(Clone)]
pub struct Mesh(Arc<Mutex<MeshInner>>);

impl Resource for Mesh {
    fn load_resource_from_file(device: &Device, path: impl AsRef<Path>) -> error::Result<Self> {
        let mut file = File::open(path.as_ref())?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        let obj_set = obj::parse(&string)?;

        let mut material_map = MaterialMap(HashMap::new());
        if let Some(mtl_file) = obj_set.material_library.as_ref() {
            if let Ok(mtl_set) = load_material(path.as_ref().parent().unwrap().join(mtl_file)) {
                for (index, mtl) in mtl_set.materials.iter().enumerate() {
                    material_map.0.insert(mtl.name.clone(), index);
                }
            } else {
                warn!("Material not found for object: {}", path.as_ref().display());
                warn!(
                    "Looked for {}",
                    path.as_ref().parent().unwrap().join(mtl_file).display()
                );
            }
        }

        //Put in a vector, because you can't sort an iterator
        let mut geometries: Vec<_> = obj_set
            .objects
            .iter()
            //find the object start index, used for offsets
            .scan(0, |offset, object| {
                let old_offset = *offset;
                *offset += object.vertices.len();
                Some((old_offset, object))
            })
            .flat_map(|object| {
                object
                    .1
                    .geometry
                    .iter()
                    .map(move |geometry| (object, geometry))
            })
            .collect();

        // Sort geometries by material index and them by name if material index does not exist
        geometries.sort_by_key(|(_, geometry)| {
            let name = &geometry.material_name;
            let id = material_map.id_of(name);
            (id, name)
        });

        let mut indices = Vec::new();
        let mut vertices: Vec<MeshVertex> = obj_set
            .objects
            .iter()
            .flat_map(|object| object.vertices.iter())
            .map(MeshVertex::from_vertex)
            .collect();
        let mut vertex_metadata = vec![VertexMetadata::default(); vertices.len()];

        let mut material_id = MaterialId {
            id: material_map.id_of(&geometries[0].1.material_name),
            name: geometries[0].1.material_name.clone(),
            offset: 0,
            len: 0,
        };
        let mut material_ids = Vec::new();

        for ((offset, object), geometry) in &geometries {
            // if material name has changed, that is the end of geometries for that material because they are sorted by material
            // we can put it into material_ids
            if geometry.material_name != material_id.name {
                material_id.len = indices.len() - material_id.offset;

                let new_material_index = MaterialId {
                    id: material_map.id_of(&geometry.material_name),
                    name: geometry.material_name.clone(),
                    offset: indices.len(),
                    len: 0,
                };

                material_ids.push(mem::replace(&mut material_id, new_material_index));
            }

            for shape in &geometry.shapes {
                match shape.primitive {
                    obj::Primitive::Triangle(a, b, c) => {
                        //in case no normal exists
                        let normal = calc_normal(object, [&a, &b, &c]);
                        let (tangent, binormal) = calc_tangents(object, [&a, &b, &c])
                            .unwrap_or(([1.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into()));
                        for i_vtn in [a, b, c] {
                            let global_index = i_vtn.0 + offset;
                            let metadata = &mut vertex_metadata[global_index];
                            if metadata.finalized {
                                if metadata.i_tex == i_vtn.1 && metadata.i_nor == i_vtn.2 {
                                    indices.push(global_index as u32);
                                } else {
                                    vertices.push(MeshVertex::from_index(
                                        object,
                                        &i_vtn,
                                        tangent.clone(),
                                        binormal.clone(),
                                    ));
                                    indices.push((vertices.len() - 1) as u32);
                                }
                            } else {
                                let mesh_vertex = &mut vertices[global_index];
                                if let Some(i_tex) = i_vtn.1 {
                                    mesh_vertex.texture = object.tex_vertices[i_tex].into();
                                    metadata.i_tex = Some(i_tex);
                                } else {
                                    mesh_vertex.texture = [0.0, 0.0].into();
                                }
                                if let Some(i_nor) = i_vtn.2 {
                                    let normal: vertex::Normal = object.normals[i_nor].into();
                                    let binormal: vertex::BiNormal =
                                        normal.clone().cross(*tangent).into();
                                    let tangent = binormal.clone().cross(*normal).into();
                                    mesh_vertex.tangent = tangent;
                                    mesh_vertex.binormal = binormal;
                                    mesh_vertex.normal = normal;
                                    metadata.i_nor = Some(i_nor);
                                } else {
                                    mesh_vertex.tangent = tangent.clone();
                                    mesh_vertex.binormal = binormal.clone();
                                    mesh_vertex.normal = normal.clone();
                                }
                                metadata.finalized = true;
                                indices.push(global_index as u32);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        //We have to put the last material in the indices
        material_id.len = indices.len() - material_id.offset;
        material_ids.push(material_id);

        if vertices.is_empty() {
            return Err(error::Custom("Empty Object".to_string()));
        }

        let vs = shader::compile_shader(
            include_bytes!("vertex_mesh_layout.hlsl"),
            "vsmain",
            "vs_5_0",
        )?;
        let vertex_buffer = device.new_vertex_buffer(&vertices, &vs)?;
        let index_buffer = device.new_index_buffer(&indices)?;

        Ok(Self(Arc::new(Mutex::new(MeshInner {
            vertices,
            vertex_buffer,
            indices,
            index_buffer,
            material_ids,
        }))))
    }
}

fn load_material<P: AsRef<Path>>(path: P) -> error::Result<mtl::MtlSet> {
    let mut string = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut string)?;
    Ok(mtl::parse(&string)?)
}

fn calc_normal(object: &obj::Object, indices: [&obj::VTNIndex; 3]) -> vertex::Normal {
    let a: Vector3d = object.vertices[indices[0].0].into();
    let b: Vector3d = object.vertices[indices[1].0].into();
    let c: Vector3d = object.vertices[indices[2].0].into();
    (b - a).cross(c - a).normalize().into()
}

fn calc_tangents(
    object: &obj::Object,
    indices: [&obj::VTNIndex; 3],
) -> Option<(vertex::Tangent, vertex::BiNormal)> {
    let p0: Vector3d = object.vertices[indices[0].0].into();
    let p1: Vector3d = object.vertices[indices[1].0].into();
    let p2: Vector3d = object.vertices[indices[2].0].into();

    // Requires texture coordinates to work
    let t0: Vector2d = object.tex_vertices[indices[0].1?].into();
    let t1: Vector2d = object.tex_vertices[indices[1].1?].into();
    let t2: Vector2d = object.tex_vertices[indices[2].1?].into();

    let e0 = p1 - p0;
    let e1 = p2 - p0;
    let delta_t0 = t1 - t0;
    let delta_t1 = t2 - t0;

    let e = Matrix([e0.into(), e1.into()]);
    let delta_t = Matrix([delta_t0.into(), delta_t1.into()]);

    let Matrix([tangent, binormal]) = delta_t.inverse() * e;
    let tangent = Vector3d::from(tangent).normalize();
    let binormal = Vector3d::from(binormal).normalize();
    Some((tangent.into(), binormal.into()))
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

/// Used to track duplicate verticies
#[derive(Clone, Default)]
struct VertexMetadata {
    finalized: bool,
    i_tex: Option<obj::TextureIndex>,
    i_nor: Option<obj::NormalIndex>,
}

#[derive(Clone, Debug)]
pub struct MaterialId {
    pub id: usize,
    pub name: Option<String>,
    pub offset: usize,
    pub len: usize,
}
struct MaterialMap(HashMap<String, usize>);

impl MaterialMap {
    fn id_of(&self, name: &Option<String>) -> usize {
        name.as_ref()
            .and_then(|name| self.0.get(name))
            .copied()
            //Default ID to number of materials
            .unwrap_or_else(|| self.0.len())
    }
}

//needed for custom derive
use crate::{self as engine};
#[derive(Debug, Vertex)]
#[repr(C)]
pub struct MeshVertex {
    position: vertex::Position,
    texture: vertex::TexCoord,
    tangent: vertex::Tangent,
    binormal: vertex::BiNormal,
    normal: vertex::Normal,
}

impl MeshVertex {
    fn from_index(
        object: &obj::Object,
        index: &obj::VTNIndex,
        tangent: vertex::Tangent,
        binormal: vertex::BiNormal,
    ) -> Self {
        let position = object.vertices[index.0].into();
        let texture = index.1.map_or([0.0, 0.0].into(), |tex_index| {
            object.tex_vertices[tex_index].into()
        });
        let normal = index.2.map_or([0.0, 0.0, 0.0].into(), |norm_index| {
            object.normals[norm_index].into()
        });

        Self {
            position,
            texture,
            normal,
            tangent,
            binormal,
        }
    }

    fn from_vertex(vertex: &obj::Vertex) -> Self {
        let position = (*vertex).into();
        let texture = [0.0, 0.0].into();
        let tangent = [1.0, 0.0, 0.0].into();
        let binormal = [0.0, 1.0, 0.0].into();
        let normal = [0.0, 0.0, 1.0].into();
        Self {
            position,
            texture,
            tangent,
            binormal,
            normal,
        }
    }
}

pub struct MeshInner {
    pub vertices: Vec<MeshVertex>,
    pub vertex_buffer: VertexBuffer<MeshVertex>,
    pub indices: Vec<u32>,
    pub index_buffer: IndexBuffer,
    pub material_ids: Vec<MaterialId>,
}

//TODO Verify
unsafe impl Send for MeshInner {}
unsafe impl Sync for MeshInner {}

impl Drop for MeshInner {
    fn drop(&mut self) {}
}
