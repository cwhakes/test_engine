use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use engine::components::{Camera0, Entity, PlayState, Screen, SpaceShip};
use engine::graphics::material::Material;
use engine::graphics::render::Render;
use engine::graphics::resource::mesh::Mesh;
use engine::input::{self, Listener};
use engine::math::{Matrix4x4, Point, Rect};
//use engine::physics::collision3::{CollisionEngine, GjkEngine, Sphere};
use engine::time::DeltaT;

use shader::Environment;

#[derive(Default)]
pub struct World {
    pub screen: Screen,

    play_state: PlayState,

    delta_t: DeltaT,
    //pub camera: ThirdPersonCamera,
    pub camera: Camera0,
    pub spaceship: SpaceShip,
    pub light_source: Matrix4x4,

    delta_mouse_x: f32,
    delta_mouse_y: f32,

    time: f32,

    entities: HashMap<Cow<'static, str>, Entity>,
    light_rad: f32,
}

impl World {
    pub fn new() -> Self {
        let camera = Camera0::new();
        let spaceship = SpaceShip::new();

        let mut light_source = Matrix4x4::identity();
        light_source *= Matrix4x4::rotation_x(-0.707);
        light_source *= Matrix4x4::rotation_y(0.707);

        Self {
            camera,
            spaceship,
            light_source,
            light_rad: 40000.0,
            ..Default::default()
        }
    }

    pub fn update(&mut self) {
        let delta_t = self.delta_t.update().get();

        for entity in self.entities.values_mut() {
            entity.update(delta_t);

            //let position = entity.position.get_location();

            //entity.color = color::WHITE.into();
            //let sphere = Sphere::new(position, 0.5);
            // if GjkEngine.collision_between(&self.camera, &sphere) {
            //     entity.color = color::RED.into()
            // };
        }

        self.spaceship
            .update(delta_t, self.delta_mouse_x, self.delta_mouse_y);
        self.camera.set_focus(
            self.spaceship.current_spaceship_pos,
            self.spaceship.spaceship_rot,
        );

        if let Some(ship) = self.entities.get_mut("ship") {
            ship.position.set_postition(
                [1.0, 1.0, 1.0],
                self.spaceship.current_spaceship_rot,
                self.spaceship.current_spaceship_pos,
            );
        }

        self.camera.update(delta_t);

        // Update Skysphere
        if let Some(entity) = self.entities.get_mut("skybox") {
            let position = self.camera.get_skysphere();
            entity.position.set_matrix(position);
        }

        //self.light_source *= Matrix4x4::rotation_y(1.0 * delta_t);
        self.time += delta_t;
    }

    pub fn environment(&self) -> Environment {
        let view = self.camera.view_cam();
        let proj = self.camera.proj_cam(Rect::<f32>::from(&self.screen.rect));

        let light_dir = self.light_source.get_direction_z().to_4d(0.0);
        let camera_pos = self.camera.get_cam_pos().to_4d(1.0);
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

    pub fn set_screen_size(&mut self, rect: Rect<i32>) {
        self.screen.set_size(rect);
    }

    pub fn add_entity(&mut self, name: Cow<'static, str>, entity: Entity) {
        self.entities.insert(name, entity);
    }

    pub fn meshes_and_materials<'a>(
        &'a mut self,
        render: &Render,
    ) -> impl Iterator<Item = (&'a mut Arc<Mesh>, &'a mut [Material])> {
        let vec: Vec<_> = self
            .entities
            .values_mut()
            .map(|entity| entity.get_mesh_and_materials(render))
            .collect();
        vec.into_iter()
    }

    pub fn set_environment_data(&mut self, render: &Render, data: &mut Environment) {
        for entity in self.entities.values_mut() {
            for material in &mut entity.materials {
                material.set_data(render, 0, data).unwrap();
            }
        }
    }

    pub fn add_sky_entity(&mut self, sky_entity: Entity) {
        self.entities.insert("skybox".into(), sky_entity);
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
                // if let Some(spaceship) = self.entities.get_mut("ship") {
                //     spaceship.position.set_forward_velocity(SPEED);
                // }
                self.spaceship.forward = self.spaceship.speed;
            }
            b'S' => {
                // if let Some(spaceship) = self.entities.get_mut("ship") {
                //     spaceship.position.set_forward_velocity(-SPEED);
                // }
                self.spaceship.forward = -self.spaceship.speed;
            }
            b'A' => {
                // self.camera.rightward = -SPEED;
            }
            b'D' => {
                // self.camera.rightward = SPEED;
            }
            b'O' => {
                self.light_rad -= 5.0 * self.delta_t.get();
            }
            b'P' => {
                self.light_rad += 5.0 * self.delta_t.get();
            }
            input::key::SHIFT => {
                self.spaceship.speed = SpaceShip::DEFAULT_SPEED * 5.0;
            }
            _ => {}
        }
    }
    fn on_key_up(&mut self, key: usize) {
        self.spaceship.reset_velocity();
        if let Some(spaceship) = self.entities.get_mut("ship") {
            spaceship.position.set_forward_velocity(0.0);
        }

        let key = key as u8;
        match key {
            input::key::ESCAPE => {
                if self.play_state.is_playing() {
                    input::show_cursor(true);
                    self.play_state.set_not_playing();
                }
            }
            input::key::SHIFT => {
                self.spaceship.speed = SpaceShip::DEFAULT_SPEED;
            }
            _ => {}
        }
    }
    fn on_mouse_move(&mut self, pos: Point) {
        if self.play_state.is_playing() {
            self.delta_mouse_x = (pos.x - self.screen.rect.center_x()) as f32;
            self.delta_mouse_y = (pos.y - self.screen.rect.center_y()) as f32;

            self.screen.center_cursor();
        }
    }
    fn on_left_mouse_down(&mut self) {
        if self.play_state.is_not_playing() {
            input::show_cursor(false);
            self.play_state.set_playing();
            self.screen.center_cursor();
        }
    }
}
