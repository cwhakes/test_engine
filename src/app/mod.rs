mod world;

use world::{World, Environment, MeshInfo};

use engine::graphics::color;
use engine::graphics::render::shader::{self, Shader};
use engine::graphics::render::{ConstantBuffer, SwapChain, WindowState};
use engine::graphics::resource::Texture;
use engine::graphics::GRAPHICS;
use engine::input::INPUT;
use engine::math::{Matrix4x4, Point};
use engine::physics::collision3::{CollisionEngine, GjkEngine, Sphere};
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
}

#[derive(Listener)]
#[listener(on_key_up)]
pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    window_state: WindowState,
    vs: Shader<shader::Vertex>,
    ps: Shader<shader::Pixel>,
    sky_ps: Shader<shader::Pixel>,
    environment: ConstantBuffer<Environment>,
    position: ConstantBuffer<Matrix4x4>,
    color: ConstantBuffer<MeshInfo>,
    wood_tex: Texture,
    sky_tex: Texture,
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
            .new_shader::<shader::Vertex, _>("vertex_shader.hlsl")
            .unwrap();
        let (pixel_shader, _) = render
            .device()
            .new_shader::<shader::Pixel, _>("pixel_shader.hlsl")
            .unwrap();
        let (skybox_shader, _) = render
            .device()
            .new_shader::<shader::Pixel, _>("skybox_shader.hlsl")
            .unwrap();
        let environment = render
            .device()
            .new_constant_buffer(0, Environment::default())
            .unwrap();
        let position = render
            .device()
            .new_constant_buffer(1, Matrix4x4::default())
            .unwrap();
        let color = render
            .device()
            .new_constant_buffer(2, MeshInfo::default())
            .unwrap();
        let wood_tex = graphics
            .get_texture_from_file("assets\\Textures\\brick.png")
            .unwrap();
        let sky_tex = graphics
            .get_texture_from_file("assets\\Textures\\sky.jpg")
            .unwrap();
        let teapot = graphics
            .get_mesh_from_file("assets\\Meshes\\statue.obj")
            .unwrap();
        let sky_mesh = graphics
            .get_mesh_from_file("assets\\Meshes\\sphere.obj")
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
        // world.add_mesh(
        //     Matrix4x4::translation([-1.0, 0.0, 0.0]),
        //     teapot.clone(),
        // );
        world.add_sky_mesh(sky_mesh);

        let mut app_window = AppWindow {
            hwnd,
            swapchain: swapchain,
            window_state: WindowState::default(),
            vs: vertex_shader,
            ps: pixel_shader,
            sky_ps: skybox_shader,
            environment,
            position,
            color,
            wood_tex,
            sky_tex,
            variables: world,
        };

        app_window.variables.set_screen_size(app_window.hwnd.rect());

        WINDOW.set_application(app_window);
        graphics.render.device().debug().unwrap();
    }

    fn on_update(&mut self) {
        let mut g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(&mut self.swapchain, color::NICE_BLUE);
        let (width, height) = self.hwnd.rect();
        context.set_viewport_size(width as f32, height as f32);

        self.variables.update();
        self.environment.update(context, self.variables.environment());

        g.render.set_back_face_culling();
        let context = g.render.immediate_context();
        for (pos, mesh) in self.variables.meshes() {
            let mut color = color::WHITE.into();
            let sphere = Sphere::new(pos.get_translation(), 0.5);
            if GjkEngine.collision_between(&self.variables.camera, &sphere) {
                color = color::RED.into()
            };

            self.color.update(context, MeshInfo {
                color,
            });

            self.position.update(context, pos);

            context.draw_mesh_and_texture(&mesh, &mut self.wood_tex, &mut self.vs, &mut self.ps);
        }

        if let Some((pos, mesh)) = self.variables.sky_mesh() {
            g.render.set_front_face_culling();
            let context = g.render.immediate_context();
            self.position.update(context, pos);
            //self.position.update(context, Matrix4x4::scaling(10.0));
            context.draw_mesh_and_texture(&mesh, &mut self.sky_tex, &mut self.vs, &mut self.sky_ps);
        }

        self.swapchain.present(0);
    }

    fn on_destroy(&mut self) {
        //GRAPHICS.lock().unwrap().destroy();
    }

    fn on_focus(window: &'static Mutex<Option<AppWindow>>) {

        INPUT.lock().unwrap().add_listener(window);
    }

    fn on_kill_focus(window: &'static Mutex<Option<AppWindow>>) {
        INPUT.lock().unwrap().remove_listener(window);
    }

    fn on_resize(&mut self) {
        self.variables.set_screen_size(self.hwnd.rect());
        let graphics = GRAPHICS.lock().unwrap();
        self.swapchain.resize(graphics.render.device()).unwrap();
    }
}

impl AppWindow {
    fn on_key_up(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'F' => {
                self.window_state.toggle();
                let state = self.window_state.clone();
                self.swapchain.set_windowed_state(
                    GRAPHICS.lock().unwrap().render.device(),
                    state,
                ).unwrap();
            }
            _ => {}
        }
        self.on_resize()
    }
}
