use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

/// Get current time, so we don't have to import another crate.
/// Returns milliseconds since program start
pub fn get_tick_count() -> u32 {
    unsafe { winapi::um::sysinfoapi::GetTickCount() }
}

/// Make a wide-encoded string for use with some APIs.
pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}
