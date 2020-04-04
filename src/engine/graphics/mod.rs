pub mod context;
pub mod swapchain;
pub mod vertex_buffer;

use crate::engine::window::Hwnd;
use crate::util::os_vec;
use context::Context;
use swapchain::SwapChain;

use std::convert::TryInto;
use std::ffi::{CString, c_void};
use std::ptr::{null, null_mut};
use std::sync::Mutex;
//use winapi::shared::dxgi::{DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use winapi::shared::winerror::{FAILED, SUCCEEDED};
use winapi::um::d3d11;
use winapi::um::d3dcommon;
use winapi::um::d3dcompiler::D3DCompileFromFile;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Option<Graphics>> = Mutex::new(None);
}

pub struct Graphics {
    device: *mut d3d11::ID3D11Device,
    feature_level: d3dcommon::D3D_FEATURE_LEVEL,
    context: Context,
    swapchain: SwapChain,
    vsblob: Option<*mut d3dcommon::ID3DBlob>,
    psblob: Option<*mut d3dcommon::ID3DBlob>,
    vertex_shader: Option<*mut d3d11::ID3D11VertexShader>,
    pixel_shader: Option<*mut d3d11::ID3D11PixelShader>,
}

//TODO FIXME verify we can do this
unsafe impl Send for Graphics {}
unsafe impl Sync for Graphics {}

impl Graphics {
    pub fn new(hwnd: &Hwnd) -> Graphics {
        unsafe {
            let driver_types = [
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                d3dcommon::D3D_DRIVER_TYPE_WARP,
                d3dcommon::D3D_DRIVER_TYPE_REFERENCE,
            ];

            let feature_levels = [d3dcommon::D3D_FEATURE_LEVEL_11_0];

            let swapchain_desc = SwapChain::get_desc(hwnd);

            let mut device = null_mut();
            let mut feature_level = Default::default();
            let mut context = null_mut();
            let mut swapchain = null_mut();

            let mut result = -1; //Default to error

            for &driver_type in driver_types.iter() {
                result = d3d11::D3D11CreateDeviceAndSwapChain(
                    null_mut(),
                    driver_type,
                    null_mut(),
                    0,
                    feature_levels.as_ptr(),
                    feature_levels.len() as u32,
                    d3d11::D3D11_SDK_VERSION,
                    &swapchain_desc,
                    &mut swapchain,
                    &mut device,
                    &mut feature_level,
                    &mut context,
                );

                if SUCCEEDED(result) {
                    break;
                }
            }

            if FAILED(result) {
                panic!();
            }

            let swapchain = SwapChain::new(swapchain, &*device);

            Graphics {
                device,
                feature_level,
                context: context.try_into().unwrap(),
                swapchain,
                vsblob: None,
                psblob: None,
                vertex_shader: None,
                pixel_shader: None,
            }
        }
    }

    pub fn immediate_context(&self) -> &Context {
        &self.context
    }

    pub fn swapchain(&self) -> &SwapChain {
        &self.swapchain
    }
/*
    pub fn create_vertex_buffer(&self) -> VertexBuffer<_> {

    }*/

    pub fn create_shaders(&mut self) {
        unsafe {
            let location = os_vec("shader.fx");
            let entry_point = CString::new("vsmain").unwrap();
            let target = CString::new("vs_5_0").unwrap();

            let mut vsblob:  *mut d3dcommon::ID3DBlob = null_mut();
            let mut err_blob: *mut d3dcommon::ID3DBlob = null_mut();

            D3DCompileFromFile(
                location.as_ptr(),
                null(),
                null_mut(),
                entry_point.as_ptr(),
                target.as_ptr(),
                0,
                0,
                &mut vsblob,
                &mut err_blob,
            );

            let entry_point = CString::new("psmain").unwrap();
            let target = CString::new("ps_5_0").unwrap();

            let mut psblob:  *mut d3dcommon::ID3DBlob = null_mut();
            let mut err_blob: *mut d3dcommon::ID3DBlob = null_mut();

            D3DCompileFromFile(
                location.as_ptr(),
                null(),
                null_mut(),
                entry_point.as_ptr(),
                target.as_ptr(),
                0,
                0,
                &mut psblob,
                &mut err_blob,
            );

            let mut vertex_shader:  *mut d3d11::ID3D11VertexShader = null_mut();

            self.device.as_ref().unwrap().CreateVertexShader(
                vsblob.as_ref().unwrap().GetBufferPointer(),
                vsblob.as_ref().unwrap().GetBufferSize(),
                null_mut(),
                &mut vertex_shader,
            );

            let mut pixel_shader:  *mut d3d11::ID3D11PixelShader = null_mut();

            self.device.as_ref().unwrap().CreatePixelShader(
                psblob.as_ref().unwrap().GetBufferPointer(),
                psblob.as_ref().unwrap().GetBufferSize(),
                null_mut(),
                &mut pixel_shader,
            );

            self.vsblob = Some(vsblob);
            self.psblob = Some(psblob);
            self.vertex_shader = Some(vertex_shader);
            self.pixel_shader = Some(pixel_shader);
        }
    }

    pub fn set_shaders(&self) {
        unsafe {
            self.context.as_ref().VSSetShader(self.vertex_shader.unwrap(), null(), 0);
            self.context.as_ref().PSSetShader(self.pixel_shader.unwrap(), null(), 0);
        }
    }

    pub fn get_shader_buffer_and_size(&self) -> (*const c_void, usize) {
        unsafe {
            let bytecode = self.vsblob.as_ref().unwrap().as_ref().unwrap().GetBufferPointer();
            let size = self.vsblob.as_ref().unwrap().as_ref().unwrap().GetBufferSize();
            (bytecode, size)
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        unsafe {
            if let Some(device) = self.device.as_ref() {
                device.Release();
            }
        }
    }
}
