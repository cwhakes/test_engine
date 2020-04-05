use crate::engine::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::shared::dxgi::{IDXGISwapChain, DXGI_SWAP_CHAIN_DESC};
use winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
use winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
use winapi::um::d3d11::{ID3D11Device, ID3D11RenderTargetView, ID3D11Resource, ID3D11Texture2D};
use winapi::Interface;

pub struct SwapChain {
    inner: NonNull<IDXGISwapChain>,
    back_buffer: NonNull<ID3D11RenderTargetView>,
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
        unsafe {
            let inner = NonNull::new(inner).unwrap();
            let mut buffer = ptr::null_mut();
            inner
                .as_ref()
                .GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut buffer);
            let buffer = buffer as *mut ID3D11Resource;

            let mut rtv = ptr::null_mut();
            device.CreateRenderTargetView(buffer, ptr::null_mut(), &mut rtv);
            let back_buffer = NonNull::new(rtv).unwrap();

            buffer.as_ref().unwrap().Release();

            SwapChain { inner, back_buffer }
        }
    }

    pub fn inner(&self) -> &IDXGISwapChain {
        unsafe { self.inner.as_ref() }
    }

    pub fn back_buffer_ptr(&self) -> *mut ID3D11RenderTargetView {
        self.back_buffer.as_ptr()
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
