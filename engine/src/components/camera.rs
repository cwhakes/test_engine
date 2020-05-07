use crate::math::{Matrix4x4, Vector4d};

#[derive(Default)]
pub struct Camera {
    matrix: Matrix4x4,
}

impl Camera {
    pub fn get_position(&self) -> Vector4d {
        self.matrix.get_translation().to_4d(1.0)
    }

    pub fn get_view(&self) -> Matrix4x4 {
        self.matrix.inverse().unwrap()
    }

    pub fn set_rotation(&mut self, rot_x: f32, rot_y: f32) -> &mut Self {

        let mut world_cam = Matrix4x4::identity();
        world_cam *= Matrix4x4::rotation_x(rot_x);
        world_cam *= Matrix4x4::rotation_y(rot_y);
        world_cam.set_translation(self.matrix.get_translation());
        self.matrix = world_cam;

        self
    }

    pub fn move_forward(&mut self, distance: f32) -> &mut Self {

        let new_pos = self.matrix.get_translation()
            + self.matrix.get_direction_z() * distance;

        self.matrix.set_translation(new_pos);
        self
    }

    pub fn move_rightward(&mut self, distance: f32) -> &mut Self {

        let new_pos = self.matrix.get_translation()
            + self.matrix.get_direction_x() * distance;

        self.matrix.set_translation(new_pos);
        self
    }
}