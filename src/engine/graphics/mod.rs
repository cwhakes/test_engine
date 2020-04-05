pub mod context;
pub mod device;
pub mod shader;
pub mod swapchain;
pub mod vertex_buffer;

use crate::engine::window::Hwnd;

use context::Context;
use device::Device;
use shader::Shader;
use swapchain::SwapChain;

use std::convert::TryInto;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::sync::Mutex;
//use winapi::shared::dxgi::{DXGI_SWAP_CHAIN_DESC, IDXGISwapChain};
use winapi::shared::winerror::{FAILED, SUCCEEDED};
use winapi::um::d3d11;
use winapi::um::d3dcommon;

lazy_static! {
    pub static ref GRAPHICS: Mutex<Option<Graphics>> = Mutex::new(None);
}

pub struct Graphics {
    device: Device,
    _feature_level: d3dcommon::D3D_FEATURE_LEVEL,
    context: Context,
    swapchain: SwapChain,
    vertex_shader: Option<Shader<shader::Vertex>>,
    pixel_shader: Option<Shader<shader::Pixel>>,
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
                device: device.try_into().unwrap(),
                _feature_level: feature_level,
                context: context.try_into().unwrap(),
                swapchain,
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

    pub fn vertex_shader(&self) -> Option<&Shader<shader::Vertex>> {
        self.vertex_shader.as_ref()
    }

    pub fn pixel_shader(&self) -> Option<&Shader<shader::Pixel>> {
        self.pixel_shader.as_ref()
    }

    pub fn create_vertex_shader(&mut self, location: &str) {
        let vertex_shader = Shader::<shader::Vertex>::new(&self.device, location);
        self.vertex_shader = Some(vertex_shader);
    }

    pub fn create_pixel_shader(&mut self, location: &str) {
        let pixel_shader = Shader::<shader::Pixel>::new(&self.device, location);
        self.pixel_shader = Some(pixel_shader);
    }

    pub fn set_shaders(&self) {
        self.context.set_shader(self.vertex_shader().unwrap());
        self.context.set_shader(self.pixel_shader().unwrap());
    }

    pub fn get_shader_buffer_and_size(&self) -> (*const c_void, usize) {
        unsafe {
            let bytecode = self
                .vertex_shader
                .as_ref()
                .unwrap()
                .blob
                .as_ref()
                .GetBufferPointer();
            let size = self
                .vertex_shader
                .as_ref()
                .unwrap()
                .blob
                .as_ref()
                .GetBufferSize();

            (bytecode, size)
        }
    }
}
