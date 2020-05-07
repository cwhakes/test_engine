#[macro_use]
mod generate;

mod blob;
mod shader;

pub use blob::Blob;
pub use shader::{Shader, ShaderType};

use crate::prelude::*;
use crate::error;
use crate::util::os_vec;

use std::ffi::CString;
use std::ptr::{null, null_mut};

use winapi::um::d3dcompiler::D3DCompileFromFile;
use winapi::um::d3d11;

shader_generate!(Pixel, d3d11::ID3D11PixelShader,
    CreatePixelShader,
    PSSetShader,
    PSSetShaderResources,
    PSSetConstantBuffers,
    "psmain",
    "ps_5_0"
);

shader_generate!(Vertex, d3d11::ID3D11VertexShader,
    CreateVertexShader,
    VSSetShader,
    VSSetShaderResources,
    VSSetConstantBuffers,
    "vsmain",
    "vs_5_0"
);

pub fn compile_shader(location: &str, entry_point: &str, target: &str) -> error::Result<Blob> {
    unsafe {
        let location = os_vec(location);
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
