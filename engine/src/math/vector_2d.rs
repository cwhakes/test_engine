use std::{convert, ops};

use wavefront_obj::obj;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn new(x: f32, y: f32) -> Vector2d {
        Vector2d { x, y }
    }

    pub fn lerp(&self, other: impl Into<Vector2d>, delta: f32) -> Vector2d  {
        let other = other.into();
        Vector2d {
            x: self.x * (1.0 - delta) + other.x * delta,
            y: self.y * (1.0 - delta) + other.y * delta,
        }
    }
}

impl convert::From<[f32; 2]> for Vector2d {
    fn from(array: [f32; 2]) -> Self {
        Vector2d {
            x: array[0],
            y: array[1],
        }
    }
}

impl convert::From<obj::TVertex> for Vector2d {
    fn from(vertex: obj::TVertex) -> Self {
        Vector2d {
            x: vertex.u as f32,
            y: vertex.v as f32,
        }
    }
}

impl ops::Add<Vector2d> for Vector2d {
    type Output = Vector2d;

    fn add(self, rhs: Vector2d) -> Self::Output{
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vector2d {
    type Output = Vector2d;

    fn mul(self, rhs: f32) -> Self::Output{
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
