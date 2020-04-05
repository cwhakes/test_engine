use std::{convert, fmt, ops};
use std::ptr::NonNull;
use winapi::um::d3dcommon::ID3DBlob;

pub struct Blob(NonNull<ID3DBlob>);

impl convert::AsRef<ID3DBlob> for Blob {
    fn as_ref(&self) -> &ID3DBlob {
        unsafe { self.0.as_ref() }
    }
}

impl convert::AsMut<ID3DBlob> for Blob {
    fn as_mut(&mut self) -> &mut ID3DBlob {
        unsafe { self.0.as_mut() }
    }
}
impl convert::TryFrom<*mut ID3DBlob> for Blob {
    type Error = ();

    fn try_from(ptr: *mut ID3DBlob) -> Result<Self, Self::Error> {
        match NonNull::new(ptr) {
            Some(inner) => Ok(Blob(inner)),
            None => Err(()),
        }
    }
}

impl fmt::Debug for Blob {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = String::from_utf8_lossy(&self);

        f.debug_struct("Blob").field("slice", &string).finish()
    }
}

impl ops::Deref for Blob {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().GetBufferPointer() as *const u8,
                self.as_ref().GetBufferSize(),
            )
        }
    }
}

impl ops::DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_ref().GetBufferPointer() as *mut u8,
                self.as_ref().GetBufferSize(),
            )
        }
    }
}

impl ops::Drop for Blob {
    fn drop(&mut self) {
        unsafe {
            self.as_ref().Release();
        }
    }
}
