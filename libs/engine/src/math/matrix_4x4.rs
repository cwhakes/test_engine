use super::{Matrix, Vector, Vector3d, Vector4d};

use std::{convert, ops};

pub type Matrix4x4 = Matrix<f32, 4, 4>;

impl Matrix4x4 {
    pub fn translation(vec: impl Into<Vector3d>) -> Self {
        let Vector([x, y, z]) = vec.into();
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x, y, z, 1.0],
        ])
    }

    pub fn set_translation(&mut self, vec: impl Into<Vector3d>) {
        let vec = vec.into();
        self.0[3][0] = vec.x();
        self.0[3][1] = vec.y();
        self.0[3][2] = vec.z();
    }

    pub fn translate(&mut self, vec: impl Into<Vector3d>) {
        *self *= Self::translation(vec);
    }

    pub fn scaling(scale: f32) -> Self {
        Self([
            [scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, scale, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scaling3(vec: impl Into<Vector3d>) -> Self {
        let Vector([x, y, z]) = vec.into();
        //let Vector3d { x, y, z } = vec.into();
        Self([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn orthoganal(width: f32, height: f32, near_plane: f32, far_plane: f32) -> Self {
        let mut matrix = Self::zero();

        matrix.0[0][0] = 2.0 / width;
        matrix.0[1][1] = 2.0 / height;
        matrix.0[2][2] = 1.0 / (far_plane - near_plane);
        matrix.0[3][2] = -(near_plane / (far_plane - near_plane));
        matrix
    }

    pub fn perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let yscale = 1.0 / (fov / 2.0).tan();
        let xscale = yscale / aspect;

        let mut matrix = Self::zero();
        matrix.0[0][0] = xscale;
        matrix.0[1][1] = yscale;
        matrix.0[2][2] = zfar / (zfar - znear);
        matrix.0[2][3] = 1.0;
        matrix.0[3][2] = (-znear * zfar) / (zfar - znear);
        matrix
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut matrix = Self::identity();
        matrix.0[1][1] = angle.cos();
        matrix.0[1][2] = angle.sin();
        matrix.0[2][1] = -angle.sin();
        matrix.0[2][2] = angle.cos();
        matrix
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut matrix = Self::identity();
        matrix.0[0][0] = angle.cos();
        matrix.0[0][2] = -angle.sin();
        matrix.0[2][0] = angle.sin();
        matrix.0[2][2] = angle.cos();
        matrix
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut matrix = Self::identity();
        matrix.0[0][0] = angle.cos();
        matrix.0[0][1] = angle.sin();
        matrix.0[1][0] = -angle.sin();
        matrix.0[1][1] = angle.cos();
        matrix
    }

    /// <https://en.wikipedia.org/wiki/Rotation_matrix#Rotation_matrix_from_axis_and_angle/>
    pub fn rotation_vec(angle: impl Into<Vector3d>) -> Self {
        let angle = angle.into();
        let mag = angle.magnitude();

        let angle = if mag > 0.0 {
            angle.normalize().to_4d(0.0)
        } else {
            [0.0, 0.0, 0.0, 0.0].into()
        };

        let cross = angle.cross_matrix();
        let outer = angle.outer(angle);

        Self::scaling(mag.cos()) + cross * (mag.sin()) + outer * (1.0 - mag.cos())
    }

    pub fn rotate_in_place(&mut self, rotation_matrix: Self) -> &mut Self {
        let translation = self.get_translation();
        self.set_translation([0.0, 0.0, 0.0]);
        *self *= rotation_matrix;
        self.set_translation(translation);
        self
    }

    // pub fn inverse(&self) -> Option<Self> {
    //     let mut out = Self::default();
    //     let mut vec = <[Vector4d; 3]>::default();

    //     let det = self.determinant();
    //     if det.is_nan() {
    //         return None;
    //     }
    //     for i in 0..4 {
    //         for j in 0..4 {
    //             if j != i {
    //                 let mut a = j;
    //                 if j > i {
    //                     a -= 1;
    //                 }
    //                 *vec[a].x_mut() = self.0[j][0];
    //                 *vec[a].y_mut() = self.0[j][1];
    //                 *vec[a].z_mut() = self.0[j][2];
    //                 *vec[a].w_mut() = self.0[j][3];
    //             }
    //         }
    //         let v = Vector4d::cross(&vec[0], &vec[1], &vec[2]);

    //         out.0[0][i] = (-1.0_f32).powi(i as i32) * v.x() / det;
    //         out.0[1][i] = (-1.0_f32).powi(i as i32) * v.y() / det;
    //         out.0[2][i] = (-1.0_f32).powi(i as i32) * v.z() / det;
    //         out.0[3][i] = (-1.0_f32).powi(i as i32) * v.w() / det;
    //     }

    //     Some(out)
    // }

    // pub fn determinant(&self) -> f32 {
    //     let v1: Vector4d = self.column(0);
    //     let v2: Vector4d = self.column(1);
    //     let v3: Vector4d = self.column(2);

    //     let minor = Vector4d::cross(&v1, &v2, &v3);
    //     -(self.0[0][3] * minor.x()
    //         + self.0[1][3] * minor.y()
    //         + self.0[2][3] * minor.z()
    //         + self.0[3][3] * minor.w())
    // }

    pub fn column(&self, i: usize) -> Vector4d {
        assert!((0..4).contains(&i));
        Vector([self.0[0][i], self.0[1][i], self.0[2][i], self.0[3][i]])
    }

    pub fn get_direction_x(&self) -> Vector3d {
        Vector3d::new(self.0[0][0], self.0[0][1], self.0[0][2])
    }

    pub fn get_direction_y(&self) -> Vector3d {
        Vector3d::new(self.0[1][0], self.0[1][1], self.0[1][2])
    }

    pub fn get_direction_z(&self) -> Vector3d {
        Vector3d::new(self.0[2][0], self.0[2][1], self.0[2][2])
    }

    pub fn get_translation(&self) -> Vector3d {
        Vector3d::new(self.0[3][0], self.0[3][1], self.0[3][2])
    }
}

impl convert::From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(array: [[f32; 4]; 4]) -> Self {
        Self(array)
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl ops::Add for Matrix4x4 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign for Matrix4x4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] += rhs.0[i][j];
            }
        }
    }
}

impl ops::Mul<f32> for Matrix4x4 {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<f32> for Matrix4x4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.0[i][j] *= rhs;
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn invert_identity() {
        use super::*;

        assert_eq!(
            Matrix4x4::identity(),
            Matrix4x4::identity().inverse().unwrap(),
        );
    }
}
