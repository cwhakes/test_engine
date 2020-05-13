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
    pub fn new() -> Position {
        dbg!(Position::default())
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

        let translation = self.position.get_translation();
        self.position.set_translation([0.0, 0.0, 0.0]);
        self.position *= Matrix4x4::rotation_vec(delta_angle);
        &self.position;
        self.position.set_translation(translation);

        self.position.translate(delta_x);
        self
    }

    pub fn get_matrix(&self) -> Matrix4x4 {
        self.position.clone()
    }

    pub fn get_location(&self) -> Vector3d {
        self.position.get_translation()
    }

    pub fn move_forward(&mut self, distance: f32) -> &mut Self {

        let new_pos = self.position.get_translation()
            + self.position.get_direction_z() * distance;

        self.position.set_translation(new_pos);
        self
    }

    pub fn pan(&mut self, angle: f32) -> &mut Self {
        let upward_direction: Vector3d = [0.0, 1.0, 0.0].into();
        let delta_angle = upward_direction * angle;
        
        let translation = self.position.get_translation();
        self.position.set_translation([0.0, 0.0, 0.0]);
        self.position *= Matrix4x4::rotation_vec(delta_angle);
        self.position.set_translation(translation);

        self
    }

    pub fn tilt(&mut self, angle: f32) -> &mut Self {
        let rigtward_direction = self.position.get_direction_x().normalize();
        let delta_angle = rigtward_direction * angle;
        
        let translation = self.position.get_translation();
        self.position.set_translation([0.0, 0.0, 0.0]);
        self.position *= Matrix4x4::rotation_vec(delta_angle);
        self.position.set_translation(translation);

        self
    }

    pub fn set_velocity(&mut self, new_velocity: impl Into<Vector3d>) -> &mut Self {
        self.velocity = new_velocity.into();
        self
    }

    pub fn set_forward_velocity(&mut self, new_velocity: f32) -> &mut Self {
        let forward_direction = self.position.get_direction_z().normalize();
        let forward_velocity = forward_direction.dot(self.velocity.clone());
        self.velocity -= forward_direction.clone() * forward_velocity;
        self.velocity += forward_direction * new_velocity;
        self
    }

    pub fn set_rightward_velocity(&mut self, new_velocity: f32) -> &mut Self  {
        let rigtward_direction = self.position.get_direction_x().normalize();
        let rightward_velocity = rigtward_direction.dot(self.velocity.clone());
        self.velocity -= rigtward_direction.clone() * rightward_velocity;
        self.velocity += rigtward_direction * new_velocity;
        self
    }

    pub fn set_pan_velocity(&mut self, new_angular: f32) -> &mut Self {
        let upward_direction: Vector3d = [0.0, 1.0, 0.0].into();
        let upward_angular = upward_direction.dot(self.angular_velocity.clone());
        self.angular_velocity -= upward_direction.clone() * upward_angular;
        self.angular_velocity += upward_direction * new_angular;
        self
    }

    pub fn set_tilt_velocity(&mut self, new_angular: f32) -> &mut Self  {
        let rigtward_direction = self.position.get_direction_x().normalize();
        let rightward_angular = rigtward_direction.dot(self.angular_velocity.clone());
        self.velocity -= rigtward_direction.clone() * rightward_angular;
        self.velocity += rigtward_direction * new_angular;
        self
    }
}
