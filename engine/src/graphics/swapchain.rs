use crate::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::shared::dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgiformat::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN};
use winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
use winapi::um::d3d11::{ID3D11Device, ID3D11RenderTargetView, ID3D11Resource, ID3D11Texture2D};
use winapi::Interface;

pub struct SwapChain {
    inner: NonNull<IDXGISwapChain>,
    back_buffer: Option<BackBuffer>,
}

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

    pub fn new(inner: *mut IDXGISwapChain, device: &ID3D11Device) -> SwapChain {
        let inner = NonNull::new(inner).unwrap();

        let mut swapchain = SwapChain { inner, back_buffer: None };
        let back_buffer = BackBuffer::new(&swapchain, device);
        swapchain.back_buffer = back_buffer;
        swapchain
    }

    pub fn inner(&self) -> &IDXGISwapChain {
        unsafe { self.inner.as_ref() }
    }

    pub fn back_buffer_ptr(&self) -> Option<*mut ID3D11RenderTargetView> {
        self.back_buffer.as_ref().map(|bb| bb.0.as_ptr())
    }

    pub fn resize(&mut self, device: &ID3D11Device) {
        unsafe {
            self.back_buffer.take();

            self.inner().ResizeBuffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0);

            self.back_buffer = BackBuffer::new(self, device);
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
    fn new(swapchain: &SwapChain, device: &ID3D11Device) -> Option<BackBuffer> {
        unsafe {
            let mut buffer = ptr::null_mut();
            swapchain.inner().GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut buffer);
            let buffer = buffer as *mut ID3D11Resource;

            let mut rtv = ptr::null_mut();
            device.CreateRenderTargetView(buffer, ptr::null_mut(), &mut rtv);
            if let Some(buffer) = buffer.as_ref() {
                buffer.Release();
            }
            NonNull::new(rtv).map(|inner| BackBuffer(inner))
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
