use crate::engine::graphics::vertex_buffer::VertexBuffer;
use crate::engine::graphics::{Graphics, GRAPHICS};
use crate::engine::window::{Hwnd, Window};

use std::sync::atomic::{AtomicBool, Ordering};

struct Vertex([f32; 3]);
struct VertexColor([f32; 3], [f32; 3]);

pub struct AppWindow {
    m_hwnd: Option<Hwnd>,
    running: AtomicBool,
    vertex_buffer: Option<VertexBuffer<VertexColor>>,
}

impl Window for AppWindow {
    fn create() -> Self {
        AppWindow {
            m_hwnd: None,
            running: AtomicBool::new(false),
            vertex_buffer: None,
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

    fn on_create(&mut self) {
        let vertex_list = [
            VertexColor([-0.5, -0.5, 0.0],[1.0,0.0,0.0]),
            VertexColor([-0.5,  0.5, 0.0],[0.0,1.0,0.0]),
            VertexColor([ 0.5, -0.5, 0.0],[0.0,0.0,1.0]),
            VertexColor([ 0.5,  0.5, 0.0],[1.0,1.0,0.0]),
        ];

        
        let mut g = GRAPHICS.lock().unwrap();
        let mut graphics = Graphics::new(self.hwnd().unwrap());
        graphics.create_vertex_shader("vertex_shader.hlsl");
        graphics.create_pixel_shader("pixel_shader.hlsl");
        let (byte_code, size) = graphics.get_shader_buffer_and_size();
        *g = Some(graphics);
        drop(g);

        let vb = VertexBuffer::new(&vertex_list, byte_code, size);
        self.vertex_buffer = Some(vb);
    }

    fn on_update(&self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_ref() {
            g.immediate_context()
                .clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            let (width, height) = self.m_hwnd.as_ref().unwrap().rect();
            g.immediate_context()
                .set_viewport_size(width as f32, height as f32);
            g.set_shaders();
            g.immediate_context()
                .set_vertex_buffer(self.vertex_buffer.as_ref().unwrap());
            g.immediate_context()
                .draw_triangle_strip::<Vertex>(self.vertex_buffer.as_ref().unwrap().len(), 0);

            g.swapchain().present(0);
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
        self.vertex_buffer.take();
    }
}
