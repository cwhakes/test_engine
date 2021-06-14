use super::context::Context;
use super::Device;

use crate::error;
use crate::util::get_output;

use std::any::{Any, TypeId};
use std::ptr::{self, NonNull};

use winapi::um::d3d11;

/// Used to communicate a single value with shaders.
/// Call `set_constant_buffer` on context to use.
pub struct ConstantBuffer<C: ?Sized> {
    buffer: NonNull<d3d11::ID3D11Buffer>,
    _phantom: std::marker::PhantomData<C>,
}

//TODO FIXME Verify
unsafe impl<C: ?Sized> Send for ConstantBuffer<C> where C: Send {}
unsafe impl<C: ?Sized> Sync for ConstantBuffer<C> where C: Sync {}

impl<C: ?Sized> ConstantBuffer<C> {
    /// Constructs a new ConstantBuffer.
    pub fn new(device: &Device, constant: &mut C) -> error::Result<Self> {
        unsafe {
            let buff_desc = d3d11::D3D11_BUFFER_DESC {
                Usage: d3d11::D3D11_USAGE_DEFAULT,
                ByteWidth: std::mem::size_of_val(constant) as u32,
                BindFlags: d3d11::D3D11_BIND_CONSTANT_BUFFER,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                ..Default::default()
            };
            assert!(buff_desc.ByteWidth % 16 == 0);

            let data = d3d11::D3D11_SUBRESOURCE_DATA {
                pSysMem: (constant as *const C).cast(),
                ..Default::default()
            };

            let buffer = get_output(|ptr| device.as_ref().CreateBuffer(&buff_desc, &data, ptr))?;

            Ok(Self {
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
            context.as_ref().UpdateSubresource(
                &**self.buffer.as_ref() as *const _ as *mut _,
                0,
                ptr::null_mut(),
                (constant as *mut C).cast(),
                0,
                0,
            );
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
