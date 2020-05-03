use crate::prelude::*;

use super::Device;
use crate::error;
use crate::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::shared::dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::um::d3d11;
use winapi::Interface;

pub struct SwapChain {
    inner: NonNull<IDXGISwapChain>,
    back_buffer: Option<BackBuffer>,
    depth_buffer: Option<DepthBuffer>,
}

//TODO FIXME Verify
unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

impl SwapChain {
    pub fn get_desc(hwnd: &Hwnd) -> DXGI_SWAP_CHAIN_DESC {
        let (width, height) = hwnd.rect();

        let mut desc = DXGI_SWAP_CHAIN_DESC::default();
        desc.BufferCount = 1;
        desc.BufferDesc.Width = width;
        desc.BufferDesc.Height = height;
        desc.BufferDesc.Format = dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        desc.BufferDesc.RefreshRate.Numerator = 60;
        desc.BufferDesc.RefreshRate.Denominator = 1;
        desc.BufferUsage = dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
        desc.OutputWindow = *hwnd.inner();
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Windowed = winapi::shared::minwindef::TRUE;

        desc
    }
    
    /// # Safety
    /// 
    /// `swapchain` must point to a valid IDXGISwapChain
    pub unsafe fn new(swapchain: *mut IDXGISwapChain, device: &Device) -> error::Result<SwapChain> {
        let inner = NonNull::new(swapchain).ok_or(null_ptr_err!())?;

        let mut swapchain = SwapChain { inner, back_buffer: None, depth_buffer: None };
        let back_buffer = BackBuffer::new(&swapchain, device)?;
        swapchain.back_buffer = Some(back_buffer);
        let depth_buffer = DepthBuffer::new(&swapchain, device)?;
        swapchain.depth_buffer = Some(depth_buffer);

        Ok(swapchain)
    }

    fn inner(&self) -> &IDXGISwapChain {
        unsafe { self.inner.as_ref() }
    }

    pub fn back_buffer_ptr(&self) -> Option<*mut d3d11::ID3D11RenderTargetView> {
        self.back_buffer.as_ref().map(|bb| bb.0.as_ptr())
    }

    pub fn depth_buffer(&self) -> Option<&DepthBuffer> {
        self.depth_buffer.as_ref()
    }

    pub fn depth_buffer_mut(&mut self) -> Option<&mut DepthBuffer> {
        self.depth_buffer.as_mut()
    }

    pub fn resize(&mut self, device: &Device) -> error::Result<()> {
        unsafe {
            self.back_buffer.take();
            self.inner().ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0).result()?;
            self.back_buffer = Some(BackBuffer::new(self, device)?);
            Ok(())
        }
    }

    pub fn present(&self, vsync: u32) {
        unsafe {
            self.inner().Present(vsync, 0);
        }
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            self.inner().Release();
        }
    }
}

struct BackBuffer(NonNull<d3d11::ID3D11RenderTargetView>);

impl BackBuffer {
    fn new(swapchain: &SwapChain, device: &Device) -> error::Result<BackBuffer> {
        unsafe {
            let mut buffer = ptr::null_mut();
            swapchain.inner().GetBuffer(0, &d3d11::ID3D11Texture2D::uuidof(), &mut buffer).result()?;
            let buffer = buffer as *mut d3d11::ID3D11Resource;

            let mut rtv = ptr::null_mut();
            device.as_ref().CreateRenderTargetView(buffer, ptr::null_mut(), &mut rtv).result()?;
            if let Some(buffer) = buffer.as_ref() {
                buffer.Release();
            }
            NonNull::new(rtv).map(BackBuffer).ok_or(null_ptr_err!())
        }
    }
}

impl Drop for BackBuffer {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().Release();
        }
    }
}

pub struct DepthBuffer(NonNull<d3d11::ID3D11DepthStencilView>);

impl DepthBuffer {
    fn new(swapchain: &SwapChain, device: &Device) -> error::Result<DepthBuffer> {
        unsafe {
            let mut sc_desc = DXGI_SWAP_CHAIN_DESC::default();
            swapchain.inner.as_ref().GetDesc(&mut sc_desc);
    
            let mut tex_desc = d3d11::D3D11_TEXTURE2D_DESC::default();
            tex_desc.Width = sc_desc.BufferDesc.Width;
            tex_desc.Height = sc_desc.BufferDesc.Height;
            tex_desc.MipLevels = 1;
            tex_desc.ArraySize = 1;
            tex_desc.Format = dxgiformat::DXGI_FORMAT_D24_UNORM_S8_UINT;
            tex_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            tex_desc.SampleDesc.Count = 1;
            tex_desc.SampleDesc.Quality = 0;
            tex_desc.BindFlags = d3d11::D3D11_BIND_DEPTH_STENCIL;
            tex_desc.CPUAccessFlags = 0;
            tex_desc.MiscFlags = 0;
    
            let mut buffer = ptr::null_mut();
            device.as_ref().CreateTexture2D(&tex_desc, ptr::null_mut(), &mut buffer).result()?;
            let buffer = buffer as *mut d3d11::ID3D11Resource;

            let mut dsv = ptr::null_mut();
            device.as_ref().CreateDepthStencilView(buffer, ptr::null_mut(), &mut dsv).result()?;
            if let Some(buffer) = buffer.as_ref() {
                buffer.Release();
            }
            NonNull::new(dsv).map(DepthBuffer).ok_or(null_ptr_err!())
        }
    }
}

impl AsRef<d3d11::ID3D11DepthStencilView> for DepthBuffer {
    fn as_ref(&self) -> &d3d11::ID3D11DepthStencilView {
        unsafe {
            self.0.as_ref()
        }
    }
}

impl AsMut<d3d11::ID3D11DepthStencilView> for DepthBuffer {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11DepthStencilView {
        unsafe {
            self.0.as_mut()
        }
    }
}

impl Drop for DepthBuffer {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().Release();
        }
    }
}
