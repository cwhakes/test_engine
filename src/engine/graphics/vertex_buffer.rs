use crate::engine::graphics::GRAPHICS;
use crate::engine::vertex::{Vertex, SemanticIndexFix};

use std::ptr::{self, NonNull};

use winapi::shared::winerror::FAILED;
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
    pub fn new(vertices: &[V], bytecode: &[u8]) -> VertexBuffer<V> {
        unsafe {
            let g = GRAPHICS.lock().unwrap();
            let g = g.as_ref().unwrap();

            let device = g.device.as_ref();

            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = (vertices.len() * std::mem::size_of::<V>()) as u32;
            buff_desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = vertices.as_ptr() as *const _;

            let mut buffer = ptr::null_mut();

            let res = device.CreateBuffer(&buff_desc, &data, &mut buffer);

            if FAILED(res) {
                panic!();
            }

            let buffer = NonNull::new(buffer).unwrap();

            /*
            let layout_desc: Vec<d3d11::D3D11_INPUT_ELEMENT_DESC> = vertex::Position::desc(0, 0)
                .chain(vertex::Position::desc(12, 1))
                .chain(vertex::Color::desc(24, 0))
                .collect();*/
            
            let layout_desc: Vec<_> = V::desc(0)
                .semantic_index_fix()
                .collect();

            let mut layout = std::ptr::null_mut();

            let res = device.CreateInputLayout(
                layout_desc.as_ptr(),
                layout_desc.len() as u32,
                bytecode.as_ptr() as *const _,
                bytecode.len(),
                &mut layout,
            );

            if FAILED(res) {
                panic!();
            }

            let layout = NonNull::new(layout).unwrap();

            VertexBuffer {
                len: vertices.len(),
                buffer,
                layout,
                _phantom: Default::default(),
            }
        }
    }

    pub fn buffer_ptr(&self) -> *mut d3d11::ID3D11Buffer {
        self.buffer.as_ptr()
    }

    pub fn layout_ptr(&self) -> *mut d3d11::ID3D11InputLayout {
        self.layout.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
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
