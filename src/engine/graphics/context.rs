use crate::engine::graphics::shader::{PixelShader, VertexShader};
use crate::engine::graphics::swapchain::SwapChain;
use crate::engine::graphics::vertex_buffer::VertexBuffer;

use std::ptr::NonNull;

use winapi::um::d3d11::{ID3D11DeviceContext, D3D11_VIEWPORT};
use winapi::um::d3dcommon;

pub struct Context(NonNull<ID3D11DeviceContext>);

impl Context {

    pub fn clear_render_target_color(
        &self,
        swapchain: &SwapChain,
        red: f32,
        grn: f32,
        blu: f32,
        alp: f32,
    ) {
        unsafe {
            self.as_ref().ClearRenderTargetView(swapchain.back_buffer(), &[red, grn, blu, alp]);
            self.as_ref().OMSetRenderTargets(1, &swapchain.back_buffer(), std::ptr::null_mut());
        }
    }

    pub fn set_vertex_buffer<V>(&self, vertex_buffer: &VertexBuffer<V>) {
        unsafe {
            self.as_ref().IASetVertexBuffers(0, 1, &vertex_buffer.buffer(), &(std::mem::size_of::<V>() as u32), &0);
            self.as_ref().IASetInputLayout(vertex_buffer.layout())
        }
    }

    pub fn set_vertex_shader(&self, shader: &VertexShader) {
        unsafe {
            self.as_ref().VSSetShader(shader.vertex_shader, std::ptr::null(), 0);
        }
    }

    pub fn set_pixel_shader(&self, shader: &PixelShader) {
        unsafe {
            self.as_ref().PSSetShader(shader.pixel_shader, std::ptr::null(), 0);
        }
    }

    pub fn _draw_triangle_list<V>(&self, vertices_len: usize, vertices_start: usize) {
        unsafe {
            self.as_ref().IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.as_ref().Draw(vertices_len as u32, vertices_start as u32);
        }
    }

    pub fn draw_triangle_strip<V>(&self, vertices_len: usize, vertices_start: usize) {
        unsafe {
            self.as_ref().IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            self.as_ref().Draw(vertices_len as u32, vertices_start as u32);
        }
    }

    pub fn set_viewport_size(&self, width: f32, height: f32) {
        unsafe {
            let mut vp = D3D11_VIEWPORT::default();
            vp.Width = width;
            vp.Height = height;
            vp.MinDepth = 0.0;
            vp.MaxDepth = 1.0;

            self.as_ref().RSSetViewports(1, &vp);
        }
    }
}

impl AsRef<ID3D11DeviceContext> for Context {
    fn as_ref(&self) -> &ID3D11DeviceContext {
        unsafe {
            self.0.as_ref()
        }        
    }
}

impl AsMut<ID3D11DeviceContext> for Context {
    fn as_mut(&mut self) -> &mut ID3D11DeviceContext {
        unsafe {
            self.0.as_mut()
        }        
    }
}

impl std::convert::TryFrom<*mut ID3D11DeviceContext> for Context {
    type Error = ();

    fn try_from(ptr: *mut ID3D11DeviceContext) -> Result<Self, Self::Error> {
        match NonNull::new(ptr) {
            Some(inner) => Ok(Context(inner)),
            None => Err(())
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
