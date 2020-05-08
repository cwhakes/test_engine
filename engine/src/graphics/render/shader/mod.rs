#[macro_use]
mod generate;

mod blob;
pub use blob::Blob;

use crate::prelude::*;
use crate::error;
use crate::graphics::render::{ConstantBuffer, Context, Device};
use crate::graphics::resource::texture::Texture;
use crate::util::{get_output, os_vec};

use std::ffi::CString;
use std::path::Path;
use std::ptr::{null, null_mut, NonNull};
use std::{convert, ops};

use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::d3d11;

/// Trait used to define new shaders.
/// It's a MadLibs trait, use to fill in some functions used throughout the render chain.
/// It obstensibly reduces boilerplate.
pub trait ShaderType {
    type ShaderInterface: ops::Deref<Target = d3d11::ID3D11DeviceChild>;

    fn create_shader(device: &Device, bytecode: &[u8]) -> error::Result<NonNull<Self::ShaderInterface>>;

    fn set_shader(context: &Context, shader: &mut Self::ShaderInterface);
    fn set_texture(context: &Context, texture: &mut Texture);

    fn set_constant_buffer<C>(context: &Context, index: u32, buffer: &mut ConstantBuffer<C>);

    const ENTRY_POINT: &'static str;
    const TARGET: &'static str;
}

shader_generate!( unsafe {
    Pixel,
    d3d11::ID3D11PixelShader,
    CreatePixelShader,
    PSSetShader,
    PSSetShaderResources,
    PSSetConstantBuffers,
    "psmain",
    "ps_5_0"
});

shader_generate!( unsafe {
    Vertex,
    d3d11::ID3D11VertexShader,
    CreateVertexShader,
    VSSetShader,
    VSSetShaderResources,
    VSSetConstantBuffers,
    "vsmain",
    "vs_5_0"
});

pub struct Shader<T: ShaderType> {
    pub shader: NonNull<T::ShaderInterface>,
}

//TODO FIXME Verify
unsafe impl<T> Send for Shader<T> where T: ShaderType + Send {}
unsafe impl<T> Sync for Shader<T> where T: ShaderType + Sync {}

impl<T: ShaderType> Shader<T> {
    pub fn new(device: &Device, location: impl AsRef<Path>) -> error::Result<(Shader<T>, Blob)> {
        let bytecode = compile_shader_from_location(location, T::ENTRY_POINT, T::TARGET)?;
        let shader = T::create_shader(device, &*bytecode)?;

        Ok((Shader { shader }, bytecode))
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

pub fn compile_shader_from_location(location: impl AsRef<Path>, entry_point: &str, target: &str) -> error::Result<Blob> {
    unsafe {
        let location = os_vec(location.as_ref().to_str().ok_or(error::Custom("Bad Path".to_owned()))?);
        let entry_point = CString::new(entry_point)
            .map_err(|_| error::Custom("Bad Entry Point".to_owned()))?;
        let target = CString::new(target)
            .map_err(|_| error::Custom("Bad Target".to_owned()))?;

        let mut blob = null_mut();
        let mut err_blob = null_mut();

        let result = D3DCompileFromFile(
            location.as_ptr(),
            null(),
            null_mut(),
            entry_point.as_ptr(),
            target.as_ptr(),
            0,
            0,
            &mut blob,
            &mut err_blob,
        ).result();

        result
            .and(Blob::new(blob))
            // use `.or_else()` to lazily evaluate
            .or_else(|_| Err(Blob::new(err_blob)?.into()))
    }
}
