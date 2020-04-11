mod hwnd;

pub use hwnd::Hwnd;

use crate::util::os_vec;

use std::sync::Mutex;
use std::{mem, ptr};

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;
use winapi::shared::windef::HBRUSH;

use winapi::um::winuser;
use winapi::um::winuser::{CreateWindowExW, RegisterClassExW, WNDCLASSEXW};
use winapi::um::winuser::COLOR_WINDOW;
use winapi::um::winuser::{DispatchMessageW, PeekMessageW, TranslateMessage};
use winapi::um::winuser::{ShowWindow, UpdateWindow};
use winapi::um::winuser::{IDC_ARROW, IDI_APPLICATION};
use winapi::um::winuser::{WS_EX_OVERLAPPEDWINDOW, WS_OVERLAPPEDWINDOW};

pub trait Window: Send + Sync {
    fn me() -> &'static Mutex<Option<Self>> where Self: Sized;
    fn window_inner(&self) -> &WindowInner;
    fn window_inner_mut(&mut self) -> &mut WindowInner;

    fn on_create(_hwnd: Hwnd) where Self: Sized {}
    fn on_update(&mut self) {}
    fn on_destroy(&mut self) {}
    fn on_focus(_window: &'static Mutex<Option<Self>>) where Self: Sized {}
    fn on_kill_focus(_window: &'static Mutex<Option<Self>>) where Self: Sized {}

    fn init()
    where
        Self: Sized + 'static,
    {
        unsafe {
            let class_name = os_vec("MyWindowClass");
            let menu_name = os_vec("");
            let window_name = os_vec("DirectX Application");

            let wc = WNDCLASSEXW {
                cbClsExtra: 0,
                cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
                cbWndExtra: 0,
                hbrBackground: COLOR_WINDOW as HBRUSH,
                hCursor: winuser::LoadCursorW(ptr::null_mut(), IDC_ARROW),
                hIcon: winuser::LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hIconSm: winuser::LoadIconW(ptr::null_mut(), IDI_APPLICATION),
                hInstance: ptr::null_mut(),
                lpszClassName: class_name.clone().as_ptr(),
                lpszMenuName: menu_name.as_ptr(),
                style: 0,
                lpfnWndProc: Some(Self::window_loop),
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

    /// Engine event loop
    fn broadcast(&mut self) {
        unsafe {
            self.on_update();

            let mut msg = Default::default();
            while 0 < PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            std::thread::sleep(Default::default());
        }
    }

    /// Windows Window event Loop
    /// 
    /// # Safety
    /// 
    /// Should only ever be called by Windows
    unsafe extern "system" fn window_loop(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT
        where Self: Sized + 'static
    {
        match msg {
            winuser::WM_CREATE => {
                let hwnd = Hwnd::new(hwnd);
                Self::on_create(hwnd);
                if let Some(window) = &mut *Self::me().lock().unwrap() {
                    window.window_inner_mut().running = true;
                }
                0
            }
            winuser::WM_SETFOCUS => {
                std::thread::spawn(|| {
                    Self::on_focus(Self::me());
                });
                0
            }
            winuser::WM_KILLFOCUS => {
                std::thread::spawn(|| {
                    Self::on_kill_focus(Self::me());
                });
                0
            }
            winuser::WM_DESTROY => {
                //Spawn a different thread to prevent recursive lock
                std::thread::spawn(|| {
                    if let Some(window) = &mut *Self::me().lock().unwrap() {
                        window.window_inner_mut().running = false;
                        window.on_destroy();
                    };
                });
                winuser::PostQuitMessage(0);
                0
            }
            _ => winuser::DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

#[derive(Default)]
pub struct WindowInner {
    pub hwnd: Option<Hwnd>,
    pub running: bool,
}

impl WindowInner {
    pub fn new() -> WindowInner {
        WindowInner {
            hwnd: None,
            running: false,
        }
    }
}
