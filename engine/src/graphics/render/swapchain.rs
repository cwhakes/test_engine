use crate::prelude::*;

use super::Device;
use crate::error;
use crate::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::shared::dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgiformat::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN};
use winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
use winapi::um::d3d11::{ID3D11RenderTargetView, ID3D11Resource, ID3D11Texture2D};
use winapi::Interface;

pub struct SwapChain {
    inner: NonNull<IDXGISwapChain>,
    back_buffer: Option<BackBuffer>,
}

//TODO FIXME Verify
unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

impl SwapChain {
    pub fn get_desc(hwnd: &Hwnd) -> DXGI_SWAP_CHAIN_DESC {
        let (width, height) = hwnd.rect();

        let mut desc: DXGI_SWAP_CHAIN_DESC = Default::default();
        desc.BufferCount = 1;
        desc.BufferDesc.Width = width;
        desc.BufferDesc.Height = height;
        desc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
        desc.BufferDesc.RefreshRate.Numerator = 60;
        desc.BufferDesc.RefreshRate.Denominator = 1;
        desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
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

        let mut swapchain = SwapChain { inner, back_buffer: None };
        let back_buffer = BackBuffer::new(&swapchain, device)?;
        swapchain.back_buffer = Some(back_buffer);
        Ok(swapchain)
    }

    pub fn inner(&self) -> &IDXGISwapChain {
        unsafe { self.inner.as_ref() }
    }

    pub fn back_buffer_ptr(&self) -> Option<*mut ID3D11RenderTargetView> {
        self.back_buffer.as_ref().map(|bb| bb.0.as_ptr())
    }

    pub fn resize(&mut self, device: &Device) -> error::Result<()> {
        unsafe {
            self.back_buffer.take();
            self.inner().ResizeBuffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0).result()?;
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

struct BackBuffer(NonNull<ID3D11RenderTargetView>);

impl BackBuffer {
    fn new(swapchain: &SwapChain, device: &Device) -> error::Result<BackBuffer> {
        unsafe {
            let mut buffer = ptr::null_mut();
            swapchain.inner().GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut buffer).result()?;
            let buffer = buffer as *mut ID3D11Resource;

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
