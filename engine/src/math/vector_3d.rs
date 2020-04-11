use super::Vector4d;

use std::{convert, ops};

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }

    pub fn lerp(&self, other: impl Into<Vector3d>, delta: f32) -> Vector3d  {
        let other = other.into();
        Vector3d {
            x: self.x * (1.0 - delta) + other.x * delta,
            y: self.y * (1.0 - delta) + other.y * delta,
            z: self.z * (1.0 - delta) + other.z * delta,
        }
    }
}

impl convert::From<[f32; 3]> for Vector3d {
    fn from(array: [f32; 3]) -> Self {
        Vector3d {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl convert::From<Vector4d> for Vector3d {
    fn from(vector_4d: Vector4d) -> Self {
        let w = vector_4d.w;
        Vector3d {
            x: vector_4d.x / w,
            y: vector_4d.x / w,
            z: vector_4d.x / w,
        }
    }
}

impl ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Self::Output{
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Self::Output{
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
