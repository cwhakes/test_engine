use crate::error;
use crate::graphics::render::Device;
use crate::util::get_output;

use std::ptr::NonNull;

use winapi::um::d3d11;

pub struct IndexBuffer {
    len: usize,
    buffer: NonNull<d3d11::ID3D11Buffer>,
}

//TODO FIXME Verify
unsafe impl Send for IndexBuffer {}
unsafe impl Sync for IndexBuffer {}

impl IndexBuffer {
    pub fn new(device: &Device, indices: &[u32]) -> error::Result<IndexBuffer> {
        unsafe {
            let buff_desc = d3d11::D3D11_BUFFER_DESC {
                Usage: d3d11::D3D11_USAGE_DEFAULT,
                ByteWidth: (indices.len() * std::mem::size_of::<u32>()) as u32,
                BindFlags: d3d11::D3D11_BIND_VERTEX_BUFFER,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                ..Default::default()
            };

            let data = d3d11::D3D11_SUBRESOURCE_DATA {
                pSysMem: indices.as_ptr() as *const _,
                ..Default::default()
            };

            let buffer = get_output(|ptr| device.as_ref().CreateBuffer(&buff_desc, &data, ptr))?;

            Ok(IndexBuffer {
                len: indices.len(),
                buffer,
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len
    }
}

impl AsRef<d3d11::ID3D11Buffer> for IndexBuffer {
    fn as_ref(&self) -> &d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_ref() }
    }
}

impl AsMut<d3d11::ID3D11Buffer> for IndexBuffer {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_mut() }
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
        }
    }
}
