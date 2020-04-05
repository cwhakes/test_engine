pub mod blob;
pub mod pixel;
pub mod shader;
pub mod vertex;

use blob::Blob;
pub use pixel::Pixel;
pub use shader::{Shader, ShaderType};
pub use vertex::Vertex;

use crate::util::os_vec;

use std::convert::TryInto;
use std::ffi::CString;
use std::ptr::{null, null_mut};

use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d3dcommon;
use winapi::um::d3dcompiler::D3DCompileFromFile;

pub fn compile_shader(location: &str, entry_point: &str, target: &str) -> Result<Blob, Blob> {
    unsafe {
        let location = os_vec(location);
        let entry_point = CString::new(entry_point).unwrap();
        let target = CString::new(target).unwrap();

        let mut blob: *mut d3dcommon::ID3DBlob = null_mut();
        let mut err_blob: *mut d3dcommon::ID3DBlob = null_mut();

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
        );

        if SUCCEEDED(result) {
            Ok(blob.try_into().unwrap())
        } else {
            Err(err_blob.try_into().unwrap())
        }
    }
}
