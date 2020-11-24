use engine::components::Camera;
use engine::graphics::color;
use engine::graphics::material::Material;
use engine::graphics::render::Render;
use engine::graphics::resource::mesh::Mesh;
use engine::input::{self, Listener};
use engine::math::{Matrix4x4, Point, Vector3d};
use engine::physics::Position;
use engine::physics::collision3::{CollisionEngine, GjkEngine, Sphere};
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

    entities: Vec<Entity>,
    sky_entity: Option<Entity>,

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

#[derive(Clone)]
pub struct Entity {
    pub mesh: Mesh,
    pub materials: Vec<Material>,

    pub position: Position,
    pub color: Vector3d,
}

impl Entity {
    pub fn new(mesh: Mesh, materials: impl IntoIterator<Item=Material>, position: Position) -> Self {
        let materials: Vec<_> = materials.into_iter().collect();
        Entity {
            mesh,
            materials,
            position,
            color: color::WHITE.into(),
        }
    }

    pub fn update(&mut self, delta_t: f32) {
        self.position.update(delta_t);
    }

    pub fn get_mesh_and_materials<'a, 'b>(&'a mut self, render: &'b Render) -> (&'a mut Mesh, &'a mut [Material]) {

        for material in self.materials.iter_mut() {
            //Datum 1 is position. How to label?
            material.set_data(render, 1, &mut self.position.get_matrix()).unwrap();
            //Datum 2 is color. Mostly unused
            material.set_data(render, 2, &mut MeshInfo { color: self.color }).unwrap();
        }

        (&mut self.mesh, &mut self.materials)
    }
}

impl World {
    pub fn new() -> World {
        let mut camera = Camera::default();
        camera.move_forward(-2.0);
        camera.move_up(1.0);
        //let light_source = Matrix4x4::rotation_x(-std::f32::consts::PI / 6.0);
        let light_source = Matrix4x4::translation([100.0, 100.0, 100.0]);

        World {
            scale_cube: 1.0,
            camera,
            light_source,
            light_rad: 40000.0,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let delta_t = self.delta_t.update().get();
        self.camera.update(delta_t);

        for entity in self.entities.iter_mut() {

            entity.update(delta_t);

            let position = entity.position.get_location();

            entity.color = color::WHITE.into();
            let sphere = Sphere::new(position, 0.5);
            if GjkEngine.collision_between(&self.camera, &sphere) {
                entity.color = color::RED.into()
            };
        }

        // Update Skysphere
        if let Some(entity) = self.sky_entity.as_mut() {
            let position = self.camera.get_skysphere();
            entity.position.set_matrix(position);
        }
        
        //self.light_source *= Matrix4x4::rotation_y(1.0 * delta_t);
        self.time += delta_t;
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
        }
    }

    pub fn set_screen_size(&mut self, (width, height): (u32, u32)) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity)
    }

    pub fn meshes_and_materials<'a, 'b>(&'a mut self, render: &'b Render) -> impl Iterator<Item = (&'a mut Mesh, &'a mut [Material])> {
        let vec: Vec<_> = self.entities.iter_mut()
            .chain(self.sky_entity.as_mut())
            .map(|entity| entity.get_mesh_and_materials(render))
            .collect();
        vec.into_iter()
    }

    pub fn set_environment_data(&mut self, render: &Render, data: &mut Environment) {
        for entity in self.entities.iter_mut()
            .chain(self.sky_entity.as_mut())
        {
            for material in entity.materials.iter_mut() {
                material.set_data(render, 0, data).unwrap();
            }
        }
    }

    pub fn add_sky_entity(&mut self, sky_entity: Entity) {
        self.sky_entity = Some(sky_entity)
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
