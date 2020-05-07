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

use crate::prelude::*;

use crate::error;

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
            let mut device = null_mut();
            let mut feature_level = Default::default();
            let mut context = null_mut();
            //Default to error
            let mut result = Err(error::Custom("No driver types specified".to_string()));

            for &driver_type in DRIVER_TYPES.iter() {
                result = d3d11::D3D11CreateDevice(
                    null_mut(),
                    driver_type,
                    null_mut(),
                    d3d11::D3D11_CREATE_DEVICE_DEBUG,
                    FEATURE_LEVELS.as_ptr(),
                    FEATURE_LEVELS.len() as u32,
                    d3d11::D3D11_SDK_VERSION,
                    &mut device,
                    &mut feature_level,
                    &mut context,
                ).result();

                if result.is_ok() {
                    break;
                }
            }
            result?;

            Ok(Render {
                device: Device::from_ptr(device)?,
                _feature_level: feature_level,
                context: Context::from_ptr(context)?,
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
