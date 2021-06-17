use winapi::shared::windef::HWND;
use winapi::um::winuser::{DestroyWindow, GetClientRect};

pub struct Hwnd(HWND);

// See https://github.com/retep998/winapi-rs/issues/396
unsafe impl Send for Hwnd {}
unsafe impl Sync for Hwnd {}

impl Hwnd {
    /// # Safety
    ///
    /// `hwnd` must be a valid HWND
    pub unsafe fn new(hwnd: HWND) -> Self {
        Self(hwnd)
    }

    pub fn inner(&self) -> &HWND {
        &self.0
    }

    pub fn rect(&self) -> (u32, u32) {
        let mut rect = Default::default();
        unsafe {
            GetClientRect(self.0, &mut rect);
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;
        (width as u32, height as u32)
    }
}

impl Drop for Hwnd {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.0);
        }
    }
}
