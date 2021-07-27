mod world;

use rand::{prelude::*, distributions::uniform::Uniform};
use shader::directional_light;
use world::{Entity, World};

use engine::error::Result;
use engine::graphics::color;
use engine::graphics::render::{SwapChain, WindowState};
use engine::graphics::GRAPHICS;
use engine::input::INPUT;
use engine::math::{Matrix4x4, Point, Vector3d};
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

    asteroids_pos: Vec<(Vector3d, Vector3d, Vector3d)>,
}

impl Application for AppWindow {
    fn me() -> &'static Window<Self> {
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

        let material = graphics.new_material(
            directional_light::VERTEX_SHADER_PATH,
            directional_light::PIXEL_SHADER_PATH,
        )?;

        let spaceship = graphics.get_mesh_from_file("assets\\Meshes\\spaceship.obj")?;
        let mut spaceship_mat = material.clone();
        spaceship_mat
            .add_texture(&graphics.get_texture_from_file("assets\\Textures\\spaceship.jpg")?);

        world.add_entity(
            "ship".into(),
            Entity::new(
                spaceship,
                vec![spaceship_mat],
                Position::new(Matrix4x4::translation([0.0, 0.0, 0.0])),
            ),
        );



        let mut sky_material = graphics
            .new_material(
                "shaders\\skybox\\vertex_shader.hlsl",
                "shaders\\skybox\\pixel_shader.hlsl",
            )?
            .with_frontface_culling();
        sky_material
            .add_texture(&graphics.get_texture_from_file("assets\\Textures\\stars_map.jpg")?);

        let sky_mesh = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;

        world.add_sky_entity(Entity::new(
            sky_mesh,
            Some(sky_material),
            Position::default(),
        ));

        let mut asteroids_pos = Vec::new();

        let asteroid = graphics.get_mesh_from_file("assets\\Meshes\\asteroid.obj")?;
        let mut asteroid_mat = material;
        asteroid_mat
            .add_texture(&graphics.get_texture_from_file("assets\\Textures\\asteroid.jpg")?);

        let mut rng = rand::thread_rng();
        let loc_range = Uniform::new(-2000.0, 2000.0);
        let rot_range = Uniform::new(0.0, 6.28);
        let scale_range = Uniform::new(1.0, 10.0);
        for i in 0..200 {
            let loc = Vector3d::new(rng.sample(&loc_range), rng.sample(&loc_range), rng.sample(&loc_range));
            let rot = Vector3d::new(rng.sample(&rot_range), rng.sample(&rot_range), rng.sample(&rot_range));
            let scale = rng.sample(&scale_range);
            let scale = Vector3d::new(scale, scale, scale);

            let mut pos= Position::default();
            pos.set_postition(scale, rot, loc);

            world.add_entity(
                format!("asteroid_{}", i).into(),
                Entity::new(
                    asteroid.clone(),
                    vec![asteroid_mat.clone()],
                    pos,
                ),
            );

            asteroids_pos.push((loc, rot, scale));
        }

        let mut app_window = Self {
            hwnd,
            swapchain,
            window_state: WindowState::default(),
            variables: world,
            asteroids_pos,
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
        let (width, height) = self.hwnd.rect().dims();
        context.set_viewport_size(width as f32, height as f32);

        self.variables.update();
        let mut environment = self.variables.environment();
        self.variables
            .set_environment_data(&g.render, &mut environment);

        for (mesh, materials) in self.variables.meshes_and_materials(&g.render) {
            g.render.draw_mesh_and_materials(mesh, materials);
        }

        self.swapchain.present(0);
    }

    fn on_destroy(&mut self) {
        //GRAPHICS.lock().unwrap().destroy();
    }

    fn on_focus(window: &'static Mutex<Option<Self>>) {
        INPUT.lock().unwrap().add_listener(window);
    }

    fn on_kill_focus(window: &'static Mutex<Option<Self>>) {
        INPUT.lock().unwrap().remove_listener(window);
    }

    fn on_resize(&mut self) {
        self.variables.set_screen_size(self.hwnd.rect());
        if self.variables.is_playing() {
            self.variables.center_cursor()
        }
        let graphics = GRAPHICS.lock().unwrap();
        self.swapchain.resize(graphics.render.device()).unwrap();
    }

    fn on_move(&mut self) {
        self.variables.set_screen_size(self.hwnd.rect());
        if self.variables.is_playing() {
            self.variables.center_cursor()
        }
    }
}

impl AppWindow {
    fn on_key_up(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'F' => {
                self.window_state.toggle();
                let state = self.window_state;
                self.swapchain
                    .set_windowed_state(GRAPHICS.lock().unwrap().render.device(), state)
                    .unwrap();
            }
            _ => {}
        }
        self.on_resize()
    }
}
