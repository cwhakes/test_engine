use crate::vertex::Vertex;
use crate::graphics::shader::{Blob, Shader, ShaderType};
use crate::graphics::{ConstantBuffer, IndexBuffer, VertexBuffer};

use std::ptr::NonNull;

use winapi::um::d3d11::ID3D11Device;

pub struct Device(NonNull<ID3D11Device>);

impl Device {
    pub fn new_constant_buffer<C>(&self, constant: &C) -> ConstantBuffer<C> {
        ConstantBuffer::new(self, constant)
    }

    pub fn new_index_buffer(&self, indices: &[u32]) -> IndexBuffer {
        IndexBuffer::new(self, indices)
    }

    pub fn new_shader<T: ShaderType>(&self, location: &str) -> (Shader<T>, Blob) {
        Shader::<T>::new(self, location)
    }

    pub fn new_vertex_buffer<V: Vertex>(&self, vertices: &[V], bytecode: &[u8]) -> VertexBuffer<V> {
        VertexBuffer::new(self, vertices, bytecode)
    }
}

impl AsRef<ID3D11Device> for Device {
    fn as_ref(&self) -> &ID3D11Device {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<ID3D11Device> for Device {
    fn as_mut(&mut self) -> &mut ID3D11Device {
        unsafe { self.0.as_mut() }
    }
}

impl std::convert::TryFrom<*mut ID3D11Device> for Device {
    type Error = ();

    fn try_from(ptr: *mut ID3D11Device) -> Result<Self, Self::Error> {
        match NonNull::new(ptr) {
            Some(inner) => Ok(Device(inner)),
            None => Err(()),
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
