use super::context::Context;
use super::Device;

use crate::error;
use crate::graphics::render::shader;
use crate::util::get_output;

use std::any::{Any, TypeId};
use std::ffi::c_void;
use std::ptr::{self, NonNull};

use winapi::um::d3d11;

/// Used to communicate a single value with shaders.
/// Call `set_constant_buffer` on context to use.
pub struct ConstantBuffer<C: ?Sized> {
    index: u32,
    buffer: NonNull<d3d11::ID3D11Buffer>,
    _phantom: std::marker::PhantomData<C>,
}

//TODO FIXME Verify
unsafe impl<C: ?Sized> Send for ConstantBuffer<C> where C: Send {}
unsafe impl<C: ?Sized> Sync for ConstantBuffer<C> where C: Sync {}

impl<C: ?Sized> ConstantBuffer<C> {
    /// Constructs a new ConstantBuffer.
    pub fn new(device: &Device, index: u32, constant: &mut C) -> error::Result<ConstantBuffer<C>> {
        unsafe {
            let mut buff_desc = d3d11::D3D11_BUFFER_DESC::default();
            buff_desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
            buff_desc.ByteWidth = std::mem::size_of_val(constant) as u32;
            assert!(buff_desc.ByteWidth % 16 == 0);
            buff_desc.BindFlags = d3d11::D3D11_BIND_CONSTANT_BUFFER;
            buff_desc.CPUAccessFlags = 0;
            buff_desc.MiscFlags = 0;

            let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
            data.pSysMem = constant as *const _ as *const c_void;

            let buffer = get_output(|ptr| {
                device.as_ref().CreateBuffer(&buff_desc, &data, ptr)
            })?;

            Ok(ConstantBuffer {
                index,
                buffer,
                _phantom: Default::default(),
            })
        }
    }

    pub fn buffer_ptr(&mut self) -> *mut d3d11::ID3D11Buffer {
        self.buffer.as_ptr()
    }

    pub fn update(&mut self, context: &Context, constant: &mut C) {
        unsafe {
            let index = self.index;

            context.as_ref().UpdateSubresource(
                &**self.buffer.as_ref() as *const _ as *mut _,
                0,
                ptr::null_mut(),
                constant as *mut C as *mut _,
                0,
                0,
            );

            context.set_constant_buffer::<shader::Vertex, _>(index, self);
            context.set_constant_buffer::<shader::Pixel, _>(index, self);
        }
    }
}

impl<A: Any> ConstantBuffer<A> {
    pub fn type_id(&self) -> TypeId {
        TypeId::of::<A>()
    }
}

impl<C: ?Sized> AsRef<d3d11::ID3D11Buffer> for ConstantBuffer<C> {
    fn as_ref(&self) -> &d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_ref() }
    }
}

impl<C: ?Sized> AsMut<d3d11::ID3D11Buffer> for ConstantBuffer<C> {
    fn as_mut(&mut self) -> &mut d3d11::ID3D11Buffer {
        unsafe { self.buffer.as_mut() }
    }
}

impl<C: ?Sized> Drop for ConstantBuffer<C> {
    fn drop(&mut self) {
        unsafe {
            self.buffer.as_ref().Release();
        }
    }
}
