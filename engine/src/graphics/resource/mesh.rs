use super::{Resource, ResourceManager};

use crate::error;
use crate::graphics::render::{Device, IndexBuffer, shaders, VertexBuffer};
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
        for object in obj_set.objects.iter().take(1) {
            for geometry in object.geometry.iter() {
                let mut index = 0;

                for shape in geometry.shapes.iter() {
                    match shape.primitive {
                        obj::Primitive::Triangle(a, b, c) => {
                            for x in [a, b, c].iter() {
                                let position = object.vertices[x.0].into();
                                let texture = object.tex_vertices[x.1.unwrap()].into();
                                vertices.push(MeshVertex(position, texture));
                                indices.push(index as u32);
                                index += 1;
                            }
                        }
                        _ => {}
                    }
                }

            }
        }

        let vs = shaders::compile_shader("vertex_mesh_layout.hlsl", "vsmain", "vs_5_0")?;
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
#[derive(Vertex)]
#[repr(C)]
pub struct MeshVertex(vertex::Position, vertex::TexCoord);

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
