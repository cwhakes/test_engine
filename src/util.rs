use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}
