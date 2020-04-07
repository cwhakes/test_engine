use engine::graphics::shader::{self, Shader};
use engine::graphics::{ConstantBuffer, VertexBuffer};
use engine::graphics::{Graphics, GRAPHICS};
use engine::util::get_tick_count;
use engine::vertex;
use engine::window::{Window, WindowInner};

#[repr(C)]
#[derive(Vertex)]
struct VertexColor(vertex::Position, vertex::Position, vertex::Color);

#[repr(C, align(16))]
struct Constant {
    time: u32,
}

pub struct AppWindow {
    window_inner: WindowInner,
    vertex_buffer: Option<VertexBuffer<VertexColor>>,
    vertex_shader: Option<Shader<shader::Vertex>>,
    pixel_shader: Option<Shader<shader::Pixel>>,
    constant_buffer: Option<ConstantBuffer<Constant>>,
}

impl Window for AppWindow {
    fn create() -> Self {
        AppWindow {
            window_inner: WindowInner::new(),
            vertex_buffer: None,
            vertex_shader: None,
            pixel_shader: None,
            constant_buffer: None,
        }
    }

    fn window_inner(&self) -> &WindowInner {
        &self.window_inner
    }

    fn window_inner_mut(&mut self) -> &mut WindowInner {
        &mut self.window_inner
    }

    fn on_create(&mut self) {
        let vertex_list = [
            VertexColor([-0.5, -0.5, 0.0].into(), [-0.5, 0.5, 0.0].into(), [1.0, 0.0, 0.0].into()),
            VertexColor([-0.5, 0.5, 0.0].into(), [-0.5, 0.5, 0.0].into(), [0.0, 1.0, 0.0].into()),
            VertexColor([0.5, -0.5, 0.0].into(), [-0.5, 0.5, 0.0].into(), [0.0, 0.0, 1.0].into()),
            VertexColor([0.5, 0.5, 0.0].into(), [-0.5, 0.5, 0.0].into(), [1.0, 1.0, 0.0].into()),
        ];

        let graphics = Graphics::new(self.window_inner.hwnd.as_ref().unwrap());
        let (vertex_shader, blob) = graphics.device().new_shader::<shader::Vertex>("vertex_shader.hlsl");
        let (pixel_shader, _) = graphics.device().new_shader::<shader::Pixel>("pixel_shader.hlsl");
        let vb = graphics.device().new_vertex_buffer(&vertex_list, &blob);
        let cb = graphics.device().new_constant_buffer(
            &Constant {
                time: get_tick_count(),
            },
        );

        self.vertex_shader = Some(vertex_shader);
        self.pixel_shader = Some(pixel_shader);
        self.vertex_buffer = Some(vb);
        self.constant_buffer = Some(cb);

        *GRAPHICS.lock().unwrap() = Some(graphics);
    }

    fn on_update(&mut self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_ref() {
            let context = g.immediate_context();
            context.clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            let (width, height) = self.window_inner.hwnd.as_ref().unwrap().rect();
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
            context.draw_triangle_strip::<VertexColor>(self.vertex_buffer.as_ref().unwrap().len(), 0);

            g.swapchain().present(0);
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
        self.vertex_buffer.take();
    }
}
