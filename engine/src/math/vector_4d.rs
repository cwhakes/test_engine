use super::Vector3d;

use std::convert;

use wavefront_obj::obj;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vector4d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4d {
    pub fn to_3d_unchecked(&self) -> Vector3d {
        Vector3d {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

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

impl convert::From<obj::Vertex> for Vector4d {
    fn from(vertex: obj::Vertex) -> Self {
        Vector4d {
            x: vertex.x as f32,
            y: vertex.y as f32,
            z: vertex.z as f32,
            w: 1.0,
        }
    }
}
