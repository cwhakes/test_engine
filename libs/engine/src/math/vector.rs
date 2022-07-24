use std::{convert, ops};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T, const N: usize>(pub [T; N]);

impl<const N: usize> Vector<f32, N> {
    pub fn lerp(mut self, rhs: impl Into<Self>, delta: f32) -> Self {
        let rhs = rhs.into();
        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s = *s * (1.0 - delta) + r * delta;
        }
        self
    }

    pub fn magnitude_squared(self) -> f32 {
        self.0.into_iter().map(|f| f * f).sum()
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        self / mag
    }

    pub fn dot(self, rhs: impl Into<Self>) -> f32 {
        self.0
            .into_iter()
            .zip(rhs.into().0)
            .map(|(s, r)| s * r)
            .sum()
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

impl<T, Rhs: Into<Self>, const N: usize> ops::Add<Rhs> for Vector<T, N>
where
    T: ops::AddAssign, // TODO: Use `ops::Add`
{
    type Output = Self;

    fn add(mut self, rhs: Rhs) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T, Rhs: Into<Self>, const N: usize> ops::AddAssign<Rhs> for Vector<T, N>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into();

        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s += r
        }
    }
}

impl<T, Rhs: Into<Self>, const N: usize> ops::Sub<Rhs> for Vector<T, N>
where
    T: ops::SubAssign, // TODO: Use `ops::Sub`
{
    type Output = Self;

    fn sub(mut self, rhs: Rhs) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T, Rhs: Into<Self>, const N: usize> ops::SubAssign<Rhs> for Vector<T, N>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        let rhs = rhs.into();

        for (s, r) in self.0.iter_mut().zip(rhs.0) {
            *s -= r
        }
    }
}

impl<T, Rhs, const N: usize> ops::Mul<Rhs> for Vector<T, N>
where
    T: ops::MulAssign<Rhs>, // TODO: use `ops::Mul`
    Rhs: Clone,
{
    type Output = Self;

    fn mul(mut self, rhs: Rhs) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T, Rhs, const N: usize> ops::MulAssign<Rhs> for Vector<T, N>
where
    T: ops::MulAssign<Rhs>,
    Rhs: Clone,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        for element in &mut self.0 {
            *element *= rhs.clone()
        }
    }
}

impl<T, Rhs, const N: usize> ops::Div<Rhs> for Vector<T, N>
where
    T: ops::DivAssign<Rhs>, // TODO: use `ops::Div`
    Rhs: Clone,
{
    type Output = Self;

    fn div(mut self, rhs: Rhs) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T, Rhs, const N: usize> ops::DivAssign<Rhs> for Vector<T, N>
where
    T: ops::DivAssign<Rhs>,
    Rhs: Clone,
{
    fn div_assign(&mut self, rhs: Rhs) {
        for element in &mut self.0 {
            *element /= rhs.clone();
        }
    }
}

impl<T: ops::Neg<Output = T> + Clone, const N: usize> ops::Neg for Vector<T, N> {
    type Output = Self;

    fn neg(mut self) -> Self {
        for element in &mut self.0 {
            *element = -element.clone();
        }

        self
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
