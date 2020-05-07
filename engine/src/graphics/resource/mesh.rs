use super::{Resource, ResourceManager};

use crate::error;
use crate::graphics::render::{Device, IndexBuffer, shader, VertexBuffer};
use crate::math::Vector3d;
use crate::vertex;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};

use wavefront_obj::obj;

pub type MeshManager = ResourceManager<Mesh>;

#[derive(Clone)]
pub struct Mesh(Arc<Mutex<MeshInner>>);

impl Resource for Mesh {
    fn load_resource_from_file(device: &Device, path: &Path) -> error::Result<Self> {
        let mut file = File::open(path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        let obj_set = obj::parse(string)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        //TODO handle multiple models
        for object in obj_set.objects.iter() {
            for geometry in object.geometry.iter() {
                let mut index = 0;
                
                for shape in geometry.shapes.iter() {
                    match shape.primitive {
                        obj::Primitive::Triangle(a, b, c) => {
                            for x in [a, b, c].iter() {
                                vertices.push(MeshVertex::from_index(object, x));
                                indices.push(index as u32);
                                index += 1;
                            }
                        }
                        _ => {}
                    }
                }

            }
        }

        if vertices.is_empty() { return Err(error::Custom("Empty Object".to_string())); }

        let vs = shader::compile_shader("vertex_mesh_layout.hlsl", "vsmain", "vs_5_0")?;
        let vertex_buffer = device.new_vertex_buffer(&vertices, &vs)?;
        let index_buffer = device.new_index_buffer(&indices)?;

        Ok( Mesh(Arc::new(Mutex::new(MeshInner {
            vertices,
            vertex_buffer,
            indices,
            index_buffer,
        }))))
    }
}

impl Mesh {
    pub fn inner(&self) -> MutexGuard<MeshInner> {
        self.0.lock().unwrap()
    }
}

//needed for custom derive
use crate::{self as engine};
#[derive(Debug, Vertex)]
#[repr(C)]
pub struct MeshVertex(vertex::Position, vertex::TexCoord, vertex::Normal);

impl MeshVertex {
    fn from_index(object: &obj::Object, index: &obj::VTNIndex) -> MeshVertex {
        let position: Vector3d = object.vertices[index.0].into();
        let position = position.to_4d(1.0).into();
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

pub struct MeshInner {
    pub vertices: Vec<MeshVertex>,
    pub vertex_buffer: VertexBuffer<MeshVertex>,
    pub indices: Vec<u32>,
    pub index_buffer:  IndexBuffer,
}

//TODO Verify
unsafe impl Send for MeshInner {}
unsafe impl Sync for MeshInner {}

impl Drop for MeshInner {
    fn drop(&mut self) {

    }
}
