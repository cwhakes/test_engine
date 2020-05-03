mod world;

use world::World;

use engine::graphics::render::shaders::{self, Shader};
use engine::graphics::render::{ConstantBuffer, SwapChain};
use engine::graphics::resource::{mesh::Mesh, texture::Texture};
use engine::graphics::GRAPHICS;
use engine::input::{self, Listener, INPUT};
use engine::math::{Matrix4x4, Point, Vector4d};
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
}

#[repr(C, align(16))]
#[derive(Default, Debug)]
pub struct Constant {
    world: Matrix4x4,
    view: Matrix4x4,
    proj: Matrix4x4,
    light_dir: Vector4d,
    camera_pos: Vector4d,
}

pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    vertex_shader: Shader<shaders::Vertex>,
    pixel_shader: Shader<shaders::Pixel>,
    constant_buffer: ConstantBuffer<Constant>,
    wood_tex: Texture,
    teapot: Mesh,
    variables: World,
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
            variables: World::new(),
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
