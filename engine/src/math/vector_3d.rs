use std::convert;

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
