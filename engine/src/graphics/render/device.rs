use crate::prelude::*;

use crate::error;
use crate::vertex::Vertex;
use crate::graphics::render::shaders::{Blob, Shader, ShaderType};
use crate::graphics::render::{ConstantBuffer, IndexBuffer, SwapChain, VertexBuffer};
use crate::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::um::d3d11sdklayers::{ID3D11Debug, D3D11_RLDO_DETAIL};
use winapi::shared::dxgi;
use winapi::um::d3d11::{ID3D11Device};

pub struct Device(NonNull<ID3D11Device>);

//TODO FIXME verify we can do this
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    /// # Safety
    /// 
    /// `device` must point to a valid ID3D11Device
    pub unsafe fn from_ptr(device: *mut ID3D11Device) -> error::Result<Device> {
        match NonNull::new(device) {
            Some(inner) => Ok(Device(inner)),
            None => Err(null_ptr_err!()),
        }
    }

    pub fn new_swapchain(&mut self, hwnd: &Hwnd) -> error::Result<SwapChain> {
        unsafe {
            let dxgi_device = self.as_ref().query_interface::<dxgi::IDXGIDevice>()?;
            let dxgi_adapter = dxgi_device.as_ref().get_parent::<dxgi::IDXGIAdapter>()?;
            let dxgi_factory = dxgi_adapter.as_ref().get_parent::<dxgi::IDXGIFactory>()?;
            
            let mut desc = SwapChain::get_desc(hwnd);
            let mut swapchain_ptr = ptr::null_mut();

            dxgi_factory.as_ref().CreateSwapChain(&**self.as_mut() as *const _ as *mut _, &mut desc, &mut swapchain_ptr).result()?;

            SwapChain::new(swapchain_ptr, self)
        }
    }

    pub fn new_constant_buffer<C>(&self, constant: &C) -> error::Result<ConstantBuffer<C>> {
        ConstantBuffer::new(self, constant)
    }

    pub fn new_index_buffer(&self, indices: &[u32]) -> error::Result<IndexBuffer> {
        IndexBuffer::new(self, indices)
    }

    pub fn new_shader<T: ShaderType>(&self, location: &str) -> error::Result<(Shader<T>, Blob)> {
        Shader::<T>::new(self, location)
    }

    pub fn new_vertex_buffer<V: Vertex>(&self, vertices: &[V], bytecode: &[u8]) -> error::Result<VertexBuffer<V>> {
        VertexBuffer::new(self, vertices, bytecode)
    }

    pub fn debug(&self) -> error::Result<()> {
        unsafe {
            let debug = self.as_ref().query_interface::<ID3D11Debug>()?;
            debug.as_ref().ReportLiveDeviceObjects(D3D11_RLDO_DETAIL).result()?;
            Ok(())
        }
    }
}

impl AsRef<ID3D11Device> for Device {
    fn as_ref(&self) -> &ID3D11Device {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<ID3D11Device> for Device {
    fn as_mut(&mut self) -> &mut ID3D11Device {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
