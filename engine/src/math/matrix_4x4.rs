use super::Vector3d;

use std::{convert, ops};

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Matrix4x4(pub [[f32; 4]; 4]);

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        let mut matrix = Matrix4x4::default();
        matrix.0[0][0] = 1.0;
        matrix.0[1][1] = 1.0;
        matrix.0[2][2] = 1.0;
        matrix.0[3][3] = 1.0;
        matrix
    }

    pub fn translation(vec: impl Into<Vector3d>) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        let vec = vec.into();
        matrix.0[3][0] = vec.x;
        matrix.0[3][1] = vec.y;
        matrix.0[3][2] = vec.z;
        matrix
    }

    pub fn scaling(vec: impl Into<Vector3d>) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        let vec = vec.into();
        matrix.0[0][0] = vec.x;
        matrix.0[1][1] = vec.y;
        matrix.0[2][2] = vec.z;
        matrix
    }

    pub fn orthoganal(width: f32, height: f32, near_plane: f32, far_plane: f32) -> Matrix4x4 {
        
        let mut matrix = Matrix4x4::identity();
        matrix.0[0][0] = 2.0 / width;
        matrix.0[1][1] = 2.0 / height;
        matrix.0[2][2] = 1.0 / ( far_plane - near_plane );
        matrix.0[3][2] = -(near_plane / (far_plane - near_plane));
        matrix
    }

    pub fn rotation_x(angle: f32) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        matrix.0[1][1] = angle.cos();
        matrix.0[1][2] = angle.sin();
        matrix.0[2][1] = -angle.sin();
        matrix.0[2][2] = angle.cos();
        matrix
    }

    pub fn rotation_y(angle: f32) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        matrix.0[0][0] = angle.cos();
        matrix.0[0][2] = -angle.sin();
        matrix.0[2][0] = angle.sin();
        matrix.0[2][2] = angle.cos();
        matrix
    }

    pub fn rotation_z(angle: f32) -> Matrix4x4 {
        let mut matrix = Matrix4x4::identity();
        matrix.0[0][0] = angle.cos();
        matrix.0[0][1] = angle.sin();
        matrix.0[1][0] = -angle.sin();
        matrix.0[1][1] = angle.cos();
        matrix
    }
}

impl convert::From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(array: [[f32; 4]; 4]) -> Self {
        Matrix4x4(array)
    }
}

impl ops::MulAssign<Matrix4x4> for Matrix4x4 {
    fn mul_assign(&mut self, rhs:Matrix4x4) {
        let mut new = Matrix4x4::default();
        for i in 0..4 {
            for j in 0..4 {
                new.0[i][j] = (0..4).map(|k| {
                    self.0[i][k] * rhs.0[k][j]
                }).sum();
            }
        }
        *self = new;
    }
}
