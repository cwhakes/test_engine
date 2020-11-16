use engine::components::Camera;
use engine::graphics::resource::mesh::Mesh;
use engine::input::{self, Listener};
use engine::math::{Matrix4x4, Point, Vector3d};
use engine::time::DeltaT;

use crate::shaders::point_light::Environment;

static SPEED: f32 = 5.0;

#[derive(Default)]
pub struct World {
    screen_width: f32,
    screen_height: f32,

    play_state: PlayState,

    delta_t: DeltaT,
    pub scale_cube: f32,
    world_matrix: Matrix4x4,
    pub camera: Camera,
    pub light_source: Matrix4x4,

    time: f32,

    meshes: Vec<(Matrix4x4, Mesh)>,
    sky_mesh: Option<Mesh>,

    light_rad: f32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum PlayState {
    Playing,
    NotPlaying,
}

impl PlayState {
    fn toggle(&mut self) {
        match self {
            PlayState::Playing => {
                input::show_cursor(true);
                *self = PlayState::NotPlaying;
            }
            PlayState::NotPlaying => {
                input::show_cursor(false);
                *self = PlayState::Playing;
            }
        }
    }
}

impl Default for PlayState {
    fn default() -> Self {
        PlayState::NotPlaying
    }
}

#[derive(Default, Debug)]
#[repr(C, align(16))]
pub struct MeshInfo {
    pub color: Vector3d,
}

impl World {
    pub fn new() -> World {
        let mut camera = Camera::default();
        camera.move_forward(-2.0);
        camera.move_up(1.0);
        //let light_source = Matrix4x4::rotation_x(-std::f32::consts::PI / 6.0);
        let light_source = Matrix4x4::translation([0.0, 1.0, 2.0]);

        World {
            scale_cube: 1.0,
            camera,
            light_source,
            light_rad: 4.0,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        self.delta_t.update();
        
        self.light_source *= Matrix4x4::rotation_y(1.0 * self.delta_t.get());
        self.time += self.delta_t.get();

        self.camera.update(self.delta_t.get());

    }

    pub fn environment(&self) -> Environment {
        let mut world = self.world_matrix.clone();
        world *= Matrix4x4::scaling(self.scale_cube);

        let view = self.camera.get_view();
        let proj = self.camera.get_proj(self.screen_width / self.screen_height);

        let light_dir = self.light_source.get_direction_z().to_4d(0.0);
        let camera_pos = self.camera.get_location();
        let light_pos = self.light_source.get_translation().to_4d(1.0);

        Environment {
            view,
            proj,
            light_dir,
            camera_pos,
            light_pos,

            time: self.time,
            light_rad: self.light_rad,

            ..Environment::new()
        }
    }

    pub fn set_screen_size(&mut self, (width, height): (u32, u32)) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;
    }

    pub fn add_mesh(&mut self, position: Matrix4x4, mesh: Mesh) {
        self.meshes.push((position, mesh))
    }

    pub fn meshes(&self) -> impl Iterator<Item=(Matrix4x4, Mesh)> {
        self.meshes.clone().into_iter()
    }

    pub fn add_sky_mesh(&mut self, sky_mesh: Mesh) {
        self.sky_mesh = Some(sky_mesh)
    }

    pub fn sky_mesh(&self) -> Option<(Matrix4x4, Mesh)> {
        if let Some(mesh) = self.sky_mesh.clone() {
            Some((
                self.camera.get_skysphere(),
                mesh,
            ))
        } else { None }
    }
}

impl Listener for World {
    fn name(&self) -> String {
        "World".to_string()
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => {self.camera.moving_forward(SPEED);}
            b'S' => {self.camera.moving_forward(-SPEED);}
            b'A' => {self.camera.moving_rightward(-SPEED);}
            b'D' => {self.camera.moving_rightward(SPEED);}
            b'O' => {self.light_rad -= 5.0 * self.delta_t.get();}
            b'P' => {self.light_rad += 5.0 * self.delta_t.get();}
            _ => {}
        }
    }
    fn on_key_up(&mut self, key: usize) {
        self.camera.reset_velocity();

        let key = key as u8;
        match key {
            b'G' => {self.play_state.toggle();}
            _ => {}
        }
    }
    fn on_mouse_move(&mut self, pos: Point) {
        if self.play_state == PlayState::Playing {

            let (width, height) = (self.screen_width as i32, self.screen_height as i32);

            self.camera.tilt( (pos.y - height / 2) as f32 * 0.002);
            self.camera.pan( (pos.x - width / 2) as f32 * 0.002);

            input::set_cursor_position((width / 2, height / 2));
        }
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
