use crate::error;
use crate::graphics::render::Device;
use crate::graphics::vertex::{SemanticIndexFix, Vertex};
use crate::util::get_output;

use std::ptr::NonNull;

use winapi::um::d3d11;

pub struct VertexBuffer<V: Vertex>
where
    V: Sized,
{
    len: usize,
    buffer: NonNull<d3d11::ID3D11Buffer>,
    layout: NonNull<d3d11::ID3D11InputLayout>,
    _phantom: std::marker::PhantomData<V>,
}

//TODO FIXME Verify
unsafe impl<V> Send for VertexBuffer<V> where V: Vertex + Send {}
unsafe impl<V> Sync for VertexBuffer<V> where V: Vertex + Sync {}

impl<V: Vertex> VertexBuffer<V> {
    pub fn new(device: &Device, vertices: &[V], bytecode: &[u8]) -> error::Result<Self> {
        unsafe {
            let buff_desc = d3d11::D3D11_BUFFER_DESC {
                Usage: d3d11::D3D11_USAGE_DEFAULT,
                ByteWidth: (vertices.len() * std::mem::size_of::<V>()) as u32,
                BindFlags: d3d11::D3D11_BIND_VERTEX_BUFFER,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                ..Default::default()
            };

            let data = d3d11::D3D11_SUBRESOURCE_DATA {
                pSysMem: vertices.as_ptr().cast(),
                ..Default::default()
            };

            let buffer = get_output(|ptr| device.as_ref().CreateBuffer(&buff_desc, &data, ptr))?;

            let layout_desc: Vec<_> = V::desc(0).semantic_index_fix().collect();

            let layout = get_output(|ptr| {
                device.as_ref().CreateInputLayout(
                    layout_desc.as_ptr(),
                    layout_desc.len() as u32,
                    bytecode.as_ptr().cast(),
                    bytecode.len(),
                    ptr,
                )
            })?;

            Ok(Self {
                len: vertices.len(),
                buffer,
                layout,
                _phantom: Default::default(),
            })
        }
    }

    pub fn buffer_ptr(&mut self) -> *mut d3d11::ID3D11Buffer {
        self.buffer.as_ptr()
    }

    pub fn layout_ptr(&mut self) -> *mut d3d11::ID3D11InputLayout {
        self.layout.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len
    }
}

impl<V: Vertex> AsRef<d3d11::ID3D11Buffer> for VertexBuffer<V> {
    fn as_ref(&self) -> &d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_ref() }
    }
}

impl<V: Vertex> AsMut<d3d11::ID3D11Buffer> for VertexBuffer<V> {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_mut() }
    }
}

impl<V: Vertex> AsRef<d3d11::ID3D11InputLayout> for VertexBuffer<V> {
    fn as_ref(&self) -> &d3d11::ID3D11InputLayout {
        unsafe { self.layout.as_ref() }
    }
}

impl<V: Vertex> AsMut<d3d11::ID3D11InputLayout> for VertexBuffer<V> {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11InputLayout {
        unsafe { self.layout.as_mut() }
    }
}

impl<V: Vertex> Drop for VertexBuffer<V> {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
            self.layout.as_ref().Release();
        }
    }
}
