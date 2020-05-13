use crate::math::{Matrix4x4, Vector4d};
use crate::physics::collision::{Collision, Sphere};
use crate::physics::position::Position;

#[derive(Default, Debug)]
pub struct Camera {
    position: Position,
}

impl Camera {
    const COLLISION_RADIUS: f32 = 0.1;

    pub fn update(&mut self, delta_t: f32) {
        self.position.update(delta_t);
    }

    pub fn get_location(&self) -> Vector4d {
        self.position.get_location().to_4d(1.0)
    }

    pub fn get_view(&self) -> Matrix4x4 {
        self.position.get_matrix().inverse().unwrap()
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position.move_forward(distance);
    }

    pub fn pan(&mut self, delta_angle: f32) -> &mut Self {
        self.position.pan(delta_angle);
        self
    }

    pub fn tilt(&mut self, delta_angle: f32) -> &mut Self {
        self.position.tilt(delta_angle);
        self
    }

    pub fn moving_forward(&mut self, velocity: f32) -> &mut Self {
        self.position.set_forward_velocity(velocity);
        self
    }

    pub fn moving_rightward(&mut self, velocity: f32) -> &mut Self {
        self.position.set_rightward_velocity(velocity);
        self
    }

    pub fn reset_velocity(&mut self) -> &mut Self {
        self.position.set_velocity([0.0, 0.0, 0.0]);
        self
    }
}

impl Collision for Camera {
    type Collider = Sphere;

    fn collider(&self) -> Sphere {
        Sphere::new(
            self.position.get_location(),
            Self::COLLISION_RADIUS,
        )
    }
}
