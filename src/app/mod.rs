mod world;

use world::{World, MeshInfo, Entity};
use crate::shaders::point_light::{self, Environment};

use engine::error::Result;
use engine::graphics::color;
use engine::graphics::render::{SwapChain, WindowState};
use engine::graphics::material::Material;
use engine::graphics::GRAPHICS;
use engine::input::INPUT;
use engine::math::{Matrix4x4, Point};
use engine::physics::collision3::{CollisionEngine, GjkEngine, Sphere};
use engine::physics::Position;
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
    //material: Material,
    sky_material: Material,
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

    fn on_create(hwnd: Hwnd) -> Result<()> {
        let mut graphics = GRAPHICS.lock().unwrap();
        let device = &mut graphics.render.device_mut();
        let swapchain = device.new_swapchain(&hwnd).unwrap();

        let mut material = device.new_material(point_light::VERTEX_SHADER_PATH, point_light::PIXEL_SHADER_PATH)?;
        let mut sky_material = device.new_material(point_light::VERTEX_SHADER_PATH, "shaders\\skybox_shader.hlsl")?.with_frontface_culling();

        material.set_data(&graphics.render, 0, &mut Environment::default())?;
        material.set_data(&graphics.render, 1, &mut Matrix4x4::default())?;
        material.set_data(&graphics.render, 2, &mut MeshInfo::default())?;

        sky_material.set_data(&graphics.render, 0, &mut Environment::default())?;
        sky_material.set_data(&graphics.render, 1, &mut Matrix4x4::default())?;
        
        material.add_texture(&graphics.get_texture_from_file("assets\\Textures\\wall.jpg")?);
        sky_material.add_texture(&graphics.get_texture_from_file("assets\\Textures\\stars_map.jpg")?);
        
        let teapot = graphics.get_mesh_from_file("assets\\Meshes\\scene.obj")?;
        let sky_mesh = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;

        let mut world = World::new();
        world.add_entity(Entity::new(
            teapot.clone(),
            material,
            Position::new(Matrix4x4::translation([0.0, 0.0, 0.0])),
        ));
        world.add_sky_mesh(sky_mesh);

        let mut app_window = AppWindow {
            hwnd,
            swapchain: swapchain,
            window_state: WindowState::default(),
            //material,
            sky_material,
            variables: world,
        };

        app_window.variables.set_screen_size(app_window.hwnd.rect());

        WINDOW.set_application(app_window);
        graphics.render.device().debug()?;

        Ok(())
    }

    fn on_update(&mut self) {
        let mut g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(&mut self.swapchain, color::NICE_BLUE);
        let (width, height) = self.hwnd.rect();
        context.set_viewport_size(width as f32, height as f32);

        self.variables.update();
        let mut environment = self.variables.environment();
        self.variables.set_environment_data(&g.render, &mut environment);
        //self.material.set_data(&g.render, 0, &mut environment).unwrap();
        self.sky_material.set_data(&g.render, 0, &mut environment).unwrap();

        for (mesh, material) in self.variables.meshes_and_materials(&g.render) {
            g.render.set_material(material);
            g.render.draw_mesh_and_material(mesh, material);
        }

        if let Some((pos, mesh)) = self.variables.sky_mesh() {
            self.sky_material.set_data(&g.render, 1, &mut pos.clone()).unwrap();
            g.render.draw_mesh_and_material(&mesh, &mut self.sky_material);
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
