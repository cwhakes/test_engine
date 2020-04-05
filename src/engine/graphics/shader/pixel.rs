use super::shader::ShaderType;

use std::ffi::c_void;

use winapi::um::d3d11::ID3D11Device;
use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11::ID3D11ClassLinkage;
use winapi::um::winnt::HRESULT;

use winapi::um::d3d11::ID3D11PixelShader;

pub enum Pixel {}

impl ShaderType for Pixel {

    type ShaderInterface = ID3D11PixelShader;

    #[allow(non_snake_case)]
    unsafe fn create_shader(
        device: &ID3D11Device,
        pShaderBytecode: *const c_void,
        BytecodeLength: SIZE_T,
        pClassLinkage: *mut ID3D11ClassLinkage,
        ppShader: *mut *mut Self::ShaderInterface,
    ) -> HRESULT {
        winapi::um::d3d11::ID3D11Device::CreatePixelShader(
            device,
            pShaderBytecode,
            BytecodeLength,
            pClassLinkage,
            ppShader,
        )
    }

    const ENTRY_POINT: &'static str = "psmain";
    const TARGET: &'static str = "ps_5_0";
}
