use engine::components::{Camera, Entity, PlayState, Screen};
use engine::graphics::color;
use engine::graphics::material::Material;
use engine::graphics::render::Render;
use engine::graphics::resource::mesh::Mesh;
use engine::input::{self, Listener};
use engine::math::{Matrix4x4, Point};
use engine::physics::collision3::{CollisionEngine, GjkEngine, Sphere};
use engine::time::DeltaT;

use shader::Environment;

static SPEED: f32 = 5.0;

#[derive(Default)]
pub struct World {
    pub screen: Screen,

    play_state: PlayState,

    delta_t: DeltaT,
    pub scale_cube: f32,
    pub camera: Camera,
    pub light_source: Matrix4x4,

    time: f32,

    entities: Vec<Entity>,
    sky_entity: Option<Entity>,

    light_rad: f32,
}

impl World {
    pub fn new() -> Self {
        let mut camera = Camera::default();
        camera.move_forward(-2.0);
        camera.move_up(1.0);
        //let light_source = Matrix4x4::rotation_x(-std::f32::consts::PI / 6.0);
        let light_source = Matrix4x4::translation([100.0, 100.0, 100.0]);

        Self {
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

        for entity in &mut self.entities {
            entity.update(delta_t);

            let position = entity.position.get_location();

            entity.color = color::WHITE.into();
            let sphere = Sphere::new(position, 0.5);
            if GjkEngine.collision_between(&self.camera, &sphere) {
                entity.color = color::RED.into();
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
        let view = self.camera.get_view();
        let proj = self.camera.get_proj(self.screen.aspect_ratio());

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

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn meshes_and_materials<'a, 'b>(
        &'a mut self,
        render: &'b Render,
    ) -> impl Iterator<Item = (&'a mut Mesh, &'a mut [Material])> {
        let vec: Vec<_> = self
            .entities
            .iter_mut()
            .chain(self.sky_entity.as_mut())
            .map(|entity| entity.get_mesh_and_materials(render))
            .collect();
        vec.into_iter()
    }

    pub fn set_environment_data(&mut self, render: &Render, data: &mut Environment) {
        for entity in self.entities.iter_mut().chain(self.sky_entity.as_mut()) {
            for material in &mut entity.materials {
                material.set_data(render, 0, data).unwrap();
            }
        }
    }

    pub fn add_sky_entity(&mut self, sky_entity: Entity) {
        self.sky_entity = Some(sky_entity);
    }

    pub fn is_playing(&self) -> bool {
        self.play_state.is_playing()
    }
}

impl Listener for World {
    fn name(&self) -> String {
        "World".to_string()
    }

    fn on_key_down(&mut self, key: usize) {
        let key = key as u8;
        match key {
            b'W' => {
                self.camera.moving_forward(SPEED);
            }
            b'S' => {
                self.camera.moving_forward(-SPEED);
            }
            b'A' => {
                self.camera.moving_rightward(-SPEED);
            }
            b'D' => {
                self.camera.moving_rightward(SPEED);
            }
            b'O' => {
                self.light_rad -= 5.0 * self.delta_t.get();
            }
            b'P' => {
                self.light_rad += 5.0 * self.delta_t.get();
            }
            _ => {}
        }
    }
    fn on_key_up(&mut self, key: usize) {
        self.camera.reset_velocity();

        let key = key as u8;
        match key {
            input::key::ESCAPE => {
                if self.play_state.is_playing() {
                    self.play_state.set_not_playing()
                }
            }
            // b'G' => {
            //     self.play_state.toggle();
            // }
            _ => {}
        }
    }
    fn on_mouse_move(&mut self, pos: Point) {
        if self.play_state == PlayState::Playing {
            self.camera
                .tilt((pos.y - self.screen.rect.center_y()) as f32 * 0.002);
            self.camera
                .pan((pos.x - self.screen.rect.center_x()) as f32 * 0.002);

            self.screen.center_cursor();
        }
    }
    fn on_left_mouse_down(&mut self) {
        if self.play_state.is_not_playing() {
            self.play_state.set_playing();
            self.screen.center_cursor();
        }
    }
}
