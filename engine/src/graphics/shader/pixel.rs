use super::shader::ShaderType;

use crate::graphics::constant_buffer::ConstantBuffer;

use std::ffi::c_void;
use std::ptr;

use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11;

pub enum Pixel {}

impl ShaderType for Pixel {
    type ShaderInterface = d3d11::ID3D11PixelShader;

    fn create_shader(
        device: &d3d11::ID3D11Device,
        bytecode: *const c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    ) {
        unsafe {
            device.CreatePixelShader(bytecode, bytecode_len, ptr::null_mut(), shader);
        }
    }

    fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: &mut Self::ShaderInterface) {
        unsafe { context.PSSetShader(shader, ptr::null(), 0) }
    }

    fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &ConstantBuffer<C>) {
        unsafe { context.PSSetConstantBuffers(0, 1, &buffer.buffer_ptr()) }
    }

    const ENTRY_POINT: &'static str = "psmain";
    const TARGET: &'static str = "ps_5_0";
}
