mod hwnd;

pub use hwnd::Hwnd;

use crate::util::os_vec;

use std::sync::Mutex;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;

use winapi::um::winuser::{CreateWindowExW, RegisterClassExW, WNDCLASSEXW};
//use winapi::um::winuser::COLOR_WINDOW;
//use winapi::shared::windef::HBRUSH;
use winapi::um::winuser::DefWindowProcW;
use winapi::um::winuser::LoadCursorW;
use winapi::um::winuser::LoadIconW;
use winapi::um::winuser::PeekMessageW;
use winapi::um::winuser::PostQuitMessage;
use winapi::um::winuser::CW_USEDEFAULT;
use winapi::um::winuser::IDC_ARROW;
use winapi::um::winuser::IDI_APPLICATION;
use winapi::um::winuser::SW_SHOW;
use winapi::um::winuser::{ShowWindow, UpdateWindow};
use winapi::um::winuser::{WM_CREATE, WM_DESTROY};
use winapi::um::winuser::{WS_EX_OVERLAPPEDWINDOW, WS_OVERLAPPEDWINDOW};
//use winapi::um::winuser::LPMSG;
use winapi::um::winuser::MSG;
use winapi::um::winuser::PM_REMOVE;
use winapi::um::winuser::{DispatchMessageW, TranslateMessage};

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
            if let Some(window) = WINDOW.lock().unwrap().as_mut() {
                window.set_hwnd(hwnd.into());
                window.on_create();
                window.set_running(true);
            };
            0
        }
        WM_DESTROY => {
            //Spawn a different thread to prevent recursive lock
            std::thread::spawn(|| {
                if let Some(window) = WINDOW.lock().unwrap().as_mut() {
                    window.on_destroy();
                    window.set_running(false);
                };
            });
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
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
    fn on_update(&self) {}
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
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                cbWndExtra: 0,
                hbrBackground: std::ptr::null_mut(), //&COLOR_WINDOW as HBRUSH,
                hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
                hIcon: LoadIconW(std::ptr::null_mut(), IDI_APPLICATION),
                hIconSm: LoadIconW(std::ptr::null_mut(), IDI_APPLICATION),
                hInstance: std::ptr::null_mut(),
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
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1024,
                768,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            ShowWindow(m_hwnd, SW_SHOW);
            UpdateWindow(m_hwnd);
        }
    }

    fn broadcast(&self) {
        unsafe {

            self.on_update();

            let mut msg = Default::default();
            while 0 < PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) {
                TranslateMessage(&mut msg as *const MSG);
                DispatchMessageW(&mut msg as *const MSG);
            }
            std::thread::sleep(Default::default());
        }
    }
}


