use crate::error;
use winapi::um::d3d11::{ID3D11DepthStencilView, ID3D11RenderTargetView};

pub trait Target {
    fn render_target_view(&self) -> error::Result<*mut ID3D11RenderTargetView>;
    fn depth_stencil_view(&self) -> error::Result<*mut ID3D11DepthStencilView>;
}
