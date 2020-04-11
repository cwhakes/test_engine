use super::Vector3d;

use std::convert;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vector4d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4d {
    pub fn cross(v1: &Vector4d, v2: &Vector4d, v3: &Vector4d) -> Vector4d {
        Vector4d {
            x: v1.y * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.y * v3.w - v3.y * v2.w) + v1.w * (v2.y * v3.z - v2.z *v3.y),
            y: -(v1.x * (v2.z * v3.w - v3.z * v2.w) - v1.z * (v2.x * v3.w - v3.x * v2.w) + v1.w * (v2.x * v3.z - v3.x * v2.z)),
            z: v1.x * (v2.y * v3.w - v3.y * v2.w) - v1.y * (v2.x *v3.w - v3.x * v2.w) + v1.w * (v2.x * v3.y - v3.x * v2.y),
            w: -(v1.x * (v2.y * v3.z - v3.y * v2.z) - v1.y * (v2.x * v3.z - v3.x *v2.z) + v1.z * (v2.x * v3.y - v3.x * v2.y)),
        }
    }
}

impl convert::From<[f32; 4]> for Vector4d {
    fn from(array: [f32; 4]) -> Self {
        Vector4d {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

impl convert::From<Vector3d> for Vector4d {
    fn from(vector_3d: Vector3d) -> Self {
        Vector4d {
            x: vector_3d.x,
            y: vector_3d.x,
            z: vector_3d.x,
            w: 1.0,
        }
    }
}
