use crate::math::Vector3d;

/// Trait to define a collision type
pub trait Collision {
    type Collider;

    fn collider(&self) -> Self::Collider;
}

/// Trait to import to see if a collision exists
pub trait CollidesWith<Oth> {
    fn collides_with(&self, other: &Oth) -> bool;
}

impl<A,  B> CollidesWith<B> for A where
    A: Collision,
    B: Collision,
    <A as Collision>::Collider: CollidesWith2<<B as Collision>::Collider>,
{
    fn collides_with(&self, other: &B) -> bool {
        let object = self.collider();
        let other = other.collider();
        object.collides_with(&other)
    }
}

/// Trait to define on colliders. Can be replaced with CollidesWith once specialization is a thing.
pub trait CollidesWith2<T> {
    fn collides_with(&self, other: &T) -> bool;
}

impl CollidesWith2<Sphere> for Sphere {
    fn collides_with(&self, other: &Sphere) -> bool {
        let mag = (self.position - other.position).magnitude();
        mag < (self.radius + other.radius)
    }
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

impl Collision for Sphere {
    type Collider = Self;

    fn collider(&self) -> Self {
        self.clone()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new([0.0; 3], 1.0)
    }
}
