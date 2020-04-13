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
use winapi::Interface;

pub struct Device(NonNull<ID3D11Device>);

impl Device {
    /// # Safety
    /// 
    /// `device` must point to a valid ID3D11Device
    pub unsafe fn new(device: *mut ID3D11Device) -> error::Result<Device> {
        match NonNull::new(device) {
            Some(inner) => Ok(Device(inner)),
            None => Err(null_ptr_err!()),
        }
    }

    pub fn new_swapchain(&mut self, hwnd: &Hwnd) -> error::Result<SwapChain> {
        unsafe {
            let mut dxgi_device = ptr::null_mut();
            self.as_ref().QueryInterface(&dxgi::IDXGIDevice::uuidof(), &mut dxgi_device).result()?;
            let dxgi_device = NonNull::new(dxgi_device as *mut dxgi::IDXGIDevice).ok_or(null_ptr_err!())?;

            let mut dxgi_adapter = ptr::null_mut();
            dxgi_device.as_ref().GetParent(&dxgi::IDXGIAdapter::uuidof(), &mut dxgi_adapter).result()?;
            let dxgi_adapter = NonNull::new(dxgi_adapter as *mut dxgi::IDXGIAdapter).ok_or(null_ptr_err!())?;

            let mut dxgi_factory = ptr::null_mut();
            dxgi_adapter.as_ref().GetParent(&dxgi::IDXGIFactory::uuidof(), &mut dxgi_factory).result()?;
            let dxgi_factory = NonNull::new(dxgi_factory as *mut dxgi::IDXGIFactory).ok_or(null_ptr_err!())?;

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
            let mut debug = ptr::null_mut();
            self.as_ref().QueryInterface(&ID3D11Debug::uuidof(), &mut debug).result()?;
            let debug = NonNull::new(debug as *mut ID3D11Debug).ok_or(null_ptr_err!())?;
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
