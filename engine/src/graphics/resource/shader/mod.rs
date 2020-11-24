#[macro_use]
mod generate;

mod blob;
pub use blob::Blob;

use super::{Resource, ResourceManager};

use crate::error;
use crate::graphics::render::{ConstantBuffer, Context, Device};
use crate::graphics::resource::texture::Texture;
use crate::prelude::*;
use crate::util::get_output;

use std::ffi::CString;
use std::path::Path;
use std::ptr::{null, null_mut, NonNull};
use std::sync::Arc;
use std::{convert, fs, ops};

use winapi::um::d3d11;
use winapi::um::d3dcompiler;

/// Trait used to define new shaders.
/// It's a MadLibs trait, use to fill in some functions used throughout the render chain.
/// It obstensibly reduces boilerplate.
pub trait ShaderType {
    type ShaderInterface: ops::Deref<Target = d3d11::ID3D11DeviceChild>;

    fn create_shader(
        device: &Device,
        bytecode: &[u8],
    ) -> error::Result<NonNull<Self::ShaderInterface>>;

    fn set_shader(context: &Context, shader: &Self::ShaderInterface);
    fn set_textures(context: &Context, textures: &mut [Option<Texture>]);

    fn set_constant_buffer<C: ?Sized>(
        context: &Context,
        index: u32,
        buffer: &mut ConstantBuffer<C>,
    );

    const ENTRY_POINT: &'static str;
    const TARGET: &'static str;
}
pub type ShaderManager<T> = ResourceManager<Shader<T>>;

#[derive(Clone)]
pub struct Shader<T: ShaderType>(Arc<ShaderInner<T>>);

pub struct ShaderInner<T: ShaderType> {
    pub shader: NonNull<T::ShaderInterface>,
}

impl<T: ShaderType + Clone> Resource for Shader<T> {
    fn load_resource_from_file(device: &Device, path: impl AsRef<Path>) -> error::Result<Self> {
        let (inner, _) = ShaderInner::new(device, path)?;
        Ok(Shader(Arc::new(inner)))
    }
}

impl<T: ShaderType> ops::Deref for Shader<T> {
    type Target = ShaderInner<T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

shader_generate!( unsafe {
    Pixel,
    d3d11::ID3D11PixelShader,
    CreatePixelShader,
    PSSetShader,
    PSSetShaderResources,
    PSSetSamplers,
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
    VSSetSamplers,
    VSSetConstantBuffers,
    "vsmain",
    "vs_5_0"
});

//Should be safe, per here https://www.youtube.com/watch?v=kvuiADqIdck&t=1980
unsafe impl<T> Send for ShaderInner<T> where T: ShaderType + Send {}
unsafe impl<T> Sync for ShaderInner<T> where T: ShaderType + Sync {}

impl<T: ShaderType> ShaderInner<T> {
    pub fn new(
        device: &Device,
        location: impl AsRef<Path>,
    ) -> error::Result<(ShaderInner<T>, Blob)> {
        let bytecode = compile_shader_from_location(location, T::ENTRY_POINT, T::TARGET)?;
        let shader = T::create_shader(device, &*bytecode)?;

        Ok((ShaderInner { shader }, bytecode))
    }
}

impl<T: ShaderType> convert::AsRef<T::ShaderInterface> for ShaderInner<T> {
    fn as_ref(&self) -> &T::ShaderInterface {
        unsafe { self.shader.as_ref() }
    }
}

impl<T: ShaderType> convert::AsMut<T::ShaderInterface> for ShaderInner<T> {
    fn as_mut(&mut self) -> &mut T::ShaderInterface {
        unsafe { self.shader.as_mut() }
    }
}

impl<T: ShaderType> ops::Drop for ShaderInner<T> {
    fn drop(&mut self) {
        unsafe {
            self.shader.as_ref().Release();
        }
    }
}

pub fn compile_shader_from_location(
    location: impl AsRef<Path>,
    entry_point: &str,
    target: &str,
) -> error::Result<Blob> {
    let uncompiled = fs::read(location)?;
    compile_shader(&uncompiled, entry_point, target)
}

pub fn compile_shader(uncompiled: &[u8], entry_point: &str, target: &str) -> error::Result<Blob> {
    unsafe {
        let entry_point =
            CString::new(entry_point).map_err(|_| error::Custom("Bad Entry Point".to_owned()))?;
        let target = CString::new(target).map_err(|_| error::Custom("Bad Target".to_owned()))?;

        let mut blob = null_mut();
        let mut err_blob = null_mut();

        let result = d3dcompiler::D3DCompile(
            uncompiled.as_ptr() as *const _,
            uncompiled.len(),
            null_mut(),
            null(),
            null_mut(),
            entry_point.as_ptr(),
            target.as_ptr(),
            0,
            0,
            &mut blob,
            &mut err_blob,
        )
        .result();

        result
            .and(Blob::new(blob))
            // use `.or_else()` to lazily evaluate
            .or_else(|_| Err(Blob::new(err_blob)?.into()))
    }
}
