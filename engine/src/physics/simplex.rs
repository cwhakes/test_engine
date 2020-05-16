use crate::prelude::*;
use crate::math::Vector3d;

use Simplex::*;

#[derive(Clone, Debug)]
pub enum Simplex {
    Null([Vector3d; 0]),
    Point([Vector3d; 1]),
    Line([Vector3d; 2]),
    Triangle([Vector3d; 3]),
    Tetrahedron([Vector3d; 4]),
}

impl Simplex {
    pub fn add_point(self, new_point: impl Into<Vector3d>) -> Self {
        let new_point = new_point.into();
        match self {
            Null([]) => Point([new_point]),
            Point([a]) => Line([a, new_point]),
            Line([a, b]) => Triangle([a, b, new_point]),
            Triangle([a, b, c]) => Tetrahedron([a, b, c, new_point]),
            Tetrahedron(_) => unimplemented!(),
        }
    }

    pub fn subsimplexes(&self) -> SubSimplexes {
        SubSimplexes {
            simplex: self.clone(),
            index: 0,
        }
    }

    pub fn nearest_point_within_simplex(&self) -> Option<Vector3d> {
        match self.clone() {
            Null(_) => None,
            Point([a]) => Some(a),
            Line([a, b]) => {
                let proj = Vector3d::ORIGIN.projection_along_line((a, b));
                if 0.0 < proj && proj < 1.0 {
                    Some(a.lerp(b, proj))
                } else { None }
            }
            Triangle([a, b, c]) => {
                let (u, v) = Vector3d::ORIGIN.projection_along_plane((a, b, c));
                if 0.0 < u && u < 1.0 &&
                    0.0 < v && v < 1.0 &&
                    u + v < 1.0
                {
                    Some(a.lerp(b, u).lerp(c, v))
                } else { None }
            }
            Tetrahedron([a, b, c, d]) => {
                if Vector3d::ORIGIN.contained_by((a, b, c, d)) {
                    Some(Vector3d::ORIGIN)
                } else { None }
            }
        }
    }

    /// Returns the sub-simplex that is nearest to the origin, along with the coordinates of the closest point
    pub fn nearest_simplex(&self) -> Option<(Simplex, Vector3d)> {
        // Early return if point is on simplex itself
        if let Some(vec) = self.nearest_point_within_simplex() {
            return Some((self.clone(), vec));
        }

        self.subsimplexes()
            .flat_map(|s| s.nearest_simplex())
            .partial_min_by_key(|(_, v)| v.magnitude_squared())
    }
}

pub struct SubSimplexes {
    simplex: Simplex,
    index: usize,
}

impl Iterator for SubSimplexes {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        let simplex = match self.simplex {
            Null([]) => None,
            Point([_]) => match self.index {
                0 => Some(Null([])),
                _ => None,
            }
            Line([a, b]) => match self.index {
                0 => Some(Point([a])),
                1 => Some(Point([b])),
                _ => None,
            }
            Triangle([a, b, c]) =>  match self.index {
                0 => Some(Line([a, b])),
                1 => Some(Line([a, c])),
                2 => Some(Line([b, c])),
                _ => None,
            }
            Tetrahedron([a, b, c, d]) =>  match self.index {
                0 => Some(Triangle([a, b, c])),
                1 => Some(Triangle([a, b, d])),
                2 => Some(Triangle([a, c, d])),
                3 => Some(Triangle([b, c, d])),
                _ => None,
            }
        };
        self.index += 1;
        simplex
    }
}

impl<T: Into<Vector3d>> From<T> for Simplex {
    fn from(point: T) -> Self {
        let point = point.into();
        Point([point])
    }
}
