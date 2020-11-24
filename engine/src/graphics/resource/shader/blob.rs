use crate::error;

use std::ptr::NonNull;
use std::{fmt, ops};
use winapi::um::d3dcommon::ID3DBlob;

pub struct Blob(NonNull<ID3DBlob>);

impl Blob {
    /// # Safety
    ///
    /// `blob` must point to a valid ID3DBlob
    pub unsafe fn new(blob: *mut ID3DBlob) -> error::Result<Blob> {
        match NonNull::new(blob) {
            Some(inner) => Ok(Blob(inner)),
            None => Err(null_ptr_err!()),
        }
    }
}

impl fmt::Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = String::from_utf8_lossy(&self);

        f.debug_struct("Blob").field("slice", &string).finish()
    }
}

impl fmt::Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = String::from_utf8_lossy(&self);

        write! {f, "{}", string}
    }
}

impl ops::Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.0.as_ref().GetBufferPointer() as *const u8,
                self.0.as_ref().GetBufferSize(),
            )
        }
    }
}

impl ops::DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.as_mut().GetBufferPointer() as *mut u8,
                self.0.as_mut().GetBufferSize(),
            )
        }
    }
}

impl ops::Drop for Blob {
    fn drop(&mut self) {
        unsafe {
            self.0.as_mut().Release();
        }
    }
}
