use std::convert;

use wavefront_obj::obj;

use super::Vector;

pub type Vector2d = Vector<f32, 2>;

impl Vector2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }
}

impl convert::From<obj::TVertex> for Vector2d {
    fn from(vertex: obj::TVertex) -> Self {
        Self::new(vertex.u as f32, vertex.v as f32)
    }
}
