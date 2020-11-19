use crate::prelude::*;

use crate::error;
use crate::graphics::render::{ConstantBuffer, IndexBuffer, SwapChain, VertexBuffer};
use crate::graphics::vertex::Vertex;
use crate::util::get_output;
use crate::window::Hwnd;

use std::ptr::NonNull;

use winapi::um::d3d11sdklayers::{ID3D11Debug, D3D11_RLDO_DETAIL};
use winapi::shared::dxgi;
use winapi::um::d3d11;

pub struct Device(NonNull<d3d11::ID3D11Device>);

// https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-render-multi-thread-intro
unsafe impl Send for Device {}
unsafe impl Sync for Device {}

impl Device {
    /// # Safety
    /// 
    /// `device` must point to a valid ID3D11Device
    pub unsafe fn from_nonnull(device: NonNull<d3d11::ID3D11Device>) -> error::Result<Device> {
        Ok(Device(device))
    }

    pub fn new_swapchain(&mut self, hwnd: &Hwnd) -> error::Result<SwapChain> {
        unsafe {
            let dxgi_device = self.as_ref().query_interface::<dxgi::IDXGIDevice>()?;
            let dxgi_adapter = dxgi_device.as_ref().get_parent::<dxgi::IDXGIAdapter>()?;
            let dxgi_factory = dxgi_adapter.as_ref().get_parent::<dxgi::IDXGIFactory>()?;
            
            let mut desc = SwapChain::get_desc(hwnd);

            let swapchain = get_output(|ptr| {
                dxgi_factory.as_ref().CreateSwapChain(&**self.as_mut() as *const _ as *mut _, &mut desc, ptr)
            })?;

            SwapChain::new(swapchain, self)
        }
    }

    pub fn new_constant_buffer<C: ?Sized>(&self, constant: &mut C) -> error::Result<ConstantBuffer<C>> {
        ConstantBuffer::new(self, constant)
    }

    pub fn new_index_buffer(&self, indices: &[u32]) -> error::Result<IndexBuffer> {
        IndexBuffer::new(self, indices)
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

impl AsRef<d3d11::ID3D11Device> for Device {
    fn as_ref(&self) -> &d3d11::ID3D11Device {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<d3d11::ID3D11Device> for Device {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11Device {
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
