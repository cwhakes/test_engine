use super::{blob::Blob, compile_shader};

use crate::engine::graphics::device::Device;

use std::ptr::null_mut;

use winapi::um::d3d11;

pub struct PixelShader {
    pub pixel_shader: *mut d3d11::ID3D11PixelShader,
    pub blob: Blob,
}

impl PixelShader {
    pub fn new(device: &Device, location: &str) -> PixelShader {
        unsafe {
            let blob = compile_shader(location, "psmain", "ps_5_0").unwrap();

            let bytecode = blob.as_ref().GetBufferPointer();
            let bytecode_len = blob.as_ref().GetBufferSize();

            let mut pixel_shader:  *mut d3d11::ID3D11PixelShader = null_mut();

            device.as_ref().CreatePixelShader(bytecode, bytecode_len, null_mut(), &mut pixel_shader);

            PixelShader {
                pixel_shader,
                blob,
            }
        }
    }
}

impl Drop for PixelShader {
    fn drop(&mut self) {
        unsafe {
            if let Some(ps) = self.pixel_shader.as_ref() {
                ps.Release();        
            }
        }
    }
}