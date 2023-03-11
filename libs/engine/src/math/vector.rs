use std::mem::{self, MaybeUninit};
use std::{convert, iter, ops};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub fn uninit() -> Vector<MaybeUninit<T>, N> {
        // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
        unsafe { Vector(MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init()) }
    }
}

impl<T, const N: usize> Vector<MaybeUninit<T>, N> {
    /// SAFETY: Caller must make sure Vector is initialized
    pub unsafe fn assume_init(self) -> Vector<T, N> {
        let ret = (&self as *const _ as *const Vector<T, N>).read();
        mem::forget(self);
        ret
    }
}

impl<T, const N: usize> Vector<T, N> {
    pub fn dot<Rhs>(self, rhs: impl Into<Vector<Rhs, N>>) -> <T as ops::Mul<Rhs>>::Output
    where
        T: ops::Mul<Rhs>,
        <T as ops::Mul<Rhs>>::Output: iter::Sum,
    {
        self.0
            .into_iter()
            .zip(rhs.into().0)
            .map(|(t, r)| t * r)
            .sum()
    }

    pub fn magnitude_squared(self) -> <T as ops::Mul<T>>::Output
    where
        T: Copy + ops::Mul<T>,
        <T as ops::Mul<T>>::Output: iter::Sum,
    {
        self.0.into_iter().map(|f| f * f).sum()
    }
}

impl<const N: usize> Vector<f32, N> {
    pub fn zero() -> Self {
        Self([0.0; N])
    }

    pub fn lerp(mut self, rhs: impl Into<Self>, delta: f32) -> Self {
        let rhs = rhs.into();
        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s = *s * (1.0 - delta) + r * delta;
        }
        self
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    pub fn set_component(&mut self, direction: impl Into<Self>, magnitude: f32) {
        let direction = direction.into().normalize();
        let old_magnitude = self.dot(direction);

        *self -= direction * old_magnitude;
        *self += direction * magnitude;
    }

    /// Returns fraction of the distance between two points closest to self.
    /// Is outside the segment if less than 0 or greater than 1.
    /// Use with `lerp()` to find a point.
    pub fn projection_along_1d(self, line: [Self; 2]) -> f32 {
        let len2 = (line[1] - line[0]).magnitude_squared();
        (self - line[0]).dot(line[1] - line[0]) / len2
    }

    pub fn projection_along_2d(self, plane: [Self; 3]) -> (f32, f32) {
        // Get location of right triangle base along 0>1 vector
        let base_u = (plane[2]).projection_along_1d([plane[0], plane[1]]);
        let base = plane[0].lerp(plane[1], base_u);
        // Projection along a line perependicular to 0>1 vector and equal to the height of the triangle.
        // This is equal to the independent projection along 0>2
        let v = self.projection_along_1d([base, plane[2]]);
        // Remove the independent projection
        let adjusted_point = self - ((plane[2] - plane[0]) * v);
        // Find the second projection
        let u = adjusted_point.projection_along_1d([plane[0], plane[1]]);
        (u, v)
    }
}

impl<T, const N: usize> convert::From<[T; N]> for Vector<T, N> {
    fn from(array: [T; N]) -> Self {
        Self(array)
    }
}

impl<T, const N: usize> convert::From<Vector<T, N>> for [T; N] {
    fn from(vector: Vector<T, N>) -> Self {
        vector.0
    }
}

impl<T, Rhs, const N: usize> ops::Add<Vector<Rhs, N>> for Vector<T, N>
where
    T: ops::Add<Rhs>,
{
    type Output = Vector<<T as ops::Add<Rhs>>::Output, N>;

    fn add(self, rhs: Vector<Rhs, N>) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, (s, rhs)) in self.0.into_iter().zip(rhs.0).enumerate() {
                new.0[i].write(s + rhs);
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const N: usize> ops::AddAssign<Vector<Rhs, N>> for Vector<T, N>
where
    T: ops::AddAssign<Rhs>,
{
    fn add_assign(&mut self, rhs: Vector<Rhs, N>) {
        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s += r;
        }
    }
}

impl<T, Rhs, const N: usize> ops::Sub<Vector<Rhs, N>> for Vector<T, N>
where
    T: ops::Sub<Rhs>,
{
    type Output = Vector<<T as ops::Sub<Rhs>>::Output, N>;

    fn sub(self, rhs: Vector<Rhs, N>) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, (s, rhs)) in self.0.into_iter().zip(rhs.0).enumerate() {
                new.0[i].write(s - rhs);
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const N: usize> ops::SubAssign<Vector<Rhs, N>> for Vector<T, N>
where
    T: ops::SubAssign<Rhs>,
{
    fn sub_assign(&mut self, rhs: Vector<Rhs, N>) {
        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s -= r;
        }
    }
}

impl<T, Rhs, const N: usize> ops::Mul<Rhs> for Vector<T, N>
where
    T: ops::Mul<Rhs>,
    Rhs: Copy,
{
    type Output = Vector<<T as ops::Mul<Rhs>>::Output, N>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, s) in self.0.into_iter().enumerate() {
                new.0[i].write(s * rhs);
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const N: usize> ops::MulAssign<Rhs> for Vector<T, N>
where
    T: ops::MulAssign<Rhs>,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        for element in &mut self.0 {
            *element *= rhs;
        }
    }
}

impl<T, Rhs, const N: usize> ops::Div<Rhs> for Vector<T, N>
where
    T: ops::Div<Rhs>,
    Rhs: Copy,
{
    type Output = Vector<<T as ops::Div<Rhs>>::Output, N>;

    fn div(self, rhs: Rhs) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, s) in self.0.into_iter().enumerate() {
                new.0[i].write(s / rhs);
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const N: usize> ops::DivAssign<Rhs> for Vector<T, N>
where
    T: ops::DivAssign<Rhs>,
    Rhs: Copy,
{
    fn div_assign(&mut self, rhs: Rhs) {
        for element in &mut self.0 {
            *element /= rhs;
        }
    }
}

impl<T: ops::Neg, const N: usize> ops::Neg for Vector<T, N> {
    type Output = Vector<<T as ops::Neg>::Output, N>;

    fn neg(self) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, s) in self.0.into_iter().enumerate() {
                new.0[i].write(-s);
            }
            new.assume_init()
        }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    [T; N]: Default,
{
    fn default() -> Self {
        Self(<[T; N]>::default())
    }
}
