use super::context::Context;
use super::Device;

use crate::prelude::*;
use crate::error;

use std::ffi::c_void;
use std::ptr::{self, NonNull};

use winapi::um::d3d11;

/// Used to communicate a single value with shaders.
/// Call `set_constant_buffer` on context to use.
pub struct ConstantBuffer<C>
where
    C: Sized,
{
    buffer: NonNull<d3d11::ID3D11Buffer>,
    _phantom: std::marker::PhantomData<C>,
}

//TODO FIXME Verify
unsafe impl<C> Send for ConstantBuffer<C> where C: Send {}
unsafe impl<C> Sync for ConstantBuffer<C> where C: Sync {}

impl<C> ConstantBuffer<C> {
    /// Constructs a new ConstantBuffer.
    pub fn new(device: &Device, constant: &C) -> error::Result<ConstantBuffer<C>> {
        unsafe {
            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = (std::mem::size_of::<C>()) as u32;
            buff_desc.BindFlags = d3d11::D3D11_BIND_CONSTANT_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = constant as *const _ as *const c_void;

            let mut buffer = ptr::null_mut();

            device.as_ref().CreateBuffer(&buff_desc, &data, &mut buffer).result()?;

            let buffer = NonNull::new(buffer).ok_or(null_ptr_err!())?;

            Ok(ConstantBuffer {
                buffer,
                _phantom: Default::default(),
            })
        }
    }

    pub fn buffer_ptr(&mut self) -> *mut d3d11::ID3D11Buffer {
        self.buffer.as_ptr()
    }

    pub fn update(&mut self, context: &Context, buffer: &mut C) {
        unsafe {
            context.as_ref().UpdateSubresource(
                &**self.buffer.as_ref() as *const _ as *mut _,
                0,
                ptr::null_mut(),
                buffer as *mut _ as *mut _,
                0,
                0,
            );
        }
    }
}

impl<C> AsRef<d3d11::ID3D11Buffer> for ConstantBuffer<C> {
    fn as_ref(&self) -> &d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_ref() }
    }
}

impl<C> AsMut<d3d11::ID3D11Buffer> for ConstantBuffer<C> {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_mut() }
    }
}

impl<C> Drop for ConstantBuffer<C> {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
        }
    }
}
