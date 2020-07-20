use super::Vector4d;

use std::{convert, ops};

use float_cmp::approx_eq;
use wavefront_obj::obj;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub const ORIGIN: Vector3d = Vector3d { x: 0.0, y: 0.0, z: 0.0 };
    pub const RIGHT: Vector3d = Vector3d { x: 1.0, y: 0.0, z: 0.0 };
    pub const UP: Vector3d = Vector3d { x: 0.0, y: 0.0, z: 1.0 };
    pub const FORWARD: Vector3d = Vector3d { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }

    pub fn magnitude(self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
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

    pub fn set_component(&mut self, new_component: impl Into<Vector3d>) {
        let new_component = new_component.into();
        let new_mag = new_component.magnitude();
        if new_mag <= 0.0 {
            return;
        };

        let direction = new_component.clone().normalize();
        let old_mag = self.clone().dot(direction.clone());
        *self -= direction * old_mag;
        *self += new_component;
    }

    pub fn lerp(self, other: impl Into<Vector3d>, delta: f32) -> Vector3d  {
        let other = other.into();
        Vector3d {
            x: self.x * (1.0 - delta) + other.x * delta,
            y: self.y * (1.0 - delta) + other.y * delta,
            z: self.z * (1.0 - delta) + other.z * delta,
        }
    }

    pub fn dot(self, rhs: impl Into<Vector3d>) -> f32 {
        let rhs = rhs.into();
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z
    }

    pub fn cross(self, rhs: impl Into<Vector3d>) -> Vector3d {
        let rhs = rhs.into();
        Vector3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn distance_to_line(self, line: (Vector3d, Vector3d)) -> f32 {
        let line_length = (line.1 - line.0).magnitude();
        assert!(line_length > 0.0);
        let ray0 = line.0 - self;
        let ray1 = line.1 - self;
        let area = ray0.cross(ray1).magnitude();
        area / line_length
    }

    /// Returns fraction of the distance between two points closest to self.
    /// Is outside the segment if less than 0 or greater than 1.
    /// Use with `lerp()` to find a point.
    pub fn projection_along_1d(self, line: [Vector3d; 2]) -> f32 {
        let len2 = (line[0] - line[1]).magnitude_squared();
        (self - line[0]).dot(line[1] - line[0]) / len2
    }

    pub fn closest_point_on_plane(self, plane: (Vector3d, Vector3d, Vector3d)) -> Vector3d {
        let normal = (plane.1 - plane.0).cross(plane.2 - plane.0).normalize();
        let distance = (plane.0 - self).dot(normal.clone());
        let projection = (plane.0 - self) - (normal * distance);
        plane.0 + projection
    }

    pub fn projection_along_2d(self, plane: [Vector3d; 3]) -> (f32, f32) {
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

    pub fn bounded_by_1d(self, line: [Vector3d; 2]) -> bool {
        let area2_of_tri = (self - line[0]).cross(line[1] - line[0]).magnitude_squared();
        let proj = self.projection_along_1d(line);

        approx_eq!( f32, 0.0, area2_of_tri ) &&
            0.0 <= proj && proj <= 1.0
    }

    pub fn bounded_by_2d(&self, plane: [Vector3d; 3]) -> bool {
        let volume_of_cube = (plane[1] - plane[0]).cross(plane[1] - plane[0]).dot(*self - plane[0]).abs();
        let (u, v) = self.projection_along_2d(plane);

        0.0 <= u && u <= 1.0 &&
            0.0 <= v && v <= 1.0 &&
            u + v <= 1.0 &&
            approx_eq!(f32, 0.0, volume_of_cube)
    }

    pub fn contained_by_3d(&self, tetrahedron: [Vector3d; 4]) -> bool {
        let t = tetrahedron;
        let p0 = (t[1] - t[0]).cross(t[2] - t[0]);
        let p1 = (t[2] - t[0]).cross(t[3] - t[0]);
        let p2 = (t[3] - t[0]).cross(t[1] - t[0]);
        let p3 = (t[3] - t[1]).cross(t[2] - t[1]);

        let sign = (*self - t[0]).dot(p0).signum();
        sign == (*self - t[0]).dot(p1).signum() &&
        sign == (*self - t[0]).dot(p2).signum() &&
        sign == (*self - t[1]).dot(p3).signum()
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

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Into<Vector3d>> ops::AddAssign<T> for Vector3d {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Into<Vector3d>> ops::Sub<T> for Vector3d {
    type Output = Vector3d;

    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Into<Vector3d>> ops::SubAssign<T> for Vector3d {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Vector3d {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3d {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

#[cfg(test)]
mod test {
    use super::*;
    use rand::{self, prelude::*};

    #[test]
    fn closest_points() {
        let mut rng = rand::thread_rng();

        let origin = Vector3d::new(rng.gen(), rng.gen(), rng.gen());
        let p1 = Vector3d::new(rng.gen(), rng.gen(), rng.gen());
        let p2 = Vector3d::new(rng.gen(), rng.gen(), rng.gen());

        let proj = origin.projection_along_1d([p1, p2]);
        let point = p1.lerp(p2.clone(), proj);
        let distance = (point - origin).magnitude();
        
        let same_distance = origin.distance_to_line((p1.clone(), p2.clone()));

        assert!((distance - same_distance).abs() < 0.001);
    }

    #[test]
    fn closest_points_uv() {
        let mut rng = rand::thread_rng();

        let origin = Vector3d::new(rng.gen(), rng.gen(), rng.gen());
        let p1 = Vector3d::new(rng.gen(), rng.gen(), rng.gen());
        let p2 = Vector3d::new(rng.gen(), rng.gen(), rng.gen());
        let p3 = Vector3d::new(rng.gen(), rng.gen(), rng.gen());

        let (u0, v0) = origin.projection_along_2d([p1, p2, p3]);
        let (v1, u1) = origin.projection_along_2d([p1, p3, p2]);

        assert!((u1 - u0).abs() < 0.001);
        assert!((v1 - v0).abs() < 0.001);
    }

    #[test]
    fn contains_origin() {
        let origin = Vector3d::ORIGIN;
        let p1 = [1.0, 0.0, 0.0].into();
        let p2 = [0.0, 1.0, 0.0].into();
        let p4 = [-0.5, -0.5, 0.5].into();
        let p3 = [-0.5, -0.5, -0.5].into();

        assert!(origin.contained_by_3d([p1, p2, p3, p4]));
        assert!(origin.contained_by_3d([p1, p2, p4, p3]));
    }

    #[test]
    fn test_projection_along_1d() {
        let origin = Vector3d::ORIGIN;
        let p1 = [10.0, 0.0, 0.0].into();
        let p2: Vector3d = [5.0, 5.0, 0.0].into();

        let proj = dbg!(p2.projection_along_1d([origin, p1]));

        assert!( approx_eq!(f32, 0.5, proj));
    }
}
