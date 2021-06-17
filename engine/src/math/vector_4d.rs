use super::{Matrix4x4, Vector3d};

use std::convert;

use wavefront_obj::obj;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector4d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4d {
    pub fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn to_3d_unchecked(self) -> Vector3d {
        Vector3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn cross(v1: &Self, v2: &Self, v3: &Self) -> Self {
        Vector4d {
            x: v1.y * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.y * v3.w - v3.y * v2.w)
                + v1.w * (v2.y * v3.z - v2.z * v3.y),
            y: -(v1.x * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.x * v3.w - v3.x * v2.w)
                + v1.w * (v2.x * v3.z - v3.x * v2.z)),
            z: v1.x * (v2.y * v3.w - v3.y * v2.w) - v1.y * (v2.x * v3.w - v3.x * v2.w)
                + v1.w * (v2.x * v3.y - v3.x * v2.y),
            w: -(v1.x * (v2.y * v3.z - v3.y * v2.z) - v1.y * (v2.x * v3.z - v3.x * v2.z)
                + v1.z * (v2.x * v3.y - v3.x * v2.y)),
        }
    }

    /// https://en.wikipedia.org/wiki/Cross_product#Conversion_to_matrix_multiplication
    pub fn cross_matrix(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::zero();

        matrix.0[0][1] = self.z;
        matrix.0[0][2] = -self.y;
        matrix.0[1][2] = self.x;

        matrix.0[1][0] = -self.z;
        matrix.0[2][0] = self.y;
        matrix.0[2][1] = -self.x;

        matrix.0[3][3] = self.w;

        matrix
    }

    pub fn outer(self, other: impl Into<Self>) -> Matrix4x4 {
        let other = other.into();

        let mut matrix = Matrix4x4::default();
        for (i, u) in self.to_array().iter().enumerate() {
            for (j, v) in other.to_array().iter().enumerate() {
                matrix.0[i][j] = u * v;
            }
        }
        matrix
    }
}

impl convert::From<[f32; 4]> for Vector4d {
    fn from(array: [f32; 4]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

impl convert::From<obj::Vertex> for Vector4d {
    fn from(vertex: obj::Vertex) -> Self {
        Self {
            x: vertex.x as f32,
            y: vertex.y as f32,
            z: vertex.z as f32,
            w: 1.0,
        }
    }
}
