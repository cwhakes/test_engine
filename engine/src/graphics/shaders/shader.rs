use super::{blob::Blob, compile_shader};

use crate::error;
use crate::graphics::constant_buffer::ConstantBuffer;
use crate::graphics::device::Device;

use std::ptr::{self, NonNull};
use std::{convert, ffi, ops};

use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11;

/// Trait used to define new shaders.
/// It's a MadLibs trait, use to fill in some functions used throughout the render chain.
/// It obstensibly reduces boilerplate.
pub trait ShaderType {
    type ShaderInterface: ops::Deref<Target = d3d11::ID3D11DeviceChild>;

    /// # Safety
    /// 
    /// Inherits safety of concrete function
    unsafe fn create_shader(
        device: &d3d11::ID3D11Device,
        bytecode: *const ffi::c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    );

    fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: &mut Self::ShaderInterface);

    fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &mut ConstantBuffer<C>);

    const ENTRY_POINT: &'static str;
    const TARGET: &'static str;
}

pub struct Shader<T: ShaderType> {
    pub shader: NonNull<T::ShaderInterface>,
}

//TODO FIXME Verify
unsafe impl<T> Send for Shader<T> where T: ShaderType + Send {}
unsafe impl<T> Sync for Shader<T> where T: ShaderType + Sync {}

impl<T: ShaderType> Shader<T> {
    pub fn new(device: &Device, location: &str) -> error::Result<(Shader<T>, Blob)> {
        unsafe {
            let blob = compile_shader(location, T::ENTRY_POINT, T::TARGET).unwrap();

            let bytecode = blob.as_ref().GetBufferPointer();
            let bytecode_len = blob.as_ref().GetBufferSize();

            let mut shader = ptr::null_mut();
            T::create_shader(device.as_ref(), bytecode, bytecode_len, &mut shader);
            let shader = NonNull::new(shader).ok_or(error::NullPointer)?;

            Ok((Shader { shader }, blob))
        }
    }
}

impl<T: ShaderType> convert::AsRef<T::ShaderInterface> for Shader<T> {
    fn as_ref(&self) -> &T::ShaderInterface {
        unsafe { self.shader.as_ref() }
    }
}

impl<T: ShaderType> convert::AsMut<T::ShaderInterface> for Shader<T> {
    fn as_mut(&mut self) -> &mut T::ShaderInterface {
        unsafe { self.shader.as_mut() }
    }
}

impl<T: ShaderType> ops::Drop for Shader<T> {
    fn drop(&mut self) {
        unsafe {
            self.shader.as_ref().Release();
        }
    }
}
