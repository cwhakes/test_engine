use super::{blob::Blob, compile_shader};

use crate::engine::graphics::device::Device;

use std::ffi::c_void;
use std::ops::Deref;
use std::ptr::null_mut;

use winapi::um::d3d11::ID3D11DeviceChild;

use winapi::um::d3d11::ID3D11Device;
use winapi::um::d3d11::ID3D11DeviceContext;
use winapi::shared::basetsd::SIZE_T;

pub trait ShaderType {

    type ShaderInterface: Deref<Target = ID3D11DeviceChild>;

    fn create_shader(
        device: &ID3D11Device,
        bytecode: *const c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    );

    fn set_shader(
        context: &ID3D11DeviceContext,
        shader: *mut Self::ShaderInterface,
    );

    const ENTRY_POINT: &'static str;
    const TARGET: &'static str;
}

pub struct Shader<T: ShaderType> {
    pub shader: *mut T::ShaderInterface,
    pub blob: Blob,
}

impl<T: ShaderType> Shader<T> {
    pub fn new(device: &Device, location: &str) -> Shader<T> {
        unsafe {
            let blob = compile_shader(location, T::ENTRY_POINT, T::TARGET).unwrap();

            let bytecode = blob.as_ref().GetBufferPointer();
            let bytecode_len = blob.as_ref().GetBufferSize();

            let mut shader = null_mut();

            T::create_shader(device.as_ref(), bytecode, bytecode_len, &mut shader);

            Shader {
                shader,
                blob,
            }
        }
    }
}

impl<T: ShaderType> Drop for Shader<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(shader) = self.shader.as_ref() {
                shader.Release();
            }
        }
    }
}