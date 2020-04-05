mod constant_buffer;
mod context;
mod device;
pub mod shader;
mod swapchain;
mod vertex_buffer;

pub use constant_buffer::ConstantBuffer;
pub use context::Context;
pub use device::Device;
pub use swapchain::SwapChain;
pub use vertex_buffer::VertexBuffer;

use super::window;

use std::convert::TryInto;
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
}

//TODO FIXME verify we can do this
unsafe impl Send for Graphics {}
unsafe impl Sync for Graphics {}

impl Graphics {
    pub fn new(hwnd: &window::Hwnd) -> Graphics {
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
            }
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn immediate_context(&self) -> &Context {
        &self.context
    }

    pub fn swapchain(&self) -> &SwapChain {
        &self.swapchain
    }
}
