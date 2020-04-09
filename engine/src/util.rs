use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Make a wide-encoded string for use with some APIs.
pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}
