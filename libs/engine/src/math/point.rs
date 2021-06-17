use std::{convert, ops};

use winapi::shared::windef;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl convert::From<windef::POINT> for Point {
    fn from(point: windef::POINT) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl convert::From<(i32, i32)> for Point {
    fn from(point: (i32, i32)) -> Self {
        Self {
            x: point.0,
            y: point.1,
        }
    }
}

impl ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
