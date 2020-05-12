use crate::math::Vector3d;

pub trait Collision<Oth> {
    fn collides_with(&self, other: &Oth) -> bool;
}

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
    fn collides_with(&self, other: &Sphere) -> bool {
        let mag = (other.position.clone() - self.position.clone()).magnitude();
        mag < (self.radius + other.radius)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new([0.0; 3], 1.0)
    }
}
