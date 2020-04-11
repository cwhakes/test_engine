use engine::graphics::render::shaders::{self, Shader};
use engine::graphics::render::{ConstantBuffer, Context, IndexBuffer, SwapChain, VertexBuffer};
use engine::graphics::GRAPHICS;
use engine::input::{self, Listener, INPUT};
use engine::math::{Matrix4x4, Point};
use engine::time::{get_tick_count, DeltaT};
use engine::vertex;
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
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

#[derive(Default)]
struct AppWindowVariables {
    delta_t: DeltaT,
    delta_pos: f32,
    delta_scale: f32,
    rot_x: f32,
    rot_y: f32,
    scale_cube: f32,
    forward: f32,
    rightward: f32,
    world_camera: Matrix4x4,
}

pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    vertex_buffer: VertexBuffer<VertexColor>,
    vertex_shader: Shader<shaders::Vertex>,
    pixel_shader: Shader<shaders::Pixel>,
    constant_buffer: ConstantBuffer<Constant>,
    index_buffer: IndexBuffer,
    variables: AppWindowVariables
}

impl Application for AppWindow {
    fn me() -> &'static Window<AppWindow> {
        &WINDOW
    }

    fn hwnd(&self) -> &Hwnd {
        &self.hwnd
    }

    fn hwnd_mut(&mut self) -> &mut Hwnd {
        &mut self.hwnd
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

        let mut graphics = GRAPHICS.lock().unwrap();
        let render = &mut graphics.render;
        let swapchain = render.device_mut().new_swapchain(&hwnd).unwrap();
        let (vertex_shader, blob) = render
            .device()
            .new_shader::<shaders::Vertex>("vertex_shader.hlsl")
            .unwrap();
        let (pixel_shader, _) = render
            .device()
            .new_shader::<shaders::Pixel>("pixel_shader.hlsl")
            .unwrap();
        let vertex_buffer = render
            .device()
            .new_vertex_buffer(&vertex_list, &blob)
            .unwrap();
        let index_buffer = render.device().new_index_buffer(&index_list).unwrap();
        let constant_buffer = render
            .device()
            .new_constant_buffer(&Constant {
                ..Default::default()
            })
            .unwrap();

        let app_window = AppWindow {
            hwnd,
            swapchain,
            vertex_buffer,
            vertex_shader,
            pixel_shader,
            constant_buffer,
            index_buffer,
            variables: AppWindowVariables::new(),
        };

        WINDOW.set_application(app_window);
        INPUT.lock().unwrap().add_listener(WINDOW.listener());
        input::show_cursor(false);
    }

    fn on_update(&mut self) {
        let g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(&self.swapchain, 1.0, 0.0, 0.0, 1.0);
        let (width, height) = self.hwnd.rect();
        context.set_viewport_size(width as f32, height as f32);

        self.variables.update(&mut self.constant_buffer, context, (width, height));

        context.set_shader(&mut self.vertex_shader);
        context.set_shader(&mut self.pixel_shader);
        context.set_vertex_buffer(&mut self.vertex_buffer);
        context.set_index_buffer(&mut self.index_buffer);
        context.draw_indexed_triangle_list(self.index_buffer.len(), 0, 0);

        self.swapchain.present(0);

        self.variables.delta_t.update();
    }

    fn on_destroy(&mut self) {
        //GRAPHICS.lock().unwrap().destroy();
    }

    fn on_focus(window: &'static Mutex<Option<AppWindow>>) {
        INPUT.lock().unwrap().add_listener(window);

        //TODO: Stop first move
    }

    fn on_kill_focus(window: &'static Mutex<Option<AppWindow>>) {
        INPUT.lock().unwrap().remove_listener(window)
    }

    fn on_resize(&mut self) {
        let graphics = GRAPHICS.lock().unwrap();
        self.swapchain.resize(graphics.render.device()).unwrap();
    }
}

impl Listener for AppWindow {
    fn name(&self) -> &'static str {
        "AppWindow"
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => self.variables.forward = 1.0 * self.variables.delta_t.get(),
            b'S' => self.variables.forward = -1.0 * self.variables.delta_t.get(),
            b'A' => self.variables.rightward = -1.0 * self.variables.delta_t.get(),
            b'D' => self.variables.rightward = 1.0 * self.variables.delta_t.get(),
            _ => {}
        }
    }
    fn on_key_up(&mut self, _key: usize) {
        self.variables.forward = 0.0;
        self.variables.rightward = 0.0;
    }
    fn on_mouse_move(&mut self, pos: Point) {
        let (width, height) = self.hwnd.rect();
        let (width, height) = (width as i32, height as i32);

        self.variables.rot_x += (pos.y - height/2) as f32 / 100.0;
        self.variables.rot_y += (pos.x - width/2) as f32 / 100.0;

        input::set_cursor_position((width/2, height/2));
    }
    fn on_left_mouse_down(&mut self) {
        self.variables.scale_cube = 0.5
    }
    fn on_right_mouse_down(&mut self) {
        self.variables.scale_cube = 1.5
    }
    fn on_left_mouse_up(&mut self) {
        self.variables.scale_cube = 1.0
    }
    fn on_right_mouse_up(&mut self) {
        self.variables.scale_cube = 1.0
    }
}

impl AppWindowVariables {
    fn new() -> AppWindowVariables {
        AppWindowVariables {
            scale_cube: 1.0,
            world_camera: Matrix4x4::translation([0.0, 0.0, -2.0]),
            ..Default::default()
        }
    }

    fn update(&mut self, constant_buffer: &mut ConstantBuffer<Constant>, context: &Context, (width, height): (u32, u32)) {
        self.delta_pos += self.delta_t.get() / 10.0;
        if self.delta_pos > 1.0 {
            self.delta_pos -= 1.0;
        }
        self.delta_scale += self.delta_t.get() / 1.0;

        let world = Matrix4x4::identity();
        
        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(self.rot_x);
        world_cam *= Matrix4x4::rotation_y(self.rot_y);

        let new_pos = self.world_camera.get_translation()
            + world_cam.get_direction_z() * (self.forward * 5.0)
            + world_cam.get_direction_x() * (self.rightward * 5.0);

        world_cam.set_translation(new_pos);
        self.world_camera = world_cam.clone();

        let view = world_cam.inverse().unwrap();

        let proj = Matrix4x4::perspective(0.785, width as f32/height as f32, 0.1, 100.0);

        let mut constant = Constant {
            world,
            view,
            proj,
            time: get_tick_count(),
        };
        constant_buffer.update(context, &mut constant);
        context.set_constant_buffer::<shaders::Vertex, _>(constant_buffer);
        context.set_constant_buffer::<shaders::Pixel, _>(constant_buffer);
    }
}
