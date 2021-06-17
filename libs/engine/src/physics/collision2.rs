use crate::math::Vector3d;
use crate::physics::simplex::Simplex;

use log::error;

pub trait ConvexCollision: ConvexCollider {
    fn collides_with<T: ConvexCollider>(&self, other: &T) -> bool {
        let initial_dir = Vector3d::RIGHT;
        let mut point = self.support(initial_dir) - other.support(-initial_dir);
        let mut simplex = Simplex::new();
        simplex.add_point(point);
        let mut dir = Vector3d::ORIGIN - point;

        for _ in 0..500 {
            point = self.support(dir) - other.support(-dir);
            if dir.dot(point) < 0.0 {
                return false;
            }
            simplex.add_point(point);
            let (new_simplex, closest_point) = simplex.nearest_simplex().unwrap();
            simplex = new_simplex;
            dir = Vector3d::ORIGIN - closest_point;
            if simplex.contains_origin() {
                return true;
            }
        }
        error!("Warning: Infinite loop");
        false
    }
}

impl<T: ConvexCollider> ConvexCollision for T {}

/// Trait to implement GJK https://cse442-17f.github.io/Gilbert-Johnson-Keerthi-Distance-Algorithm/
pub trait ConvexCollider {
    /// Given an angle, find a point that is furthest away from the shape's origin.
    fn support(&self, angle: Vector3d) -> Vector3d;

    fn bounding_box(&self) -> (Vector3d, Vector3d) {
        let a_x = self.support([-1.0, 0.0, 0.0].into()).x;
        let a_y = self.support([0.0, -1.0, 0.0].into()).y;
        let a_z = self.support([0.0, 0.0, -1.0].into()).z;

        let b_x = self.support([1.0, 0.0, 0.0].into()).x;
        let b_y = self.support([0.0, 1.0, 0.0].into()).y;
        let b_z = self.support([0.0, 0.0, 1.0].into()).z;

        let a = Vector3d::new(a_x, a_y, a_z);
        let b = Vector3d::new(b_x, b_y, b_z);

        (a, b)
    }
}

pub struct Sphere {
    position: Vector3d,
    radius: f32,
}

impl Sphere {
    pub fn new(position: impl Into<Vector3d>, radius: f32) -> Self {
        Self {
            position: position.into(),
            radius,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new([0.0; 3], 1.0)
    }
}

impl ConvexCollider for Sphere {
    fn support(&self, angle: Vector3d) -> Vector3d {
        angle.normalize() * self.radius + self.position
    }

    fn bounding_box(&self) -> (Vector3d, Vector3d) {
        let r = self.radius;
        let a = self.position - Vector3d::new(r, r, r);
        let b = self.position + Vector3d::new(r, r, r);

        (a, b)
    }
}

pub trait InheritedCollider {
    type Collider: ConvexCollider;

    fn collider(&self) -> Self::Collider;
}

impl<T> ConvexCollider for T
where
    T: InheritedCollider,
{
    fn support(&self, angle: Vector3d) -> Vector3d {
        self.collider().support(angle)
    }

    fn bounding_box(&self) -> (Vector3d, Vector3d) {
        self.collider().bounding_box()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sphere_collision() {
        let s0 = Sphere::new([0.0, 0.0, 0.0], 0.6);
        let s1 = Sphere::new([1.0, 0.0, 0.0], 0.6);
        let s2 = Sphere::new([1.0, 1.0, 0.0], 0.6);

        assert!(s0.collides_with(&s1));
        assert!(!s0.collides_with(&s2));
        assert!(s1.collides_with(&s2));
    }
}
