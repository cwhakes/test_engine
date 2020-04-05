use super::{blob::Blob, compile_shader};

use crate::engine::graphics::constant_buffer::ConstantBuffer;
use crate::engine::graphics::device::Device;

use std::ffi::c_void;
use std::ops::Deref;
use std::ptr::null_mut;

use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11;

pub trait ShaderType {
    type ShaderInterface: Deref<Target = d3d11::ID3D11DeviceChild>;

    fn create_shader(
        device: &d3d11::ID3D11Device,
        bytecode: *const c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    );

    fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: *mut Self::ShaderInterface);

    fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &ConstantBuffer<C>);

    const ENTRY_POINT: &'static str;
    const TARGET: &'static str;
}

pub struct Shader<T: ShaderType> {
    pub shader: *mut T::ShaderInterface,
}

impl<T: ShaderType> Shader<T> {
    pub fn new(device: &Device, location: &str) -> (Shader<T>, Blob) {
        unsafe {
            let blob = compile_shader(location, T::ENTRY_POINT, T::TARGET).unwrap();

            let bytecode = blob.as_ref().GetBufferPointer();
            let bytecode_len = blob.as_ref().GetBufferSize();

            let mut shader = null_mut();

            T::create_shader(device.as_ref(), bytecode, bytecode_len, &mut shader);

            (Shader { shader }, blob)
        }
    }
}

//TODO FIXME Verify
unsafe impl<T> Send for Shader<T> where T: ShaderType + Send {}
unsafe impl<T> Sync for Shader<T> where T: ShaderType + Sync {}

impl<T: ShaderType> Drop for Shader<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(shader) = self.shader.as_ref() {
                shader.Release();
            }
        }
    }
}
