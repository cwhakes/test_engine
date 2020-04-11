use crate::prelude::*;

use crate::graphics::render::Device;
use crate::error;
use crate::vertex::{Vertex, SemanticIndexFix};

use std::ptr::{self, NonNull};

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
    pub fn new(device: &Device, vertices: &[V], bytecode: &[u8]) -> error::Result<VertexBuffer<V>> {
        unsafe {
            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = (vertices.len() * std::mem::size_of::<V>()) as u32;
            buff_desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = vertices.as_ptr() as *const _;

            let mut buffer = ptr::null_mut();

            device.as_ref().CreateBuffer(&buff_desc, &data, &mut buffer).result()?;

            let buffer = NonNull::new(buffer).ok_or(null_ptr_err!())?;
            
            let layout_desc: Vec<_> = V::desc(0)
                .semantic_index_fix()
                .collect();

            let mut layout = std::ptr::null_mut();

            device.as_ref().CreateInputLayout(
                layout_desc.as_ptr(),
                layout_desc.len() as u32,
                bytecode.as_ptr() as *const _,
                bytecode.len(),
                &mut layout,
            ).result()?;


            let layout = NonNull::new(layout).ok_or(null_ptr_err!())?;

            Ok(VertexBuffer {
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
