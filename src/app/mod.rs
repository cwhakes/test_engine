use crate::engine::graphics::shader::{self, Shader};
use crate::engine::graphics::{ConstantBuffer, VertexBuffer};
use crate::engine::graphics::{Graphics, GRAPHICS};
use crate::engine::window::{Hwnd, Window};

use crate::util::get_tick_count;

use std::sync::atomic::{AtomicBool, Ordering};

#[repr(C)]
struct Vertex([f32; 3]);
#[repr(C)]
struct VertexColor([f32; 3], [f32; 3], [f32; 3]);

#[repr(C, align(16))]
struct Constant {
    time: u32,
}

pub struct AppWindow {
    m_hwnd: Option<Hwnd>,
    running: AtomicBool,
    vertex_buffer: Option<VertexBuffer<VertexColor>>,
    vertex_shader: Option<Shader<shader::Vertex>>,
    pixel_shader: Option<Shader<shader::Pixel>>,
    constant_buffer: Option<ConstantBuffer<Constant>>,
}

impl Window for AppWindow {
    fn create() -> Self {
        AppWindow {
            m_hwnd: None,
            running: AtomicBool::new(false),
            vertex_buffer: None,
            vertex_shader: None,
            pixel_shader: None,
            constant_buffer: None,
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
            VertexColor([-0.5, -0.5, 0.0], [-0.5, 0.5, 0.0], [1.0, 0.0, 0.0]),
            VertexColor([-0.5, 0.5, 0.0], [-0.5, 0.5, 0.0], [0.0, 1.0, 0.0]),
            VertexColor([0.5, -0.5, 0.0], [-0.5, 0.5, 0.0], [0.0, 0.0, 1.0]),
            VertexColor([0.5, 0.5, 0.0], [-0.5, 0.5, 0.0], [1.0, 1.0, 0.0]),
        ];

        let graphics = Graphics::new(self.hwnd().unwrap());
        let (vertex_shader, blob) =
            Shader::<shader::Vertex>::new(graphics.device(), "vertex_shader.hlsl");
        let (pixel_shader, _) =
            Shader::<shader::Pixel>::new(graphics.device(), "pixel_shader.hlsl");
        *GRAPHICS.lock().unwrap() = Some(graphics);

        self.vertex_shader = Some(vertex_shader);
        self.pixel_shader = Some(pixel_shader);

        let vb = VertexBuffer::new(&vertex_list, &blob);
        self.vertex_buffer = Some(vb);
        let cb = ConstantBuffer::new(&Constant {
            time: get_tick_count(),
        });
        self.constant_buffer = Some(cb);
    }

    fn on_update(&mut self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_ref() {
            let context = g.immediate_context();
            context.clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            let (width, height) = self.m_hwnd.as_ref().unwrap().rect();
            context.set_viewport_size(width as f32, height as f32);

            if let Some(cb) = self.constant_buffer.as_mut() {
                let mut constant = Constant {
                    time: get_tick_count(),
                };
                cb.update(context, &mut constant as *mut _ as *mut _);
                context.set_constant_buffer::<shader::Vertex, _>(cb);
                context.set_constant_buffer::<shader::Pixel, _>(cb);
            }

            context.set_shader(self.vertex_shader.as_mut().unwrap());
            context.set_shader(self.pixel_shader.as_mut().unwrap());
            context.set_vertex_buffer(self.vertex_buffer.as_ref().unwrap());
            context.draw_triangle_strip::<Vertex>(self.vertex_buffer.as_ref().unwrap().len(), 0);

            g.swapchain().present(0);
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
        self.vertex_buffer.take();
    }
}
