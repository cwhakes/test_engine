use crate::engine::graphics::{Graphics, GRAPHICS};
use crate::engine::window::{Hwnd, Window};

use std::sync::atomic::{AtomicBool, Ordering};

pub struct AppWindow {
    m_hwnd: Option<Hwnd>,
    running: AtomicBool,
}

impl Window for AppWindow {
    fn create() -> Self {
        AppWindow {
            m_hwnd: None,
            running: AtomicBool::new(false),
        }
    }

    fn set_hwnd(&mut self, m_hwnd: Hwnd) {
        self.m_hwnd = Some(m_hwnd)
    }

    fn hwnd(&self) -> Option<&Hwnd> {
        self.m_hwnd.as_ref()
    }

    fn set_running(&self, running: bool) {
        self.running.store(running, Ordering::Relaxed);
    }

    fn running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    fn on_create(&self) {
        let mut g = GRAPHICS.lock().unwrap();
        *g = Some(Graphics::new(self.hwnd().unwrap()));
    }

    fn on_update(&self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_ref() {
            g.immediate_context()
                .clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            g.swapchain().present(0);
        }
    }

    fn on_destroy(&self) {
        GRAPHICS.lock().unwrap().take();
    }
}
