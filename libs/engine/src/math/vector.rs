use std::{convert, ops};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<const N: usize> Vector<f32, N> {
    pub fn magnitude_squared(self) -> f32 {
        std::array::IntoIter::new(self.0).map(|f| f * f).sum()
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }
}

impl<const N: usize> Vector<f32, N> where [f32; N]: Default {
    pub fn lerp(&self, other: impl Into<Self>, delta: f32) -> Self {
        let other = other.into();
        let mut new = Self(<[f32; N]>::default());
        for ((n, s), o) in (new.0.iter_mut()).zip(self.0).zip(other.0) {
            *n = s * (1.0 - delta) + o * delta;
        }
        new
    }
}

impl<T, const N: usize> convert::From<[T; N]> for Vector<T, N> {
    fn from(array: [T; N]) -> Self {
        Self(array)
    }
}

impl<Rhs: Into<Self>, const N: usize> ops::Add<Rhs> for Vector<f32, N> {
    type Output = Self;

    fn add(mut self, rhs: Rhs) -> Self::Output {
        self += rhs;
        self
    }
}

impl<Rhs: Into<Self>, const N: usize> ops::AddAssign<Rhs> for Vector<f32, N> {
    fn add_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into();

        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s += r
        }
    }
}

impl<Rhs: Into<Self>, const N: usize> ops::Sub<Rhs> for Vector<f32, N> {
    type Output = Self;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<Rhs: Into<Self>, const N: usize> ops::SubAssign<Rhs> for Vector<f32, N> {
    fn sub_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into();

        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s -= r
        }
    }
}

impl<const N: usize> ops::Mul<f32> for Vector<f32, N> {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        for element in &mut self.0 {
            *element *= rhs
        }
        self
    }
}

impl<const N: usize> ops::Div<f32> for Vector<f32, N> {
    type Output = Self;

    fn div(mut self, rhs: f32) -> Self::Output {
        for element in &mut self.0 {
            *element /= rhs
        }
        self
    }
}

impl<T, const N: usize> Default for Vector<T, N> where [T; N]: Default {
    fn default() -> Self {
        Self(<[T; N]>::default())
    }
}
