use std::{convert, ops};

use winapi::shared::windef;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl convert::From<windef::POINT> for Point {
    fn from(point: windef::POINT) -> Self {
        Point {
            x: point.x,
            y: point.y,
        }
    }
}

impl convert::From<(i32, i32)> for Point {
    fn from(point: (i32, i32)) -> Self {
        Point {
            x: point.0,
            y: point.1,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
