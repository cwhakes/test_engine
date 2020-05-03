use super::shader::ShaderType;

use crate::graphics::render::ConstantBuffer;
use crate::graphics::resource::texture::Texture;

use std::ffi::c_void;
use std::ptr;

use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11;

pub enum Pixel {}

impl ShaderType for Pixel {
    type ShaderInterface = d3d11::ID3D11PixelShader;

    unsafe fn create_shader(
        device: &d3d11::ID3D11Device,
        bytecode: *const c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    ) {
        device.CreatePixelShader(bytecode, bytecode_len, ptr::null_mut(), shader);
    }

    fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: &mut Self::ShaderInterface) {
        unsafe { context.PSSetShader(shader, ptr::null(), 0) }
    }

    fn set_texture(context: &d3d11::ID3D11DeviceContext, texture: &mut Texture) {
        unsafe {
            context.PSSetShaderResources(0, 1, &texture.resource_view_ptr());
        }
    }

    fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &mut ConstantBuffer<C>) {
        unsafe { context.PSSetConstantBuffers(0, 1, &buffer.buffer_ptr()) }
    }

    const ENTRY_POINT: &'static str = "psmain";
    const TARGET: &'static str = "ps_5_0";
}