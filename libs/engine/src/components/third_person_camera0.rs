use crate::math::{Matrix4x4, Rect, Vector3d};

const FRONT_PLATE: f32 = 0.1;
const BACK_PLATE: f32 = 5000.0;
const FOV: f32 = std::f32::consts::PI / 4.0;

#[derive(Default)]
pub struct Camera {
    pub world_cam: Matrix4x4,
    pub view_cam: Matrix4x4,

    pub current_cam_rot: Vector3d,
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

        self.current_cam_rot = self.current_cam_rot.lerp(self.cam_rot, 3.0*delta_t);

        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(self.current_cam_rot.x());
        world_cam *= Matrix4x4::rotation_y(self.current_cam_rot.y());

        let mut new_pos = self.cam_pos;
        new_pos += world_cam.get_direction_z() * (-self.cam_distance);
        new_pos += world_cam.get_direction_y() * (5.0);

        world_cam.set_translation(new_pos);
        self.world_cam = world_cam.clone();
        self.view_cam = world_cam.inverse().unwrap();
    }

    pub fn proj_cam(&self, rect: Rect<f32>) -> Matrix4x4 {
        Matrix4x4::perspective(FOV, rect.aspect(), FRONT_PLATE, BACK_PLATE)
    }

    pub fn get_skysphere(&self) -> Matrix4x4 {
        // Scale down by one because sphere isn't perfect
        let mut matrix = Matrix4x4::scaling(BACK_PLATE - 100f32);
        matrix.set_translation(self.world_cam.get_translation());
        matrix
    }

    pub fn reset_velocity(&mut self) {
        self.forward = 0.0;
        self.rightward = 0.0;
    }
}