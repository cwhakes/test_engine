use std::ops::{Range, Sub};

#[derive(Clone, Debug, Default)]
pub struct Rect<T>(pub [Range<T>; 2]);

impl<T: Copy> Rect<T> {
    pub fn left(&self) -> T {
        self.0[0].start
    }

    pub fn top(&self) -> T {
        self.0[1].start
    }
}

impl<T: Sub + Copy> Rect<T> {
    pub fn width(&self) -> <T as Sub>::Output {
        self.0[0].end - self.0[0].start
    }

    pub fn height(&self) -> <T as Sub>::Output {
        self.0[1].end - self.0[1].start
    }

    pub fn dims(&self) -> (<T as Sub>::Output, <T as Sub>::Output) {
        (self.width(), self.height())
    }
}

impl Rect<i32> {
    pub fn center_x(&self) -> i32 {
        self.left() + self.width() / 2
    }

    pub fn center_y(&self) -> i32 {
        self.top() + self.height() / 2
    }
}

impl Rect<f32> {
    pub fn aspect(&self) -> f32 {
        self.width() / self.height()
    }
}

impl From<&Rect<i32>> for Rect<f32> {
    fn from(other: &Rect<i32>) -> Self {
        Self([
            (other.0[0].start as f32)..(other.0[0].end as f32),
            (other.0[1].start as f32)..(other.0[1].end as f32),
        ])
    }
}

impl From<&Rect<i32>> for Rect<u32> {
    fn from(other: &Rect<i32>) -> Self {
        Self([
            (other.0[0].start as u32)..(other.0[0].end as u32),
            (other.0[1].start as u32)..(other.0[1].end as u32),
        ])
    }
}
