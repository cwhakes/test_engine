mod world;

use world::{World, Entity};
use crate::shaders::point_light;

use engine::error::Result;
use engine::graphics::color;
use engine::graphics::render::{SwapChain, WindowState};
use engine::graphics::GRAPHICS;
use engine::input::INPUT;
use engine::math::{Matrix4x4, Point};
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

        let mut world = World::new();

        let material = graphics.new_material(point_light::VERTEX_SHADER_PATH, point_light::PIXEL_SHADER_PATH)?;
        
        let sphere = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;
        let torus = graphics.get_mesh_from_file("assets\\Meshes\\torus.obj")?;
        let suzanne = graphics.get_mesh_from_file("assets\\Meshes\\suzanne.obj")?;
        let plane = graphics.get_mesh_from_file("assets\\Meshes\\plane2.obj")?;
        
        let mut stone = material.clone();
        stone.add_texture(&graphics.get_texture_from_file("assets\\Textures\\wall.jpg")?);
        let mut brick = material.clone();
        brick.add_texture(&graphics.get_texture_from_file("assets\\Textures\\brick.png")?);
        let mut earth = material.clone();
        earth.add_texture(&graphics.get_texture_from_file("assets\\Textures\\earth_color.jpg")?);

        let mut entity_0 = Entity::new(sphere, stone.clone(), Position::new(Matrix4x4::translation([0.0, 2.0, 0.0])));
        let mut entity_1 = Entity::new(torus, brick, Position::new(Matrix4x4::translation([5.0, 2.0, 0.0])));
        let mut entity_2 = Entity::new(suzanne, earth, Position::new(Matrix4x4::translation([-5.0, 2.0, 0.0])));

        for _ in 0..3 {
            world.add_entity(entity_0.clone());
            entity_0.position.move_forward(4.0);
            world.add_entity(entity_1.clone());
            entity_1.position.move_forward(4.0);
            world.add_entity(entity_2.clone());
            entity_2.position.move_forward(4.0);
        }

        world.add_entity(Entity::new(plane, stone, Position::default()));
        
        let mut sky_material = graphics.new_material(point_light::VERTEX_SHADER_PATH, "shaders\\skybox_shader.hlsl")?.with_frontface_culling();
        sky_material.add_texture(&graphics.get_texture_from_file("assets\\Textures\\stars_map.jpg")?);

        let sky_mesh = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;

        world.add_sky_entity(Entity::new(
            sky_mesh.clone(),
            sky_material,
            Position::default(),
        ));

        let mut app_window = AppWindow {
            hwnd,
            swapchain: swapchain,
            window_state: WindowState::default(),
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

        for (mesh, material) in self.variables.meshes_and_materials(&g.render) {
            g.render.set_material(material);
            g.render.draw_mesh_and_materials(mesh, std::slice::from_mut(material));
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
