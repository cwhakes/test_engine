use crate::prelude::*;

use super::{Device, Target as RenderTarget};
use crate::error;
use crate::util::get_output;
use crate::window::Hwnd;

use std::ptr::{self, NonNull};
use std::u32;

use winapi::shared::dxgi;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::minwindef;
use winapi::um::d3d11;
use winapi::Interface;

pub struct SwapChain {
    inner: SwapChainInner,
    back_buffer: Option<BackBuffer>,
    depth_buffer: Option<DepthBuffer>,
}

impl SwapChain {
    pub fn get_desc(hwnd: &Hwnd) -> dxgi::DXGI_SWAP_CHAIN_DESC {
        let (width, height) = hwnd.rect().dims();

        dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferCount: 1,
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: width as u32,
                Height: height as u32,
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                ..dxgitype::DXGI_MODE_DESC::default()
            },
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: *hwnd.inner(),
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Flags: dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            Windowed: minwindef::TRUE,

            ..Default::default()
        }
    }

    /// # Safety
    ///
    /// `swapchain` must point to a valid `IDXGISwapChain`
    pub unsafe fn new(
        swapchain: NonNull<dxgi::IDXGISwapChain>,
        device: &Device,
    ) -> error::Result<Self> {
        let inner = SwapChainInner::new(swapchain);
        let mut swapchain = Self {
            inner,
            back_buffer: None,
            depth_buffer: None,
        };
        let back_buffer = BackBuffer::new(&swapchain, device)?;
        swapchain.back_buffer = Some(back_buffer);
        let depth_buffer = DepthBuffer::new(&swapchain, device)?;
        swapchain.depth_buffer = Some(depth_buffer);

        Ok(swapchain)
    }

    fn inner(&self) -> &dxgi::IDXGISwapChain {
        self.inner.as_ref()
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
            self.depth_buffer.take();

            self.inner()
                .ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0)
                .result()?;

            self.back_buffer = Some(BackBuffer::new(self, device)?);
            self.depth_buffer = Some(DepthBuffer::new(self, device)?);

            Ok(())
        }
    }

    pub fn set_windowed_state(&mut self, device: &Device, state: WindowState) -> error::Result<()> {
        unsafe {
            let output = get_output(|ptr| self.inner().GetContainingOutput(ptr))?;

            match state {
                WindowState::Windowed => self
                    .inner()
                    .SetFullscreenState(minwindef::FALSE, ptr::null_mut())
                    .result()
                    .unwrap(),
                WindowState::Fullscreen => {
                    let inner = self.inner();
                    //let null = ptr::null_mut();
                    let out = inner.SetFullscreenState(minwindef::TRUE, output.as_ptr());
                    out.result().unwrap()
                }
            };

            self.resize(device)?;

            Ok(())
        }
    }

    pub fn present(&self, vsync: u32) {
        unsafe {
            self.inner().Present(vsync, 0);
        }
    }
}

impl RenderTarget for SwapChain {
    fn render_target_view(&self) -> error::Result<*mut d3d11::ID3D11RenderTargetView> {
        self.back_buffer
            .as_ref()
            .map(|bb| bb.0.as_ptr())
            .ok_or("No bac kbuffer".into())
    }

    fn depth_stencil_view(&self) -> error::Result<*mut d3d11::ID3D11DepthStencilView> {
        self.depth_buffer
            .as_ref()
            .map(|db| db.0.as_ptr())
            .ok_or("No depth buffer".into())
    }
}

struct SwapChainInner(NonNull<dxgi::IDXGISwapChain>);

//TODO FIXME Verify
unsafe impl Send for SwapChainInner {}
unsafe impl Sync for SwapChainInner {}

impl SwapChainInner {
    unsafe fn new(swapchain: NonNull<dxgi::IDXGISwapChain>) -> Self {
        Self(swapchain)
    }
}

impl AsRef<dxgi::IDXGISwapChain> for SwapChainInner {
    fn as_ref(&self) -> &dxgi::IDXGISwapChain {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<dxgi::IDXGISwapChain> for SwapChainInner {
    fn as_mut(&mut self) -> &mut dxgi::IDXGISwapChain {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for SwapChainInner {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().Release();
        }
    }
}

struct BackBuffer(NonNull<d3d11::ID3D11RenderTargetView>);

//TODO FIXME Verify
unsafe impl Send for BackBuffer {}
unsafe impl Sync for BackBuffer {}

impl BackBuffer {
    fn new(swapchain: &SwapChain, device: &Device) -> error::Result<Self> {
        unsafe {
            let buffer = get_output(|ptr| {
                swapchain
                    .inner()
                    .GetBuffer(0, &d3d11::ID3D11Texture2D::uuidof(), ptr)
            })?
            .cast::<d3d11::ID3D11Resource>();

            let rtv = get_output(|ptr| {
                device
                    .as_ref()
                    .CreateRenderTargetView(buffer.as_ptr(), ptr::null_mut(), ptr)
            })?;

            buffer.as_ref().Release();

            Ok(Self(rtv))
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

//TODO FIXME Verify
unsafe impl Send for DepthBuffer {}
unsafe impl Sync for DepthBuffer {}

impl DepthBuffer {
    fn new(swapchain: &SwapChain, device: &Device) -> error::Result<Self> {
        unsafe {
            let mut sc_desc = dxgi::DXGI_SWAP_CHAIN_DESC::default();
            swapchain.inner.as_ref().GetDesc(&mut sc_desc);

            let tex_desc = d3d11::D3D11_TEXTURE2D_DESC {
                Width: sc_desc.BufferDesc.Width,
                Height: sc_desc.BufferDesc.Height,
                MipLevels: 1,
                ArraySize: 1,
                Format: dxgiformat::DXGI_FORMAT_D24_UNORM_S8_UINT,
                Usage: d3d11::D3D11_USAGE_DEFAULT,

                SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },

                BindFlags: d3d11::D3D11_BIND_DEPTH_STENCIL,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };

            let buffer = get_output(|ptr| {
                device
                    .as_ref()
                    .CreateTexture2D(&tex_desc, ptr::null_mut(), ptr)
            })?
            .cast::<d3d11::ID3D11Resource>();

            let dsv = get_output(|ptr| {
                device
                    .as_ref()
                    .CreateDepthStencilView(buffer.as_ptr(), ptr::null_mut(), ptr)
            })?;

            buffer.as_ref().Release();

            Ok(Self(dsv))
        }
    }
}

impl AsRef<d3d11::ID3D11DepthStencilView> for DepthBuffer {
    fn as_ref(&self) -> &d3d11::ID3D11DepthStencilView {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<d3d11::ID3D11DepthStencilView> for DepthBuffer {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11DepthStencilView {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for DepthBuffer {
    fn drop(&mut self) {
        unsafe {
            self.0.as_ref().Release();
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowState {
    Windowed,
    Fullscreen,
}

impl WindowState {
    pub fn toggle(&mut self) {
        match self {
            Self::Windowed => *self = Self::Fullscreen,
            Self::Fullscreen => *self = Self::Windowed,
        }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self::Windowed
    }
}
