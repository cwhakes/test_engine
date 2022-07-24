use crate::math::{Matrix4x4, Rect, Vector3d};

const FRONT_PLATE: f32 = 0.1;
const BACK_PLATE: f32 = 5000.0;
const FOV: f32 = std::f32::consts::PI / 4.0;

#[derive(Default)]
pub struct Camera {
    focus_pos: Vector3d,
    focus_rot: Vector3d,

    cam_pos: Vector3d,
    cam_rot: Vector3d,

    cam_distance: f32,
    cam_offset: f32,

    forward: f32,
    rightward: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            focus_rot: [1.0, 0.0, 0.0].into(),
            cam_distance: 14.0,
            cam_offset: 5.0,
            ..Self::default()
        }
    }

    pub fn set_focus(&mut self, pos: Vector3d, rot: Vector3d) {
        self.focus_pos = pos;
        self.focus_rot = rot;
    }

    pub fn get_cam_pos(&self) -> Vector3d {
        self.world_cam().get_translation()
    }

    pub fn update(&mut self, delta_t: f32) {
        self.cam_pos = self.cam_pos.lerp(self.focus_pos, 3.0 * delta_t);
        self.cam_rot = self.cam_rot.lerp(self.focus_rot, 3.0 * delta_t);
    }

    pub fn world_cam(&self) -> Matrix4x4 {
        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(self.cam_rot.x());
        world_cam *= Matrix4x4::rotation_y(self.cam_rot.y());
        let mut pos = self.cam_pos;
        pos += world_cam.get_direction_z() * (-self.cam_distance);
        pos += world_cam.get_direction_y() * (self.cam_offset);
        world_cam.set_translation(pos);
        world_cam
    }

    pub fn view_cam(&self) -> Matrix4x4 {
        self.world_cam().inverse().unwrap()
    }

    pub fn proj_cam(&self, rect: Rect<f32>) -> Matrix4x4 {
        Matrix4x4::perspective(FOV, rect.aspect(), FRONT_PLATE, BACK_PLATE)
    }

    pub fn get_skysphere(&self) -> Matrix4x4 {
        // Scale down by one because sphere isn't perfect
        let mut matrix = Matrix4x4::scaling(BACK_PLATE - 100f32);
        matrix.set_translation(self.world_cam().get_translation());
        matrix
    }

    pub fn reset_velocity(&mut self) {
        self.forward = 0.0;
        self.rightward = 0.0;
    }
}
