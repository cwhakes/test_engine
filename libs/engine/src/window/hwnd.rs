use winapi::shared::windef::{HWND, POINT};
use winapi::um::winuser::{ClientToScreen, DestroyWindow, GetClientRect};

use crate::math::Rect;

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

    pub fn rect(&self) -> Rect<i32> {
        let mut rect = Default::default();
        unsafe {
            GetClientRect(self.0, &mut rect);
        }

        let mut upper_left = POINT {
            x: rect.left,
            y: rect.top,
        };
        let mut lower_right = POINT {
            x: rect.right,
            y: rect.bottom,
        };
        unsafe {
            ClientToScreen(self.0, &mut upper_left);
            ClientToScreen(self.0, &mut lower_right);
        }

        Rect([upper_left.x..lower_right.x, upper_left.y..lower_right.y])
    }
}

impl Drop for Hwnd {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.0);
        }
    }
}
