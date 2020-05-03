use engine::graphics::render::shaders::{self, Shader};
use engine::graphics::render::{ConstantBuffer, Context, SwapChain};
use engine::graphics::resource::{mesh::Mesh, texture::Texture};
use engine::graphics::GRAPHICS;
use engine::input::{self, Listener, INPUT};
use engine::math::{Matrix4x4, Point, Vector4d};
use engine::time::DeltaT;
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
}

#[repr(C, align(16))]
#[derive(Default, Debug)]
struct Constant {
    world: Matrix4x4,
    view: Matrix4x4,
    proj: Matrix4x4,
    light_dir: Vector4d,
    camera_pos: Vector4d,
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
    light_source: Matrix4x4,
}

pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    vertex_shader: Shader<shaders::Vertex>,
    pixel_shader: Shader<shaders::Pixel>,
    constant_buffer: ConstantBuffer<Constant>,
    wood_tex: Texture,
    teapot: Mesh,
    variables: AppWindowVariables,
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
        let mut graphics = GRAPHICS.lock().unwrap();
        let render = &mut graphics.render;
        let swapchain = render.device_mut().new_swapchain(&hwnd).unwrap();
        let (vertex_shader, _) = render
            .device()
            .new_shader::<shaders::Vertex>("vertex_shader.hlsl")
            .unwrap();
        let (pixel_shader, _) = render
            .device()
            .new_shader::<shaders::Pixel>("pixel_shader.hlsl")
            .unwrap();
        let constant_buffer = render
            .device()
            .new_constant_buffer(&Constant {
                ..Default::default()
            })
            .unwrap();
        let wood_tex = graphics
            .get_texture_from_file("assets\\Textures\\brick.png".as_ref())
            .unwrap();
        let teapot = graphics
            .get_mesh_from_file("assets\\Meshes\\statue.obj".as_ref())
            .unwrap();

        let app_window = AppWindow {
            hwnd,
            swapchain,
            vertex_shader,
            pixel_shader,
            constant_buffer,
            wood_tex,
            teapot,
            variables: AppWindowVariables::new(),
        };

        WINDOW.set_application(app_window);
        INPUT.lock().unwrap().add_listener(WINDOW.listener());
        input::show_cursor(false);
        graphics.render.device().debug().unwrap();
    }

    fn on_update(&mut self) {
        let g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(&mut self.swapchain, 0.2, 0.4, 0.8, 1.0);
        let (width, height) = self.hwnd.rect();
        context.set_viewport_size(width as f32, height as f32);

        self.variables
            .update(&mut self.constant_buffer, context, (width, height));

        context.set_shader(&mut self.vertex_shader);
        context.set_shader(&mut self.pixel_shader);
        context.set_texture::<shaders::Pixel>(&mut self.wood_tex);
        context.set_vertex_buffer(&mut self.teapot.inner().vertex_buffer);
        context.set_index_buffer(&mut self.teapot.inner().index_buffer);
        context.draw_indexed_triangle_list(self.teapot.inner().index_buffer.len(), 0, 0);

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
            b'W' => self.variables.forward = 0.1 * self.variables.delta_t.get(),
            b'S' => self.variables.forward = -0.1 * self.variables.delta_t.get(),
            b'A' => self.variables.rightward = -0.1 * self.variables.delta_t.get(),
            b'D' => self.variables.rightward = 0.1 * self.variables.delta_t.get(),
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

        self.variables.rot_x += (pos.y - height / 2) as f32 * 0.002;
        self.variables.rot_y += (pos.x - width / 2) as f32 * 0.002;

        input::set_cursor_position((width / 2, height / 2));
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
            world_camera: Matrix4x4::translation([0.0, 0.0, -1.0]),
            ..Default::default()
        }
    }

    fn update(
        &mut self,
        constant_buffer: &mut ConstantBuffer<Constant>,
        context: &Context,
        (width, height): (u32, u32),
    ) {
        //self.delta_pos += self.delta_t.get() / 10.0;
        //if self.delta_pos > 1.0 {
        //    self.delta_pos -= 1.0;
        //}
        self.delta_scale += self.delta_t.get() / 1.0;
        self.light_source *= Matrix4x4::rotation_y(1.0 * self.delta_t.get());

        let world = Matrix4x4::scaling([self.scale_cube, self.scale_cube, self.scale_cube]);

        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(self.rot_x);
        world_cam *= Matrix4x4::rotation_y(self.rot_y);

        let new_pos = self.world_camera.get_translation()
            + world_cam.get_direction_z() * (self.forward * 5.0)
            + world_cam.get_direction_x() * (self.rightward * 5.0);

        world_cam.set_translation(new_pos);
        self.world_camera = world_cam.clone();

        let view = world_cam.inverse().unwrap();

        let proj = Matrix4x4::perspective(0.785, width as f32 / height as f32, 0.001, 100.0);

        let light_dir = self.light_source.get_direction_z().to_4d(0.0);
        let camera_pos = self.world_camera.get_translation().to_4d(1.0);

        let mut constant = Constant {
            world,
            view,
            proj,
            light_dir,
            camera_pos,
        };
        constant_buffer.update(context, &mut constant);
        context.set_constant_buffer::<shaders::Vertex, _>(constant_buffer);
        context.set_constant_buffer::<shaders::Pixel, _>(constant_buffer);
    }
}
