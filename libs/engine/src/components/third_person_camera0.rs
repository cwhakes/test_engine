use crate::math::{Matrix4x4, Rect, Vector3d};

#[derive(Default)]
pub struct Camera {
    pub world_cam: Matrix4x4,
    pub view_cam: Matrix4x4,

    pub cam_rot: Vector3d,
    pub cam_pos: Vector3d,
    pub target_pos: Vector3d,

    pub cam_distance: f32,

    pub rot_x: f32,
    pub rot_y: f32,

    pub forward: f32,
    pub rightward: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            cam_distance: 14.0,
            ..Self::default()
        }
    }

    pub fn update(&mut self, delta_t: f32, delta_mouse_x: f32, delta_mouse_y: f32) {
        *self.cam_rot.x_mut() += delta_mouse_y * delta_t * 0.1;
        *self.cam_rot.x_mut() = self.cam_rot.x().clamp(-1.57, 1.57);
        *self.cam_rot.y_mut() += delta_mouse_x * delta_t * 0.1;

        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(self.cam_rot.x());
        world_cam *= Matrix4x4::rotation_y(self.cam_rot.y());

        let mut new_pos = self.cam_pos;
        new_pos += world_cam.get_direction_z() * (-self.cam_distance);
        new_pos += world_cam.get_direction_y() * (5.0);

        world_cam.set_translation(new_pos);
        self.world_cam = world_cam.clone();
        self.view_cam = world_cam.inverse().unwrap();
    }

    pub fn proj_cam(&self, rect: Rect<f32>) -> Matrix4x4 {
        Matrix4x4::perspective(std::f32::consts::PI / 2.0, rect.aspect(), 0.1, 100.0)
    }

    pub fn get_skysphere(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::scaling(100.0);
        matrix.set_translation(self.world_cam.get_translation());
        matrix
    }

    pub fn reset_velocity(&mut self) {
        self.forward = 0.0;
        self.rightward = 0.0;
    }
}
