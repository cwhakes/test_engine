use super::Device;

use crate::error;
use crate::util::get_output;

use std::ptr::NonNull;

use winapi::shared::minwindef;
use winapi::um::d3d11;

pub struct RasterState(NonNull<d3d11::ID3D11RasterizerState>);

//TODO FIXME Verify
unsafe impl Send for RasterState {}
unsafe impl Sync for RasterState {}

impl RasterState {
    pub fn new_front(device: &Device) -> error::Result<RasterState> {
        unsafe {
            let mut desc = d3d11::D3D11_RASTERIZER_DESC::default();
            desc.CullMode = d3d11::D3D11_CULL_FRONT;
            desc.DepthClipEnable = minwindef::FALSE;
            desc.FillMode = d3d11::D3D11_FILL_SOLID;

            get_output(|ptr| device.as_ref().CreateRasterizerState(&desc, ptr)).map(RasterState)
        }
    }

    pub fn new_back(device: &Device) -> error::Result<RasterState> {
        unsafe {
            let mut desc = d3d11::D3D11_RASTERIZER_DESC::default();
            desc.CullMode = d3d11::D3D11_CULL_BACK;
            desc.DepthClipEnable = minwindef::TRUE;
            desc.FillMode = d3d11::D3D11_FILL_SOLID;

            get_output(|ptr| device.as_ref().CreateRasterizerState(&desc, ptr)).map(RasterState)
        }
    }
}

impl AsRef<d3d11::ID3D11RasterizerState> for RasterState {
    fn as_ref(&self) -> &d3d11::ID3D11RasterizerState {
        unsafe { self.0.as_ref() }
    }
}

impl AsMut<d3d11::ID3D11RasterizerState> for RasterState {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11RasterizerState {
        unsafe { self.0.as_mut() }
    }
}

impl Drop for RasterState {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
