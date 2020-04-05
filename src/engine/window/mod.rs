mod hwnd;

pub use hwnd::Hwnd;

use crate::util::os_vec;

use std::sync::Mutex;
use std::{mem, ptr};

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;

use winapi::um::winuser;
use winapi::um::winuser::{CreateWindowExW, RegisterClassExW, WNDCLASSEXW};
//use winapi::um::winuser::COLOR_WINDOW;
//use winapi::shared::windef::HBRUSH;
use winapi::um::winuser::{DispatchMessageW, PeekMessageW, TranslateMessage};
use winapi::um::winuser::{ShowWindow, UpdateWindow};
use winapi::um::winuser::{IDC_ARROW, IDI_APPLICATION};
use winapi::um::winuser::{WM_CREATE, WM_DESTROY};
use winapi::um::winuser::{WS_EX_OVERLAPPEDWINDOW, WS_OVERLAPPEDWINDOW};

lazy_static! {
    pub static ref WINDOW: Mutex<Option<Box<dyn Window>>> = Mutex::new(None);
}

unsafe extern "system" fn window_loop(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CREATE => {
            if let Some(ref mut window) = *WINDOW.lock().unwrap() {
                window.set_hwnd(hwnd.into());
                window.on_create();
                window.set_running(true);
            };
            0
        }
        WM_DESTROY => {
            //Spawn a different thread to prevent recursive lock
            std::thread::spawn(|| {
                if let Some(ref mut window) = *WINDOW.lock().unwrap() {
                    window.on_destroy();
                    window.set_running(false);
                };
            });
            winuser::PostQuitMessage(0);
            0
        }
        _ => winuser::DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub trait Window: Send + Sync {
    fn create() -> Self
    where
        Self: Sized;
    fn set_hwnd(&mut self, m_hwnd: Hwnd);
    fn hwnd(&self) -> Option<&Hwnd>;
    fn set_running(&self, running: bool);
    fn running(&self) -> bool;

    fn on_create(&mut self) {}
    fn on_update(&mut self) {}
    fn on_destroy(&mut self) {}

    fn init()
    where
        Self: Sized + 'static,
    {
        unsafe {
            let class_name = os_vec("MyWindowClass");
            let menu_name = os_vec("");
            let window_name = os_vec("DirectX Application");

            let window = Self::create();
            *WINDOW.lock().unwrap() = Some(Box::new(window));

            let wc = WNDCLASSEXW {
                cbClsExtra: 0,
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                cbWndExtra: 0,
                hbrBackground: ptr::null_mut(), //&COLOR_WINDOW as HBRUSH,
                hCursor: winuser::LoadCursorW(ptr::null_mut(), IDC_ARROW),
                hIcon: winuser::LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hIconSm: winuser::LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hInstance: ptr::null_mut(),
                lpszClassName: class_name.clone().as_ptr(),
                lpszMenuName: menu_name.as_ptr(),
                style: 0,
                lpfnWndProc: Some(window_loop),
            };
            if 0 == RegisterClassExW(&wc) {
                panic!()
            };

            let m_hwnd = CreateWindowExW(
                WS_EX_OVERLAPPEDWINDOW,
                class_name.as_ptr(),
                window_name.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                1024,
                768,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );

            ShowWindow(m_hwnd, winuser::SW_SHOW);
            UpdateWindow(m_hwnd);
        }
    }

    fn broadcast(&mut self) {
        unsafe {
            self.on_update();

            let mut msg = Default::default();
            while 0 < PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) {
                TranslateMessage(&mut msg as *const _);
                DispatchMessageW(&mut msg as *const _);
            }
            std::thread::sleep(Default::default());
        }
    }
}
