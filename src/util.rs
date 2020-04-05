use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

pub fn get_tick_count() -> u32 {
    unsafe { winapi::um::sysinfoapi::GetTickCount() }
}

pub fn os_vec(text: &str) -> Vec<u16> {
    OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}
