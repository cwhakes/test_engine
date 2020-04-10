use engine::graphics::shader::{self, Shader};
use engine::graphics::{ConstantBuffer, Context, IndexBuffer, VertexBuffer};
use engine::graphics::{Graphics, GRAPHICS};
use engine::input::{INPUT, Listener};
use engine::math::Matrix4x4;
use engine::time::{DeltaT, get_tick_count};
use engine::vertex;
use engine::window::{Window, WindowInner};

use std::sync::{Arc, Mutex};

#[repr(C)]
#[derive(Vertex)]
struct VertexColor(vertex::Position, vertex::Color, vertex::Color);

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
    index_buffer: Option<IndexBuffer>,
    delta_t: DeltaT,
    delta_pos: f32,
    delta_scale: f32,
    rot_x: f32,
    rot_y: f32,
}

impl Window for AppWindow {
    fn create() -> Arc<Mutex<Self>> {
        let window = AppWindow {
            window_inner: WindowInner::new(),
            vertex_buffer: None,
            vertex_shader: None,
            pixel_shader: None,
            constant_buffer: None,
            index_buffer: None,
            ..Default::default()
        };
        let window = Arc::new(Mutex::new(window));
        INPUT.lock().unwrap().add_listener(window.clone());
        window
    }

    fn window_inner(&self) -> &WindowInner {
        &self.window_inner
    }

    fn window_inner_mut(&mut self) -> &mut WindowInner {
        &mut self.window_inner
    }

    fn on_create(&mut self) {
        let vertex_list = [
            VertexColor([-0.5, -0.5, -0.5].into(), [0.0, 0.0, 0.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([-0.5, 0.5, -0.5].into(), [0.0, 1.0, 0.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([0.5, 0.5, -0.5].into(), [1.0, 1.0, 0.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([0.5, -0.5, -0.5].into(), [1.0, 0.0, 0.0].into(), [0.2, 0.2, 0.2].into()),
            
            VertexColor([0.5, -0.5, 0.5].into(), [1.0, 0.0, 1.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([0.5, 0.5, 0.5].into(), [1.0, 1.0, 1.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([-0.5, 0.5, 0.5].into(), [0.0, 1.0, 1.0].into(), [0.2, 0.2, 0.2].into()),
            VertexColor([-0.5, -0.5, 0.5].into(), [0.0, 0.0, 1.0].into(), [0.2, 0.2, 0.2].into()),
        ];

        let index_list = [
            //front
            0, 1, 2,
            2, 3, 0,
            //back
            4, 5, 6,
            6, 7, 4,
            //top
            1, 6, 5,
            5, 2, 1,
            //bottom
            7, 0, 3,
            3, 4, 7,
            //right
            3, 2, 5,
            5, 4, 3,
            //left
            7, 6, 1,
            1, 0, 7,
        ];

        let graphics = Graphics::new(self.window_inner.hwnd.as_ref().unwrap());
        let (vertex_shader, blob) = graphics.device().new_shader::<shader::Vertex>("vertex_shader.hlsl");
        let (pixel_shader, _) = graphics.device().new_shader::<shader::Pixel>("pixel_shader.hlsl");
        let vb = graphics.device().new_vertex_buffer(&vertex_list, &blob);
        let ib = graphics.device().new_index_buffer(&index_list);
        let cb = graphics.device().new_constant_buffer(
            &Constant {
                ..Default::default()
            },
        );

        self.vertex_shader = Some(vertex_shader);
        self.pixel_shader = Some(pixel_shader);
        self.vertex_buffer = Some(vb);
        self.constant_buffer = Some(cb);
        self.index_buffer = Some(ib);

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
            context.set_index_buffer(self.index_buffer.as_ref().unwrap());
            context.draw_indexed_triangle_list(self.index_buffer.as_ref().unwrap().len(), 0, 0,);

            g.resize();
            g.swapchain().present(0);

            self.delta_t.update();
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
        self.vertex_buffer.take();
    }
}

impl Listener for AppWindow {
    fn name(&self) -> &'static str {
        "AppWindow"
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => self.rot_x += 3.14 * self.delta_t.get(),
            b'S' => self.rot_x -= 3.14 * self.delta_t.get(),
            b'A' => self.rot_y += 3.14 * self.delta_t.get(),
            b'D' => self.rot_y -= 3.14 * self.delta_t.get(),
            _ => {},
        }
    }
    fn on_key_up(&mut self, _key: usize) {}
}

impl AppWindow {
    fn update_quad_position(&mut self, context: &Context) {
        let (width, height) = self.window_inner().hwnd.as_ref().unwrap().rect();
        if let Some(cb) = self.constant_buffer.as_mut() {
            self.delta_pos += self.delta_t.get() / 10.0;
            if self.delta_pos > 1.0 {
                self.delta_pos -= 1.0;
            }
            self.delta_scale += self.delta_t.get() / 1.0;
            /*let mut world = Matrix4x4::scaling(
                Vector3d::new(0.5, 0.5, 0.0).lerp([1.0, 1.0, 0.0], (self.delta_scale.sin() +1.0)/2.0)
            );
            world *= Matrix4x4::translation(
                Vector3d::new(-1.5, -1.5, 0.0).lerp([1.5, 1.5, 0.0], self.delta_pos)
            );*/
            let mut world = Matrix4x4::scaling([1.0, 1.0, 1.0]);
            world *= Matrix4x4::rotation_z(0.0);
            world *= Matrix4x4::rotation_y(self.rot_y);
            world *= Matrix4x4::rotation_x(self.rot_x);

            let view = Matrix4x4::identity();
            let proj = Matrix4x4::orthoganal(
                width as f32 / 300.0,
                height as f32 / 300.0,
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
