use crate::graphics::Device;

use std::ptr::{self, NonNull};

use winapi::shared::winerror::FAILED;
use winapi::um::d3d11;

pub struct IndexBuffer {
    len: usize,
    buffer: NonNull<d3d11::ID3D11Buffer>,
}

//TODO FIXME Verify
unsafe impl Send for IndexBuffer {}
unsafe impl Sync for IndexBuffer {}

impl IndexBuffer {
    pub fn new(device: &Device, indices: &[u32]) -> IndexBuffer {
        unsafe {
            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = (indices.len() * std::mem::size_of::<u32>()) as u32;
            buff_desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = indices.as_ptr() as *const _;

            let mut buffer = ptr::null_mut();

            let res = device.as_ref().CreateBuffer(&buff_desc, &data, &mut buffer);

            if FAILED(res) {
                panic!();
            }

            let buffer = NonNull::new(buffer).unwrap();

            IndexBuffer {
                len: indices.len(),
                buffer,
            }
        }
    }

    pub fn buffer_ptr(&self) -> *mut d3d11::ID3D11Buffer {
        self.buffer.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
        }
    }
}
