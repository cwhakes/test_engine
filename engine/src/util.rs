mod partial_max_min;

pub use partial_max_min::PartialMaxMin;

use crate::error::{self, HResultToResult};

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::{self, NonNull};

use winapi::Interface;
use winapi::shared::dxgi::IDXGIObject;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt;
use winapi::um::winuser;

/// Make a wide-encoded string for use with some APIs.
pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

pub fn get_output<F, A>(function: F) -> error::Result<NonNull<A>> where
    F: FnOnce(&mut *mut A) -> winnt::HRESULT
{
    let mut ptr = ptr::null_mut();
    function(&mut ptr).result()?;
    NonNull::new(ptr).ok_or(null_ptr_err!())
}

pub fn get_output2<F, A, B>(function: F) -> error::Result<(NonNull<A>, NonNull<B>)> where
    F: FnOnce(&mut *mut A, &mut *mut B) -> winnt::HRESULT
{
    let mut ptr_a = ptr::null_mut();
    let mut ptr_b = ptr::null_mut();
    function(&mut ptr_a, &mut ptr_b).result()?;
    let a = NonNull::new(ptr_a).ok_or(null_ptr_err!())?;
    let b = NonNull::new(ptr_b).ok_or(null_ptr_err!())?;
    Ok((a, b))
}

pub fn kill_window_focus() {
    unsafe {
        winuser::SetFocus(ptr::null_mut());
    }
}

/// A wrapper for the winapi function of the same name, used through the prelude
pub trait QueryInterface {
    fn query_interface<I: Interface>(&self) -> error::Result<NonNull<I>>;
}

impl QueryInterface for IUnknown {
    fn query_interface<I: Interface>(&self) -> error::Result<NonNull<I>> {
        unsafe {
            get_output(|ptr| {
                self.QueryInterface(&I::uuidof(), ptr)
            }).map(NonNull::cast::<I>)
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
            get_output(|ptr| {
                self.GetParent(&I::uuidof(), ptr)
            }).map(NonNull::cast::<I>)
        }
    }
}
