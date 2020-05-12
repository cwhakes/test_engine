use crate::math::Vector3d;

pub trait Collision<Obj> {
    fn collider(&self) -> Obj;
}

pub trait CollidesWith<Obj, Oth>: Collision<Obj> {
    fn collides_with<C: Collision<Oth>>(&self, other: &C) -> bool;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub position: Vector3d,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: impl Into<Vector3d>, radius: f32 ) -> Sphere {
        Sphere {
            position: position.into(),
            radius,
        }
    }
}

impl Collision<Sphere> for Sphere {
    fn collider(&self) -> Sphere {
        self.clone()
    }
}

impl<T: Collision<Sphere>> CollidesWith<Sphere, Sphere> for T {
    fn collides_with<C: Collision<Sphere>>(&self, other: &C) -> bool {
        let obj = self.collider();
        let oth = other.collider();

        let mag = (obj.position.clone() - oth.position.clone()).magnitude();
        mag < (obj.radius + oth.radius)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new([0.0; 3], 1.0)
    }
}
