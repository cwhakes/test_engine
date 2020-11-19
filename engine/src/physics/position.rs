use crate::math::{Matrix4x4, Vector3d};

#[derive(Clone, Debug, Default)]
pub struct Position {
    position: Matrix4x4,
    velocity: Vector3d,
    angular_velocity: Vector3d,
    accelleration: Vector3d,
    angular_accelleration: Vector3d,
}

impl Position {
    pub fn new(position: Matrix4x4) -> Self {
        Position {
            position,
            ..Position::default()
        }
    }

    pub fn with_gravity(mut self, gravity: f32) -> Position {
        self.accelleration.y = -gravity;
        self
    }

    pub fn update(&mut self, delta_t: f32) -> &mut Self {
        let delta_x =
            (self.velocity.clone() * delta_t) +
            (self.accelleration.clone() * delta_t.powi(2) / 2.0);
        let delta_angle =
            (self.angular_velocity.clone() * delta_t) +
            (self.angular_accelleration.clone() * delta_t.powi(2) / 2.0);

        self.velocity += self.accelleration.clone() * delta_t;
        self.angular_velocity += self.angular_accelleration.clone() * delta_t;

        self.position.rotate_in_place(Matrix4x4::rotation_vec(delta_angle));
        self.position.translate(delta_x);

        self
    }

    pub fn get_matrix(&self) -> Matrix4x4 {
        self.position.clone()
    }

    pub fn set_matrix(&mut self, matrix: Matrix4x4) {
        self.position = matrix
    }

    pub fn get_location(&self) -> Vector3d {
        self.position.get_translation()
    }

    pub fn right(&self) -> Vector3d {
        self.position.get_direction_x().normalize()
    }

    pub fn up(&self) -> Vector3d {
        self.position.get_direction_y().normalize()
    }

    pub fn forward(&self) -> Vector3d {
        self.position.get_direction_z().normalize()
    }

    pub fn move_forward(&mut self, distance: f32) -> &mut Self {

        let new_pos = self.position.get_translation()
            + self.position.get_direction_z() * distance;

        self.position.set_translation(new_pos);
        self
    }

    pub fn move_up(&mut self, distance: f32) -> &mut Self {

        let new_pos = self.position.get_translation()
            + self.position.get_direction_y() * distance;

        self.position.set_translation(new_pos);
        self
    }

    pub fn pan(&mut self, angle: f32) -> &mut Self {
        let upward_direction: Vector3d = [0.0, 1.0, 0.0].into();
        let delta_angle = upward_direction * angle;
        
        self.position.rotate_in_place(Matrix4x4::rotation_vec(delta_angle));

        self
    }

    pub fn tilt(&mut self, angle: f32) -> &mut Self {
        let delta_angle = self.right() * angle;
        
        self.position.rotate_in_place(Matrix4x4::rotation_vec(delta_angle));

        self
    }

    pub fn set_velocity(&mut self, new_velocity: impl Into<Vector3d>) -> &mut Self {
        self.velocity = new_velocity.into();
        self
    }

    pub fn set_forward_velocity(&mut self, new_velocity: f32) -> &mut Self {
        self.velocity.set_component(self.forward() * new_velocity);
        self
    }

    pub fn set_rightward_velocity(&mut self, new_velocity: f32) -> &mut Self  {
        self.velocity.set_component(self.right() * new_velocity);
        self
    }

    pub fn set_pan_velocity(&mut self, new_angular: f32) -> &mut Self {
        let upward_direction: Vector3d = [0.0, 1.0, 0.0].into();
        self.velocity.set_component(upward_direction * new_angular);
        self
    }

    pub fn set_tilt_velocity(&mut self, new_angular: f32) -> &mut Self  {
        self.velocity.set_component(self.right() * new_angular);
        self
    }
}
