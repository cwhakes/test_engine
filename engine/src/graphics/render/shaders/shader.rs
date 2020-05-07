use super::{blob::Blob, compile_shader};

use crate::error;
use crate::graphics::render::{ConstantBuffer, Context, Device};
use crate::graphics::resource::texture::Texture;

use std::ptr::NonNull;
use std::{convert, ops};

use winapi::um::d3d11;

/// Trait used to define new shaders.
/// It's a MadLibs trait, use to fill in some functions used throughout the render chain.
/// It obstensibly reduces boilerplate.
pub trait ShaderType {
    type ShaderInterface: ops::Deref<Target = d3d11::ID3D11DeviceChild>;

    /// # Safety
    /// 
    /// Inherits safety of concrete function
    unsafe fn create_shader(device: &Device, bytecode: &[u8]) -> error::Result<*mut Self::ShaderInterface>;

    fn set_shader(context: &Context, shader: &mut Self::ShaderInterface);
    fn set_texture(context: &Context, texture: &mut Texture);

    fn set_constant_buffer<C>(context: &Context, buffer: &mut ConstantBuffer<C>);

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
            let bytecode = compile_shader(location, T::ENTRY_POINT, T::TARGET)?;
            let shader = T::create_shader(device, &*bytecode)?;
            let shader = NonNull::new(shader).ok_or(null_ptr_err!())?;

            Ok((Shader { shader }, bytecode))
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
