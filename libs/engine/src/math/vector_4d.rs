use super::{Matrix4x4, Vector, Vector3d};

use std::convert;

use wavefront_obj::obj;

pub type Vector4d = Vector<f32, 4>;

impl Vector4d {
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.0[2]
    }

    pub fn w(&self) -> f32 {
        self.0[3]
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.0[3]
    }

    pub fn to_3d_unchecked(self) -> Vector3d {
        let Vector([x, y, z, ..]) = self;
        Vector3d::new(x, y, z)
    }

    pub fn cross(v1: &Self, v2: &Self, v3: &Self) -> Self {
        Self([
            v1.y() * (v2.z() * v3.w() - v3.z() * v2.w())
                - v1.z() * (v2.y() * v3.w() - v3.y() * v2.w())
                + v1.w() * (v2.y() * v3.z() - v2.z() * v3.y()),
            -(v1.x() * (v2.z() * v3.w() - v3.z() * v2.w())
                - v1.z() * (v2.x() * v3.w() - v3.x() * v2.w())
                + v1.w() * (v2.x() * v3.z() - v3.x() * v2.z())),
            v1.x() * (v2.y() * v3.w() - v3.y() * v2.w())
                - v1.y() * (v2.x() * v3.w() - v3.x() * v2.w())
                + v1.w() * (v2.x() * v3.y() - v3.x() * v2.y()),
            -(v1.x() * (v2.y() * v3.z() - v3.y() * v2.z())
                - v1.y() * (v2.x() * v3.z() - v3.x() * v2.z())
                + v1.z() * (v2.x() * v3.y() - v3.x() * v2.y())),
        ])
    }

    /// https://en.wikipedia.org/wiki/Cross_product#Conversion_to_matrix_multiplication
    pub fn cross_matrix(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::zero();

        matrix.0[0][1] = self.z();
        matrix.0[0][2] = -self.y();
        matrix.0[1][2] = self.x();

        matrix.0[1][0] = -self.z();
        matrix.0[2][0] = self.y();
        matrix.0[2][1] = -self.x();

        matrix.0[3][3] = self.w();

        matrix
    }

    pub fn outer(self, other: impl Into<Self>) -> Matrix4x4 {
        let other = other.into();

        let mut matrix = Matrix4x4::default();
        for (i, u) in std::array::IntoIter::new(self.0).enumerate() {
            for (j, v) in std::array::IntoIter::new(other.0).enumerate() {
                matrix.0[i][j] = u * v;
            }
        }
        matrix
    }
}

impl convert::From<obj::Vertex> for Vector4d {
    fn from(vertex: obj::Vertex) -> Self {
        Self([vertex.x as f32, vertex.y as f32, vertex.z as f32, 1.0])
    }
}
