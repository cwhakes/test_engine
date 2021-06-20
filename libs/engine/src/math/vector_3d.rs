use super::{Vector, Vector4d};

use std::convert;

use float_cmp::approx_eq;
use wavefront_obj::obj;

pub type Vector3d = Vector<f32, 3>;

impl Vector3d {
    pub const ORIGIN: Self = Self([0.0, 0.0, 0.0]);
    pub const RIGHT: Self = Self([1.0, 0.0, 0.0]);
    pub const UP: Self = Self([0.0, 1.0, 0.0]);
    pub const FORWARD: Self = Self([0.0, 0.0, 1.0]);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.0[2]
    }

    pub fn to_4d(self, w: f32) -> Vector4d {
        Vector([
            self.x(),
            self.y(),
            self.z(),
            w,
        ])
    }

    pub fn cross(self, rhs: impl Into<Self>) -> Self {
        let rhs = rhs.into();
        Self([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }

    pub fn distance_to_line(self, line: (Self, Self)) -> f32 {
        let line_length = (line.1 - line.0).magnitude();
        assert!(line_length > 0.0);
        let ray0 = line.0 - self;
        let ray1 = line.1 - self;
        let area = ray0.cross(ray1).magnitude();
        area / line_length
    }

    pub fn closest_point_on_plane(self, plane: (Self, Self, Self)) -> Self {
        let normal = (plane.1 - plane.0).cross(plane.2 - plane.0).normalize();
        let distance = (plane.0 - self).dot(normal);
        let projection = (plane.0 - self) - (normal * distance);
        plane.0 + projection
    }

    pub fn bounded_by_1d(self, line: [Self; 2]) -> bool {
        let area2_of_tri = (self - line[0])
            .cross(line[1] - line[0])
            .magnitude_squared();
        let proj = self.projection_along_1d(line);

        approx_eq!(f32, 0.0, area2_of_tri) && (0.0..=1.0).contains(&proj)
    }

    pub fn bounded_by_2d(&self, plane: [Self; 3]) -> bool {
        let volume_of_cube = (plane[1] - plane[0])
            .cross(plane[1] - plane[0])
            .dot(*self - plane[0])
            .abs();
        let (u, v) = self.projection_along_2d(plane);

        (0.0..=1.0).contains(&u)
            && (0.0..=1.0).contains(&v)
            && u + v <= 1.0
            && approx_eq!(f32, 0.0, volume_of_cube)
    }

    pub fn contained_by_3d(&self, tetrahedron: [Self; 4]) -> bool {
        let t = tetrahedron;
        //Calculate normals
        let p0 = (t[1] - t[0]).cross(t[2] - t[0]);
        let p1 = (t[2] - t[0]).cross(t[3] - t[0]);
        let p2 = (t[3] - t[0]).cross(t[1] - t[0]);
        let p3 = (t[3] - t[1]).cross(t[2] - t[1]);

        //If the signs are the same, all normals are pointing either inwards or outwards
        let sign = (*self - t[0]).dot(p0).signum();
        0.0 < sign * (*self - t[0]).dot(p1).signum()
            && 0.0 < sign * (*self - t[0]).dot(p2).signum()
            && 0.0 < sign * (*self - t[1]).dot(p3).signum()
    }
}

impl convert::From<obj::Vertex> for Vector3d {
    fn from(vertex: obj::Vertex) -> Self {
        Self([vertex.x as f32, vertex.y as f32, vertex.z as f32])
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

        assert!(approx_eq!(f32, 0.5, proj));
    }
}
