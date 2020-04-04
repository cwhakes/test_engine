use crate::engine::graphics::GRAPHICS;

use std::ffi::c_void;

use winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
use winapi::shared::winerror::FAILED;
use winapi::um::d3d11;

pub struct VertexBuffer<V>
where
    V: Sized
{
    len: usize,
    buffer: *mut d3d11::ID3D11Buffer,
    layout: *mut d3d11::ID3D11InputLayout,
    _phantom: std::marker::PhantomData<V>,
}

//TODO FIXME Verify
unsafe impl<V> Send for VertexBuffer<V> where V: Send {}
unsafe impl<V> Sync for VertexBuffer<V> where V: Sync {}

impl<V> VertexBuffer<V> {
    pub fn new( vertices: &[V], shader_byte_code: *const c_void, shader_len: usize, ) -> VertexBuffer<V> {

        unsafe {
            let g = GRAPHICS.lock().unwrap();
            let g = g.as_ref().unwrap();

            if let Some(device) = g.device.as_ref() {

                let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
                buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
                buff_desc.ByteWidth = (vertices.len() * std::mem::size_of::<V>()) as u32;
                buff_desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
                buff_desc.CPUAccessFlags =0;
                buff_desc.MiscFlags = 0;
        
                let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
                data.pSysMem = vertices.as_ptr() as *const c_void;
        
                let mut buffer = std::ptr::null_mut();
        
                let res = device.CreateBuffer(
                    &buff_desc,
                    &data,
                    &mut buffer,
                );

                if FAILED(res) {
                    panic!();
                }

                let semantic_name = std::ffi::CString::new("POSITION").unwrap();

                let layout_desc = [
                    d3d11::D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: semantic_name.as_ptr(),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32B32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 0,
                        InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    }
                ];

                let mut layout = std::ptr::null_mut();

                let res = device.CreateInputLayout(
                    layout_desc.as_ptr(),
                    layout_desc.len() as u32,
                    shader_byte_code,
                    shader_len,
                    &mut layout,
                );

                if FAILED(res) {
                    panic!();
                }

                VertexBuffer {
                    len: vertices.len(),
                    buffer,
                    layout,
                    _phantom: Default::default(),
                }

            } else {
                panic!();
            }
        }
    }

    pub fn buffer(&self) -> *mut d3d11::ID3D11Buffer {
        self.buffer
    }

    pub fn layout(&self) -> *mut d3d11::ID3D11InputLayout {
        self.layout
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<V> Drop for VertexBuffer<V> {
    fn drop(&mut self) {
        unsafe {
            if let Some(buffer) = self.buffer.as_ref() {
                buffer.Release();
            }
            if let Some(layout) = self.layout.as_ref() {
                layout.Release();
            }
        }
    }
}