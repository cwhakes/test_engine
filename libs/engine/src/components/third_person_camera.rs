use crate::math::{Matrix4x4, Vector3d};
use crate::physics::position::Position;

#[derive(Debug)]
pub struct ThirdPersonCamera {
    position: Position,
    pub offset: f32,
    fov: f32,
    front_plate: f32,
    back_plate: f32,
}

impl ThirdPersonCamera {
    pub fn update(&mut self, delta_t: f32) {
        self.position.update(delta_t);
    }

    pub fn get_location(&self) -> Vector3d {
        self.position.get_location()
    }

    pub fn set_location(&mut self, loc: impl Into<Vector3d>) {
        self.position.set_location(loc);
    }

    pub fn get_skysphere(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::scaling(self.back_plate);
        let mut position = self.position.clone();
        position.move_forward(-self.offset);
        matrix.set_translation(position.get_location());
        matrix
    }

    pub fn get_view(&self) -> Matrix4x4 {
        let mut position = self.position.clone();
        position.move_forward(-self.offset);
        position.get_matrix().inverse().unwrap()
    }

    pub fn get_proj(&self, aspect_ratio: f32) -> Matrix4x4 {
        Matrix4x4::perspective(self.fov, aspect_ratio, self.front_plate, self.back_plate)
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position.move_forward(distance);
    }

    pub fn move_up(&mut self, distance: f32) {
        self.position.move_up(distance);
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

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            position: Position::default(),
            offset: 15.0,
            fov: std::f32::consts::PI / 4.0,
            front_plate: 0.01,
            back_plate: 100.0,
        }
    }
}
