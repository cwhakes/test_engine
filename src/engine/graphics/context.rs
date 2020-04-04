use crate::engine::graphics::swapchain::SwapChain;

use winapi::um::d3d11::ID3D11DeviceContext;

pub struct Context(*mut ID3D11DeviceContext);

impl Context {
    pub fn clear_render_target_color(
        &self,
        swapchain: &SwapChain,
        red: f32,
        grn: f32,
        blu: f32,
        alp: f32,
    ) {
        unsafe {
            self.0
                .as_ref()
                .unwrap()
                .ClearRenderTargetView(swapchain.back_buffer(), &[red, grn, blu, alp])
        }
    }
}

impl From<*mut ID3D11DeviceContext> for Context {
    fn from(ptr: *mut ID3D11DeviceContext) -> Self {
        Context(ptr)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.0.as_ref() {
                ptr.Release();
            }
        }
    }
}
