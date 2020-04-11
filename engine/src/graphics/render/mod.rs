mod constant_buffer;
mod context;
mod device;
mod index_buffer;
pub mod shaders;
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
use winapi::um::d3d11;
use winapi::um::d3dcommon;

pub struct Render {
    device: Device,
    _feature_level: d3dcommon::D3D_FEATURE_LEVEL,
    context: Context,
}

//TODO FIXME verify we can do this
unsafe impl Send for Render {}
unsafe impl Sync for Render {}

impl Render {
    pub fn new() -> error::Result<Render> {
        unsafe {
            let driver_types = [
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                d3dcommon::D3D_DRIVER_TYPE_WARP,
                d3dcommon::D3D_DRIVER_TYPE_REFERENCE,
            ];

            let feature_levels = [d3dcommon::D3D_FEATURE_LEVEL_11_0];

            let mut device = null_mut();
            let mut feature_level = Default::default();
            let mut context = null_mut();
            let mut result = Err(error::HResult(-1)); //Default to error

            for &driver_type in driver_types.iter() {
                result = d3d11::D3D11CreateDevice(
                    null_mut(),
                    driver_type,
                    null_mut(),
                    0,
                    feature_levels.as_ptr(),
                    feature_levels.len() as u32,
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

            let device = Device::new(device)?;
            let context = Context::new(context)?;

            Ok(Render {
                device,
                _feature_level: feature_level,
                context,
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
