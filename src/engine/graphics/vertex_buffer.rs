use crate::engine::graphics::GRAPHICS;

use std::ffi::CString;
use std::ptr::{self, NonNull};

use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::shared::winerror::FAILED;
use winapi::um::d3d11;

pub struct VertexBuffer<V>
where
    V: Sized,
{
    len: usize,
    buffer: NonNull<d3d11::ID3D11Buffer>,
    layout: NonNull<d3d11::ID3D11InputLayout>,
    _phantom: std::marker::PhantomData<V>,
}

//TODO FIXME Verify
unsafe impl<V> Send for VertexBuffer<V> where V: Send {}
unsafe impl<V> Sync for VertexBuffer<V> where V: Sync {}

impl<V> VertexBuffer<V> {
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

            let semantic_name_pos = CString::new("POSITION").unwrap();
            let semantic_name_col = CString::new("COLOR").unwrap();

            let layout_desc = [
                d3d11::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: semantic_name_pos.as_ptr(),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                d3d11::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: semantic_name_pos.as_ptr(),
                    SemanticIndex: 1,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 12,
                    InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                d3d11::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: semantic_name_col.as_ptr(),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 24,
                    InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];

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

impl<V> Drop for VertexBuffer<V> {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
            self.layout.as_ref().Release();
        }
    }
}
