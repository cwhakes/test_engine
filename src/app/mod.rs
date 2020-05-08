mod world;

use world::{World, Environment};

use engine::graphics::render::shader::{self, Shader};
use engine::graphics::render::{ConstantBuffer, SwapChain};
use engine::graphics::resource::{texture::Texture};
use engine::graphics::GRAPHICS;
use engine::input::{self, INPUT};
use engine::math::{Matrix4x4, Point};
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
}

#[derive(Listener)]
pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    vertex_shader: Shader<shader::Vertex>,
    pixel_shader: Shader<shader::Pixel>,
    constant_buffer: ConstantBuffer<Environment>,
    constant_buffer1: ConstantBuffer<Matrix4x4>,
    wood_tex: Texture,
    #[listener]
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
            .new_shader::<shader::Vertex>("vertex_shader.hlsl")
            .unwrap();
        let (pixel_shader, _) = render
            .device()
            .new_shader::<shader::Pixel>("pixel_shader.hlsl")
            .unwrap();
        let constant_buffer = render
            .device()
            .new_constant_buffer(0, Environment::default())
            .unwrap();
        let constant_buffer1 = render
            .device()
            .new_constant_buffer(1, Matrix4x4::identity())
            .unwrap();
        let wood_tex = graphics
            .get_texture_from_file("assets\\Textures\\brick.png".as_ref())
            .unwrap();
        let teapot = graphics
            .get_mesh_from_file("assets\\Meshes\\statue.obj".as_ref())
            .unwrap();

        let mut world = World::new();
        world.add_mesh(
            Matrix4x4::translation([0.0, 0.0, 0.0]),
            teapot.clone(),
        );
        world.add_mesh(
            Matrix4x4::translation([1.0, 0.0, 0.0]),
            teapot.clone(),
        );
        world.add_mesh(
            Matrix4x4::translation([-1.0, 0.0, 0.0]),
            teapot.clone(),
        );

        let mut app_window = AppWindow {
            hwnd,
            swapchain,
            vertex_shader,
            pixel_shader,
            constant_buffer,
            constant_buffer1,
            wood_tex,
            variables: world,
        };

        app_window.variables.set_screen_size(app_window.hwnd.rect());

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
        context.set_shader(&mut self.vertex_shader);
        context.set_shader(&mut self.pixel_shader);
        context.set_texture::<shader::Pixel>(&mut self.wood_tex);

        self.variables.update();
        let constant = self.variables.environment();
        self.constant_buffer.update(context, constant);

        for (pos, mesh) in self.variables.meshes() {
            self.constant_buffer1.update(context, pos.clone());
            context.draw_mesh(mesh);
        }

        self.swapchain.present(0);
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
        self.variables.set_screen_size(self.hwnd.rect());
        let graphics = GRAPHICS.lock().unwrap();
        self.swapchain.resize(graphics.render.device()).unwrap();
    }
}
