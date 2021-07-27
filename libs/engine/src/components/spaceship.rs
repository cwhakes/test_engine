use crate::math::{Matrix4x4, Vector3d};

#[derive(Default)]
pub struct SpaceShip {
    pub spaceship_rot: Vector3d,
    pub current_spaceship_rot: Vector3d,
    pub spaceship_pos: Vector3d,
    pub current_spaceship_pos: Vector3d,
    pub rot_x: f32,
    pub rot_y: f32,

    pub forward: f32,
    pub rightward: f32,
}

impl SpaceShip {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, delta_t: f32, delta_mouse_x: f32, delta_mouse_y: f32) {
        *self.spaceship_rot.x_mut() += delta_mouse_y * delta_t * 0.1;
        *self.spaceship_rot.x_mut() = self.spaceship_rot.x().clamp(-1.57, 1.57);
        *self.spaceship_rot.y_mut() += delta_mouse_x * delta_t * 0.1;

        self.current_spaceship_rot = self.current_spaceship_rot.lerp(self.spaceship_rot, 5.0*delta_t);

        let mut world_model = Matrix4x4::identity();
        world_model *= Matrix4x4::rotation_x(self.current_spaceship_rot.x());
        world_model *= Matrix4x4::rotation_y(self.current_spaceship_rot.y());

        self.spaceship_pos =
            self.spaceship_pos + world_model.get_direction_z() * (self.forward) * delta_t;

        self.current_spaceship_pos = self.current_spaceship_pos.lerp(self.spaceship_pos, 3.0 * delta_t);
    }

    pub fn reset_velocity(&mut self) {
        self.forward = 0.0;
        self.rightward = 0.0;
    }
}
