use super::shader::{self, Shader, ShaderType};
use super::{ConstantBuffer, IndexBuffer, SwapChain, VertexBuffer};

use crate::error;
use crate::graphics::resource::mesh::Mesh;
use crate::graphics::resource::texture::Texture;
use crate::graphics::vertex::{Vertex, Color};

use std::ptr::NonNull;

use winapi::shared::dxgiformat;
use winapi::um::d3d11;
use winapi::um::d3dcommon;

pub struct Context(NonNull<d3d11::ID3D11DeviceContext>);

// https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-render-multi-thread-intro
unsafe impl Send for Context {}
//NOT SYNC

impl Context {
    /// # Safety
    /// 
    /// `context` must point to a valid ID3D11DeviceContext
    pub unsafe fn from_nonnull(context: NonNull<d3d11::ID3D11DeviceContext>) -> error::Result<Context> {
        Ok(Context(context))
    }

    pub fn clear_render_target_color(&self, swapchain: &mut SwapChain, color: impl Into<Color>) {
        let color = color.into();
        unsafe {
            if let Some(back_buffer) = swapchain.back_buffer_ptr() {
                if let Some(depth_buffer) = swapchain.depth_buffer_mut() {
                    self.as_ref().ClearRenderTargetView(back_buffer, &[color.x, color.y, color.z, 1.0]);
                    self.as_ref().ClearDepthStencilView(
                        depth_buffer.as_mut(),
                        d3d11::D3D11_CLEAR_DEPTH | d3d11::D3D11_CLEAR_STENCIL,
                        1.0,
                        0,
                    );
                    self.as_ref().OMSetRenderTargets(1, &back_buffer, depth_buffer.as_mut());
                }
            }
        }
    }

    pub fn set_constant_buffer<S: ShaderType, C: ?Sized>(&self, index: u32, buffer: &mut ConstantBuffer<C>) {
        S::set_constant_buffer(self, index, buffer)
    }

    pub fn set_index_buffer(&self, index_buffer: &mut IndexBuffer) {
        unsafe {
            self.as_ref().IASetIndexBuffer(
                index_buffer.as_mut(),
                dxgiformat::DXGI_FORMAT_R32_UINT,
                0,
            );
        }
    }

    pub fn set_vertex_buffer<V: Vertex>(&self, vertex_buffer: &mut VertexBuffer<V>) {
        unsafe {
            self.as_ref().IASetVertexBuffers(
                0,
                1,
                &vertex_buffer.buffer_ptr(),
                &(std::mem::size_of::<V>() as u32),
                &0,
            );
            self.as_ref().IASetInputLayout(vertex_buffer.as_mut())
        }
    }

    pub fn set_shader<S: ShaderType>(&self, shader: &mut Shader<S>) {
        S::set_shader(self, shader.as_mut());
    }

    pub fn set_textures<S: ShaderType>(&self, textures: &mut [Option<Texture>]) {
        S::set_textures(self, textures);
    }

    pub fn draw_triangle_list(&self, vertices_len: usize, vertices_start: usize) {
        unsafe {
            self.as_ref()
                .IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.as_ref()
                .Draw(vertices_len as u32, vertices_start as u32);
        }
    }

    pub fn draw_triangle_strip(&self, vertices_len: usize, vertices_start: usize) {
        unsafe {
            self.as_ref()
                .IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            self.as_ref()
                .Draw(vertices_len as u32, vertices_start as u32);
        }
    }

    pub fn draw_indexed_triangle_list(&self, indices_len: usize, indices_start: usize, vertices_offset: isize) {
        unsafe {
            self.as_ref()
                .IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.as_ref()
                .DrawIndexed(indices_len as u32, indices_start as u32, vertices_offset as i32);
        }
    }

    pub fn draw_mesh(&self, mesh: &Mesh) {
        self.set_vertex_buffer(&mut mesh.inner().vertex_buffer);
        self.set_index_buffer(&mut mesh.inner().index_buffer);
        self.draw_indexed_triangle_list(mesh.inner().index_buffer.len(), 0, 0);
    }

    pub fn draw_mesh_and_texture(
        &self,
        mesh: &Mesh,
        textures: &mut [Option<Texture>],
        vertex_shader: &mut Shader<shader::Vertex>,
        pixel_shader: &mut Shader<shader::Pixel>,
    ) {
        self.set_shader(vertex_shader);
        self.set_shader(pixel_shader);
        self.set_textures::<shader::Pixel>(textures);
        
        self.set_vertex_buffer(&mut mesh.inner().vertex_buffer);
        self.set_index_buffer(&mut mesh.inner().index_buffer);
        self.draw_indexed_triangle_list(mesh.inner().index_buffer.len(), 0, 0);
    }

    pub fn set_viewport_size(&self, width: f32, height: f32) {
        unsafe {
            let mut vp = d3d11::D3D11_VIEWPORT::default();
            vp.Width = width;
            vp.Height = height;
            vp.MinDepth = 0.0;
            vp.MaxDepth = 1.0;

            self.as_ref().RSSetViewports(1, &vp);
        }
    }
}

impl AsRef<d3d11::ID3D11DeviceContext> for Context {
    fn as_ref(&self) -> &d3d11::ID3D11DeviceContext {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<d3d11::ID3D11DeviceContext> for Context {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11DeviceContext {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
