use crate::prelude::*;

use super::Device;
use crate::error;
use crate::util::get_output;
use crate::window::Hwnd;

use std::ptr::{self, NonNull};

use winapi::shared::dxgi;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::minwindef;
use winapi::um::d3d11;
use winapi::um::wingdi;
use winapi::um::winuser;
use winapi::Interface;

pub struct SwapChain {
    inner: NonNull<dxgi::IDXGISwapChain>,
    back_buffer: Option<BackBuffer>,
    depth_buffer: Option<DepthBuffer>,
}

//TODO FIXME Verify
unsafe impl Send for SwapChain {}
unsafe impl Sync for SwapChain {}

impl SwapChain {
    pub fn get_desc(hwnd: &Hwnd) -> dxgi::DXGI_SWAP_CHAIN_DESC {
        let (width, height) = hwnd.rect();

        let mut desc = dxgi::DXGI_SWAP_CHAIN_DESC::default();
        desc.BufferCount = 1;
        desc.BufferDesc.Width = width;
        desc.BufferDesc.Height = height;
        desc.BufferDesc.Format = dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        desc.BufferUsage = dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
        desc.OutputWindow = *hwnd.inner();
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Flags = dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
        desc.Windowed = minwindef::TRUE;

        // unsafe {
        //     let mut dev_mode = wingdi::DEVMODEW::default();

        //     winuser::EnumDisplaySettingsW(
        //         ptr::null_mut(),
        //         winuser::ENUM_CURRENT_SETTINGS,
        //         &mut dev_mode);

        //     let refresh_rate = dev_mode.dmDisplayFrequency;
            
        //     desc.BufferDesc.RefreshRate.Numerator = refresh_rate;
        //     desc.BufferDesc.RefreshRate.Denominator = 1;
        // }

        desc
    }
    
    /// # Safety
    /// 
    /// `swapchain` must point to a valid IDXGISwapChain
    pub unsafe fn new(swapchain: NonNull<dxgi::IDXGISwapChain>, device: &Device) -> error::Result<SwapChain> {
        let inner = swapchain;
        let mut swapchain = SwapChain { inner, back_buffer: None, depth_buffer: None };
        let back_buffer = BackBuffer::new(&swapchain, device)?;
        swapchain.back_buffer = Some(back_buffer);
        let depth_buffer = DepthBuffer::new(&swapchain, device)?;
        swapchain.depth_buffer = Some(depth_buffer);

        Ok(swapchain)
    }

    fn inner(&self) -> &dxgi::IDXGISwapChain {
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
            self.depth_buffer.take();

            self.inner().ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0).result()?;

            self.back_buffer = Some(BackBuffer::new(self, device)?);
            self.depth_buffer = Some(DepthBuffer::new(self, device)?);

            Ok(())
        }
    }

    pub fn set_windowed_state(&mut self, device: &Device, state: WindowState) -> error::Result<()> {
        
        unsafe {
            //self.back_buffer.take();
            //self.depth_buffer.take();

            //self.inner().ResizeBuffers(1, 0, 0, dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, 0).result()?;

            let output = get_output(|ptr| {
                self.inner().GetContainingOutput(ptr)
            })?;

            match state {
                WindowState::Windowed => self.inner().SetFullscreenState(minwindef::FALSE, ptr::null_mut()).result().unwrap(),
                WindowState::Fullscreen => {
                    let inner = self.inner();
                    //let null = ptr::null_mut();
                    dbg!("heya");
                    let out = inner.SetFullscreenState(minwindef::TRUE, output.as_ptr());
                    println!("Output: {:?}", out);
                    out.result().unwrap()
                },
            };
            dbg!("heyc");

            self.back_buffer.take();
            self.depth_buffer.take();

            self.inner().ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0).result()?;

            self.back_buffer = Some(BackBuffer::new(self, device)?);
            self.depth_buffer = Some(DepthBuffer::new(self, device)?);

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

            let buffer = get_output(|ptr| {
                swapchain.inner().GetBuffer(0, &d3d11::ID3D11Texture2D::uuidof(), ptr)
            })?.cast::<d3d11::ID3D11Resource>();
            
            let rtv = get_output(|ptr| {
                device.as_ref().CreateRenderTargetView(buffer.as_ptr(), ptr::null_mut(), ptr)
            })?;

            buffer.as_ref().Release();

            Ok(BackBuffer(rtv))
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
            let mut sc_desc = dxgi::DXGI_SWAP_CHAIN_DESC::default();
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

            let buffer = get_output(|ptr| {
                device.as_ref().CreateTexture2D(&tex_desc, ptr::null_mut(), ptr)
            })?.cast::<d3d11::ID3D11Resource>();

            let dsv = get_output(|ptr| {
                device.as_ref().CreateDepthStencilView(buffer.as_ptr(), ptr::null_mut(), ptr)
            })?;

            buffer.as_ref().Release();

            Ok(DepthBuffer(dsv))
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WindowState {
    Windowed,
    Fullscreen,
}

impl WindowState {
    pub fn toggle(&mut self) {
        match self {
            WindowState::Windowed => *self = WindowState::Fullscreen,
            WindowState::Fullscreen => *self = WindowState::Windowed,
        }
    }
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState::Windowed
    }
}
