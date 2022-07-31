mod world;

use shader::{DirectionalLight, Skybox};
use world::World;

use engine::components::Entity;
use engine::error::Result;
use engine::graphics::color;
use engine::graphics::render::{SwapChain, WindowState};
use engine::graphics::GRAPHICS;
use engine::input::INPUT;
use engine::math::{Matrix4x4, Point, Rect};
use engine::physics::Position;
use engine::window::{Application, Hwnd, Window};

use std::sync::Mutex;

use crate::minigame::MiniGame;

lazy_static! {
    pub static ref WINDOW: Window<AppWindow> = Window::new();
}

#[derive(Listener)]
#[listener(on_key_up, on_mouse_move)]
pub struct AppWindow {
    hwnd: Hwnd,
    swapchain: SwapChain,
    window_state: WindowState,
    #[listener]
    variables: World,
    #[listener]
    minigame: MiniGame,
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
        let minigame = MiniGame::new(Rect([0..1280, 0..720]))?;

        let mut graphics = GRAPHICS.lock().unwrap();
        let device = &mut graphics.render.device_mut();
        let swapchain = device.new_swapchain(&hwnd).unwrap();

        let mut world = World::new();

        let monitor_mesh = graphics.get_mesh_from_file("assets\\Meshes\\monitor.obj")?;

        let mut monitor_mat = graphics.new_material::<DirectionalLight>()?;
        monitor_mat.add_texture(graphics.get_texture_from_file("assets\\Textures\\brick_d.jpg")?);
        let mut screen_mat = graphics.new_material::<DirectionalLight>()?;
        screen_mat.add_texture(minigame.render_target.clone());

        world.add_entity(Entity::new(
            monitor_mesh,
            [monitor_mat, screen_mat],
            Position::new(Matrix4x4::rotation_y(std::f32::consts::PI)),
        ));

        let mut sky_material = graphics.new_material::<Skybox>()?.with_frontface_culling();
        sky_material
            .add_texture(graphics.get_texture_from_file("assets\\Textures\\stars_map.jpg")?);

        let sky_mesh = graphics.get_mesh_from_file("assets\\Meshes\\sphere.obj")?;

        world.add_sky_entity(Entity::new(
            sky_mesh,
            Some(sky_material),
            Position::default(),
        ));

        let mut app_window = Self {
            hwnd,
            swapchain,
            window_state: WindowState::default(),
            variables: world,
            minigame,
        };

        app_window.variables.screen.set_size(app_window.hwnd.rect());

        WINDOW.set_application(app_window);
        graphics.render.device().debug()?;

        Ok(())
    }

    fn on_update(&mut self) {
        self.minigame.update();

        let mut g = GRAPHICS.lock().unwrap();
        let context = g.render.immediate_context();
        context.clear_render_target_color(&mut self.swapchain, color::NICE_BLUE);
        context.set_render_target(&mut self.swapchain);
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
        self.variables.screen.set_size(self.hwnd.rect());
        if self.variables.is_playing() {
            self.variables.screen.center_cursor();
        }
        let graphics = GRAPHICS.lock().unwrap();
        self.swapchain.resize(graphics.render.device()).unwrap();
    }

    fn on_move(&mut self) {
        self.variables.screen.set_size(self.hwnd.rect());
        if self.variables.is_playing() {
            self.variables.screen.center_cursor();
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
        self.on_resize();
    }

    fn on_mouse_move(&mut self, pos: Point) {
        if self.minigame.variables.play_state.is_playing() {
            self.minigame.variables.delta_mouse_x =
                (pos.x - self.variables.screen.rect.center_x()) as f32;
            self.minigame.variables.delta_mouse_y =
                (pos.y - self.variables.screen.rect.center_y()) as f32;

            self.variables.screen.center_cursor();
        }
    }
}
