mod constant_buffer;
mod context;
mod device;
mod index_buffer;
pub mod shader;
mod swapchain;
mod vertex_buffer;

pub use constant_buffer::ConstantBuffer;
pub use context::Context;
pub use device::Device;
pub use index_buffer::IndexBuffer;
pub use swapchain::SwapChain;
pub use vertex_buffer::VertexBuffer;

use crate::error;
use crate::util::get_output2;

use std::ptr::null_mut;
use winapi::um::{d3d11, d3dcommon};

pub struct Render {
    device: Device,
    _feature_level: d3dcommon::D3D_FEATURE_LEVEL,
    context: Context,
}

const DRIVER_TYPES: [d3dcommon::D3D_DRIVER_TYPE; 3] = [
    d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
    d3dcommon::D3D_DRIVER_TYPE_WARP,
    d3dcommon::D3D_DRIVER_TYPE_REFERENCE,
];

const FEATURE_LEVELS: [d3dcommon::D3D_FEATURE_LEVEL; 1] = [
    d3dcommon::D3D_FEATURE_LEVEL_11_0
];

impl Render {
    pub fn new() -> error::Result<Render> {
        unsafe {
            let mut feature_level = Default::default();
            //Default to error
            let mut result = Err(error::Custom("No driver types specified".to_string()));

            for &driver_type in DRIVER_TYPES.iter() {
                result = get_output2(|ptr1, ptr2| {
                    d3d11::D3D11CreateDevice(
                        null_mut(),
                        driver_type,
                        null_mut(),
                        d3d11::D3D11_CREATE_DEVICE_DEBUG,
                        FEATURE_LEVELS.as_ptr(),
                        FEATURE_LEVELS.len() as u32,
                        d3d11::D3D11_SDK_VERSION,
                        ptr1,
                        &mut feature_level,
                        ptr2,
                    )
                });

                if result.is_ok() {
                    break;
                }
            }
            let (device, context) = result?;

            Ok(Render {
                device: Device::from_nonnull(device)?,
                _feature_level: feature_level,
                context: Context::from_nonnull(context)?,
            })
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut Device {
        &mut self.device
    }

    pub fn immediate_context(&self) -> &Context {
        &self.context
    }
}
