use crate::error::{self, HResultToResult};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{self, NonNull};

use winapi::Interface;
use winapi::shared::dxgi::IDXGIObject;
use winapi::um::unknwnbase::IUnknown;

/// Make a wide-encoded string for use with some APIs.
pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

/// A wrapper for the winapi function of the same name, used through the prelude
pub trait QueryInterface {
    fn query_interface<I: Interface>(&self) -> error::Result<NonNull<I>>;
}

impl QueryInterface for IUnknown {
    fn query_interface<I: Interface>(&self) -> error::Result<NonNull<I>> {
        unsafe {
            let mut output = ptr::null_mut();
            self.QueryInterface(&I::uuidof(), &mut output).result()?;
            NonNull::new(output as *mut I).ok_or(null_ptr_err!())
        }
    }
}

/// A wrapper for the winapi function of the same name, used through the prelude
pub trait GetParent {
    fn get_parent<I: Interface>(&self) -> error::Result<NonNull<I>>;
}

impl GetParent for IDXGIObject {
    fn get_parent<I: Interface>(&self) -> error::Result<NonNull<I>> {
        unsafe {
            let mut output = ptr::null_mut();
            self.GetParent(&I::uuidof(), &mut output).result()?;
            NonNull::new(output as *mut I).ok_or(null_ptr_err!())
        }
    }
}
