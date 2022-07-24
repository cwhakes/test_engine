use crate::math::Vector3d;
use crate::physics::simplex::Simplex;

use log::error;

pub trait CollisionEngine {
    type Collider: ?Sized;
    fn collision_between(&mut self, obj0: &Self::Collider, obj1: &Self::Collider) -> bool;

    fn collisions<'a, 'b>(
        &mut self,
        objs: &'a [&'b Self::Collider],
    ) -> Vec<(&'b Self::Collider, &'b Self::Collider)> {
        let mut collisions = Vec::new();
        for i in 0..objs.len() {
            for j in (i + 1)..objs.len() {
                if self.collision_between(objs[i], objs[j]) {
                    collisions.push((objs[i], objs[j]));
                }
            }
        }
        collisions
    }
}

pub struct GjkEngine;

impl CollisionEngine for GjkEngine {
    type Collider = dyn GjkCollider;

    fn collision_between(&mut self, obj0: &Self::Collider, obj1: &Self::Collider) -> bool {
        let initial_dir = Vector3d::RIGHT;
        let mut point = obj0.support(initial_dir) - obj1.support(-initial_dir);
        let mut simplex = Simplex::new();
        simplex.add_point(point);
        let mut dir = Vector3d::ORIGIN - point;

        for _ in 0..500 {
            point = obj0.support(dir) - obj1.support(-dir);
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

/// Trait to implement GJK <https://cse442-17f.github.io/Gilbert-Johnson-Keerthi-Distance-Algorithm/>
pub trait GjkCollider {
    /// Given an angle, find a point that is furthest away from the shape's origin.
    fn support(&self, angle: Vector3d) -> Vector3d;

    fn bounding_box(&self) -> (Vector3d, Vector3d) {
        let a_x = self.support([-1.0, 0.0, 0.0].into()).x();
        let a_y = self.support([0.0, -1.0, 0.0].into()).y();
        let a_z = self.support([0.0, 0.0, -1.0].into()).z();

        let b_x = self.support([1.0, 0.0, 0.0].into()).x();
        let b_y = self.support([0.0, 1.0, 0.0].into()).y();
        let b_z = self.support([0.0, 0.0, 1.0].into()).z();

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

impl GjkCollider for Sphere {
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
    type Collider: GjkCollider;

    fn collider(&self) -> Self::Collider;
}

impl<T> GjkCollider for T
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
        let mut gjk = GjkEngine;

        let s0 = Sphere::new([0.0, 0.0, 0.0], 0.6);
        let s1 = Sphere::new([1.0, 0.0, 0.0], 0.6);
        let s2 = Sphere::new([1.0, 1.0, 0.0], 0.6);

        assert!(gjk.collision_between(&s0, &s1));
        assert!(!gjk.collision_between(&s0, &s2));
        assert!(gjk.collision_between(&s1, &s2));
    }
}
