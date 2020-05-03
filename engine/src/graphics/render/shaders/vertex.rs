use super::shader::ShaderType;

use crate::graphics::render::constant_buffer::ConstantBuffer;
use crate::graphics::resource::texture::Texture;

use std::ffi::c_void;
use std::ptr;

use winapi::shared::basetsd::SIZE_T;
use winapi::um::d3d11;

pub enum Vertex {}

impl ShaderType for Vertex {
    type ShaderInterface = d3d11::ID3D11VertexShader;

    unsafe fn create_shader(
        device: &d3d11::ID3D11Device,
        bytecode: *const c_void,
        bytecode_len: SIZE_T,
        shader: *mut *mut Self::ShaderInterface,
    ) {
        device.CreateVertexShader(bytecode, bytecode_len, ptr::null_mut(), shader);
    }

    fn set_shader(context: &d3d11::ID3D11DeviceContext, shader: &mut Self::ShaderInterface) {
        unsafe { context.VSSetShader(shader, ptr::null(), 0) }
    }

    fn set_texture(context: &d3d11::ID3D11DeviceContext, texture: &mut Texture) {
        unsafe {
            context.VSSetShaderResources(0, 1, &texture.resource_view_ptr());
        }
    }

    fn set_constant_buffer<C>(context: &d3d11::ID3D11DeviceContext, buffer: &mut ConstantBuffer<C>) {
        unsafe { context.VSSetConstantBuffers(0, 1, &buffer.buffer_ptr()) }
    }

    const ENTRY_POINT: &'static str = "vsmain";
    const TARGET: &'static str = "vs_5_0";
}