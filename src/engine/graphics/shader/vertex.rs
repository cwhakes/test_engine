use super::shader::ShaderType;

use std::ffi::c_void;

use winapi::um::d3d11::ID3D11Device;
use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11::ID3D11ClassLinkage;
use winapi::um::winnt::HRESULT;

use winapi::um::d3d11::ID3D11VertexShader;

pub enum Vertex {}

impl ShaderType for Vertex {

    type ShaderInterface = ID3D11VertexShader;

    #[allow(non_snake_case)]
    unsafe fn create_shader(
        device: &ID3D11Device,
        pShaderBytecode: *const c_void,
        BytecodeLength: SIZE_T,
        pClassLinkage: *mut ID3D11ClassLinkage,
        ppVertexShader: *mut *mut Self::ShaderInterface,
    ) -> HRESULT {
        winapi::um::d3d11::ID3D11Device::CreateVertexShader(
            device,
            pShaderBytecode,
            BytecodeLength,
            pClassLinkage,
            ppVertexShader,
        )
    }

    const ENTRY_POINT: &'static str = "vsmain";
    const TARGET: &'static str = "vs_5_0";
}
