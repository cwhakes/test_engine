use engine::graphics::shader::{self, Shader};
use engine::graphics::{ConstantBuffer, Context, VertexBuffer};
use engine::graphics::{Graphics, GRAPHICS};
use engine::math::{Matrix4x4, Vector3d};
use engine::util::get_tick_count;
use engine::vertex;
use engine::window::{Window, WindowInner};

#[repr(C)]
#[derive(Vertex)]
struct VertexColor(vertex::Position, vertex::Position, vertex::Color);

#[repr(C, align(16))]
#[derive(Default, Debug)]
struct Constant {
    world: Matrix4x4,
    view: Matrix4x4,
    proj: Matrix4x4,
    time: u32,
}

#[derive(Default)]
pub struct AppWindow {
    window_inner: WindowInner,
    vertex_buffer: Option<VertexBuffer<VertexColor>>,
    vertex_shader: Option<Shader<shader::Vertex>>,
    pixel_shader: Option<Shader<shader::Pixel>>,
    constant_buffer: Option<ConstantBuffer<Constant>>,
    prev_time: u32,
    curr_time: u32,
    delta_t: f32,
    delta_pos: f32,
    delta_scale: f32,
}

impl Window for AppWindow {
    fn create() -> Self {
        AppWindow {
            window_inner: WindowInner::new(),
            vertex_buffer: None,
            vertex_shader: None,
            pixel_shader: None,
            constant_buffer: None,
            prev_time: get_tick_count(),
            curr_time: get_tick_count(),
            ..Default::default()
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
                ..Default::default()
            },
        );

        self.vertex_shader = Some(vertex_shader);
        self.pixel_shader = Some(pixel_shader);
        self.vertex_buffer = Some(vb);
        self.constant_buffer = Some(cb);

        *GRAPHICS.lock().unwrap() = Some(graphics);
    }

    fn on_update(&mut self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_mut() {
            let context = g.immediate_context();
            context.clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            let (width, height) = self.window_inner.hwnd.as_ref().unwrap().rect();
            context.set_viewport_size(width as f32, height as f32);

            self.update_quad_position(context);

            context.set_shader(self.vertex_shader.as_mut().unwrap());
            context.set_shader(self.pixel_shader.as_mut().unwrap());
            context.set_vertex_buffer(self.vertex_buffer.as_ref().unwrap());
            context.draw_triangle_strip::<VertexColor>(self.vertex_buffer.as_ref().unwrap().len(), 0);

            g.resize();
            g.swapchain().present(0);

            self.prev_time = self.curr_time;
            self.curr_time = get_tick_count();
            self.delta_t = (self.curr_time - self.prev_time) as f32 /1000.0;
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
        self.vertex_buffer.take();
    }
}

impl AppWindow {
    fn update_quad_position(&mut self, context: &Context) {
        let (width, height) = self.window_inner().hwnd.as_ref().unwrap().rect();
        if let Some(cb) = self.constant_buffer.as_mut() {
            self.delta_pos += self.delta_t / 10.0;
            if self.delta_pos > 1.0 {
                self.delta_pos -= 1.0;
            }
            self.delta_scale += self.delta_t / 0.15;
            let mut world = Matrix4x4::scaling(
                Vector3d::new(0.5, 0.5, 0.0).lerp([1.0, 1.0, 0.0], (self.delta_scale.sin() +1.0)/2.0)
            );
            world *= Matrix4x4::translation(
                Vector3d::new(-1.5, -1.5, 0.0).lerp([1.5, 1.5, 0.0], self.delta_pos)
            );
            let view = Matrix4x4::identity();
            let proj = Matrix4x4::orthoganal(
                width as f32 / 400.0,
                height as f32 / 400.0,
                -4.0,
                4.0,
            );

            let mut constant = Constant {
                world,
                view,
                proj,
                time: get_tick_count(),
            };
            cb.update(context, &mut constant);
            context.set_constant_buffer::<shader::Vertex, _>(cb);
            context.set_constant_buffer::<shader::Pixel, _>(cb);
        }
    }
}
