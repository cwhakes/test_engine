use crate::engine::graphics::GRAPHICS;
use crate::engine::graphics::context::Context;

use std::ffi::c_void;

use winapi::shared::winerror::FAILED;
use winapi::um::d3d11;

pub struct ConstantBuffer<C>
where
    C: Sized,
{
    pub buffer: *mut d3d11::ID3D11Buffer,
    _phantom: std::marker::PhantomData<C>,
}

//TODO FIXME Verify
unsafe impl<C> Send for ConstantBuffer<C> where C: Send {}
unsafe impl<C> Sync for ConstantBuffer<C> where C: Sync {}

impl<C> ConstantBuffer<C> {
    pub fn new(constant: &C) -> ConstantBuffer<C> {
        unsafe {
            let g = GRAPHICS.lock().unwrap();
            let g = g.as_ref().unwrap();

            let device = g.device.as_ref();

            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = (std::mem::size_of::<C>()) as u32;
            buff_desc.BindFlags = d3d11::D3D11_BIND_CONSTANT_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = constant as *const _ as *const c_void;

            let mut buffer = std::ptr::null_mut();

            let res = device.CreateBuffer(&buff_desc, &data, &mut buffer);

            if FAILED(res) {
                panic!();
            }

            ConstantBuffer {
                buffer,
                _phantom: Default::default(),
            }
        }
    }

    pub fn update(&mut self, context: &Context, buffer: *mut c_void) {
        unsafe {
            context.as_ref().UpdateSubresource(
                &**self.buffer as *const _ as *mut _,
                0,
                std::ptr::null_mut(),
                buffer,
                0,
                0,
            );
        }
    }
}

impl<C> Drop for ConstantBuffer<C> {
    fn drop(&mut self) {
        unsafe {
            if let Some(buffer) = self.buffer.as_ref() {
                buffer.Release();
            }
        }
    }
}
