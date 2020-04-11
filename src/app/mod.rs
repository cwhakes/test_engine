use engine::graphics::shaders::{self, Shader};
use engine::graphics::{ConstantBuffer, Context, IndexBuffer, VertexBuffer};
use engine::graphics::{Graphics, GRAPHICS};
use engine::input::{Listener, INPUT};
use engine::math::{Matrix4x4, Point};
use engine::time::{get_tick_count, DeltaT};
use engine::vertex;
use engine::window::{Hwnd, Window, WindowInner};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Mutex<Option<AppWindow>> = Mutex::new(None);
}

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

pub struct AppWindow {
    window_inner: WindowInner,
    vertex_buffer: VertexBuffer<VertexColor>,
    vertex_shader: Shader<shaders::Vertex>,
    pixel_shader: Shader<shaders::Pixel>,
    constant_buffer: ConstantBuffer<Constant>,
    index_buffer: IndexBuffer,
    delta_t: DeltaT,
    delta_pos: f32,
    delta_scale: f32,
    rot_x: f32,
    rot_y: f32,
    scale_cube: f32,
}

impl Window for AppWindow {
    fn me() -> &'static Mutex<Option<AppWindow>> {
        &WINDOW
    }

    fn window_inner(&self) -> &WindowInner {
        &self.window_inner
    }

    fn window_inner_mut(&mut self) -> &mut WindowInner {
        &mut self.window_inner
    }

    fn on_create(hwnd: Hwnd) {
        let vertex_list = [
            VertexColor(
                [-0.5, -0.5, -0.5].into(),
                [0.0, 0.0, 0.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [-0.5, 0.5, -0.5].into(),
                [0.0, 1.0, 0.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [0.5, 0.5, -0.5].into(),
                [1.0, 1.0, 0.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [0.5, -0.5, -0.5].into(),
                [1.0, 0.0, 0.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [0.5, -0.5, 0.5].into(),
                [1.0, 0.0, 1.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [0.5, 0.5, 0.5].into(),
                [1.0, 1.0, 1.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [-0.5, 0.5, 0.5].into(),
                [0.0, 1.0, 1.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
            VertexColor(
                [-0.5, -0.5, 0.5].into(),
                [0.0, 0.0, 1.0].into(),
                [0.2, 0.2, 0.2].into(),
            ),
        ];

        let index_list = [
            0, 1, 2, 2, 3, 0, //front
            4, 5, 6, 6, 7, 4, //back
            1, 6, 5, 5, 2, 1, //top
            7, 0, 3, 3, 4, 7, //bottom
            3, 2, 5, 5, 4, 3, //right
            7, 6, 1, 1, 0, 7, //left
        ];

        let mut window_inner = WindowInner::default();
        window_inner.hwnd = Some(hwnd);

        let graphics = Graphics::new(window_inner.hwnd.as_ref().unwrap()).unwrap();
        let (vertex_shader, blob) = graphics
            .device()
            .new_shader::<shaders::Vertex>("vertex_shader.hlsl")
            .unwrap();
        let (pixel_shader, _) = graphics
            .device()
            .new_shader::<shaders::Pixel>("pixel_shader.hlsl")
            .unwrap();
        let vertex_buffer = graphics
            .device()
            .new_vertex_buffer(&vertex_list, &blob)
            .unwrap();
        let index_buffer = graphics.device().new_index_buffer(&index_list).unwrap();
        let constant_buffer = graphics
            .device()
            .new_constant_buffer(&Constant {
                ..Default::default()
            })
            .unwrap();

        let app_window = AppWindow {
            window_inner,
            vertex_buffer,
            vertex_shader,
            pixel_shader,
            constant_buffer,
            index_buffer,
            delta_t: DeltaT::default(),
            delta_pos: 0.0,
            delta_scale: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            scale_cube: 1.0,
        };

        *GRAPHICS.lock().unwrap() = Some(graphics);

        *WINDOW.lock().unwrap() = Some(app_window);
        INPUT.lock().unwrap().add_listener(&*WINDOW);
    }

    fn on_update(&mut self) {
        if let Some(g) = GRAPHICS.lock().unwrap().as_mut() {
            let context = g.immediate_context();
            context.clear_render_target_color(g.swapchain(), 1.0, 0.0, 0.0, 1.0);
            let (width, height) = self.window_inner.hwnd.as_ref().unwrap().rect();
            context.set_viewport_size(width as f32, height as f32);

            self.update_quad_position(context);

            context.set_shader(&mut self.vertex_shader);
            context.set_shader(&mut self.pixel_shader);
            context.set_vertex_buffer(&mut self.vertex_buffer);
            context.set_index_buffer(&mut self.index_buffer);
            context.draw_indexed_triangle_list(self.index_buffer.len(), 0, 0);

            g.resize().unwrap();
            g.swapchain().present(0);

            self.delta_t.update();
        }
    }

    fn on_destroy(&mut self) {
        GRAPHICS.lock().unwrap().take();
    }

    fn on_focus(window: &'static Mutex<Option<AppWindow>>) {
        INPUT.lock().unwrap().add_listener(window)
    }

    fn on_kill_focus(window: &'static Mutex<Option<AppWindow>>) {
        INPUT.lock().unwrap().remove_listener(window)
    }
}

impl Listener for AppWindow {
    fn name(&self) -> &'static str {
        "AppWindow"
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => self.rot_x += 3.0 * self.delta_t.get(),
            b'S' => self.rot_x -= 3.0 * self.delta_t.get(),
            b'A' => self.rot_y += 3.0 * self.delta_t.get(),
            b'D' => self.rot_y -= 3.0 * self.delta_t.get(),
            _ => {}
        }
    }
    fn on_key_up(&mut self, _key: usize) {}
    fn on_mouse_move(&mut self, point: Point) {
        self.rot_x -= point.y as f32 / 200.0;
        self.rot_y -= point.x as f32 / 200.0;
    }
    fn on_left_mouse_down(&mut self) {
        self.scale_cube = 0.5
    }
    fn on_right_mouse_down(&mut self) {
        self.scale_cube = 1.5
    }
    fn on_left_mouse_up(&mut self) {
        self.scale_cube = 1.0
    }
    fn on_right_mouse_up(&mut self) {
        self.scale_cube = 1.0
    }
}

impl AppWindow {
    fn update_quad_position(&mut self, context: &Context) {
        let (width, height) = self.window_inner().hwnd.as_ref().unwrap().rect();
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
        let mut world = Matrix4x4::scaling([self.scale_cube, self.scale_cube, self.scale_cube]);
        world *= Matrix4x4::rotation_z(0.0);
        world *= Matrix4x4::rotation_y(self.rot_y);
        world *= Matrix4x4::rotation_x(self.rot_x);

        let view = Matrix4x4::identity();
        let proj = Matrix4x4::orthoganal(width as f32 / 300.0, height as f32 / 300.0, -4.0, 4.0);

        let mut constant = Constant {
            world,
            view,
            proj,
            time: get_tick_count(),
        };
        self.constant_buffer.update(context, &mut constant);
        context.set_constant_buffer::<shaders::Vertex, _>(&mut self.constant_buffer);
        context.set_constant_buffer::<shaders::Pixel, _>(&mut self.constant_buffer);
    }
}
