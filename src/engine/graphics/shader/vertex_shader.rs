use super::{blob::Blob, compile_shader};

use crate::engine::graphics::device::Device;

use std::ptr::null_mut;

use winapi::um::d3d11;

pub struct VertexShader {
    pub vertex_shader: *mut d3d11::ID3D11VertexShader,
    pub blob: Blob,
}

impl VertexShader {
    pub fn new(device: &Device, location: &str) -> VertexShader {
        unsafe {
            let blob = compile_shader(location, "vsmain", "vs_5_0").unwrap();

            let bytecode = blob.as_ref().GetBufferPointer();
            let bytecode_len = blob.as_ref().GetBufferSize();

            let mut vertex_shader:  *mut d3d11::ID3D11VertexShader = null_mut();

            device.as_ref().CreateVertexShader(bytecode, bytecode_len, null_mut(), &mut vertex_shader);

            VertexShader {
                vertex_shader,
                blob,
            }
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            if let Some(vs) = self.vertex_shader.as_ref() {
                vs.Release();        
            }
        }
    }
}