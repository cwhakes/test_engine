use super::Vector4d;

use std::{convert, ops};

use wavefront_obj::obj;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }

    pub fn magnitude(&self) -> f32 {
        let mag2 = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        mag2.sqrt()
    }

    pub fn normalize(self) -> Vector3d {
        let mag = self.magnitude();
        self / mag
    }

    pub fn to_4d(self, w: f32) -> Vector4d {
        Vector4d {
            x: self.x,
            y: self.y,
            z: self.z,
            w: w,
        }
    }

    pub fn lerp(&self, other: impl Into<Vector3d>, delta: f32) -> Vector3d  {
        let other = other.into();
        Vector3d {
            x: self.x * (1.0 - delta) + other.x * delta,
            y: self.y * (1.0 - delta) + other.y * delta,
            z: self.z * (1.0 - delta) + other.z * delta,
        }
    }

    pub fn dot(&self, rhs: impl Into<Vector3d>) -> Vector3d {
        let rhs = rhs.into();
        Vector3d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    pub fn cross(&self, rhs: impl Into<Vector3d>) -> Vector3d {
        let rhs = rhs.into();
        Vector3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
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

impl convert::From<obj::Vertex> for Vector3d {
    fn from(vertex: obj::Vertex) -> Self {
        Vector3d {
            x: vertex.x as f32,
            y: vertex.y as f32,
            z: vertex.z as f32,
        }
    }
}

impl<T: Into<Vector3d>> ops::Add<T> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vector3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Into<Vector3d>> ops::Sub<T> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Vector3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
