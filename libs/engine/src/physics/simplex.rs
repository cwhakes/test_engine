#![allow(clippy::many_single_char_names)]

use crate::math::{Vector, Vector3d};
use crate::prelude::*;

use Simplex::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Simplex {
    Null([Vector3d; 0]),
    Point([Vector3d; 1]),
    Line([Vector3d; 2]),
    Triangle([Vector3d; 3]),
    Tetrahedron([Vector3d; 4]),
}

impl Simplex {
    pub fn new() -> Self {
        Null([])
    }

    pub fn add_point(&mut self, new_point: impl Into<Vector3d>) -> &mut Self {
        let new_point = new_point.into();
        *self = match self {
            Null([]) => Point([new_point]),
            Point([a]) => Line([*a, new_point]),
            Line([a, b]) => Triangle([*a, *b, new_point]),
            Triangle([a, b, c]) => Tetrahedron([*a, *b, *c, new_point]),
            Tetrahedron(_) => unimplemented!(),
        };
        self
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
                let proj = Vector3d::ORIGIN.projection_along_1d([a, b]);
                if 0.0 < proj && proj < 1.0 {
                    Some(a.lerp(b, proj))
                } else {
                    None
                }
            }
            Triangle([a, b, c]) => {
                let Vector([u, v]) = Vector3d::ORIGIN.projection_along_2d([a, b, c]);
                if 0.0 < u && u < 1.0 && 0.0 < v && v < 1.0 && u + v < 1.0 {
                    Some(a.lerp(b, u).lerp(c, v))
                } else {
                    None
                }
            }
            Tetrahedron(volume) => {
                if Vector3d::ORIGIN.contained_by_3d(volume) {
                    Some(Vector3d::ORIGIN)
                } else {
                    None
                }
            }
        }
    }

    /// Returns the sub-simplex that is nearest to the origin, along with the coordinates of the closest point
    pub fn nearest_simplex(self) -> Option<(Self, Vector3d)> {
        // Early return if point is on simplex itself
        if let Some(vec) = self.nearest_point_within_simplex() {
            return Some((self, vec));
        }

        self.subsimplexes()
            .filter_map(Self::nearest_simplex)
            .partial_min_by_key(|(_, v)| v.magnitude_squared())
    }

    pub fn contains_origin(&self) -> bool {
        match self.clone() {
            Point([a]) => a.magnitude_squared() < 0.001,
            Line(line) => Vector3d::ORIGIN.bounded_by_1d(line),
            Triangle(plane) => Vector3d::ORIGIN.bounded_by_2d(plane),
            Tetrahedron(volume) => Vector3d::ORIGIN.contained_by_3d(volume),
            _ => false,
        }
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
            },
            Line([a, b]) => match self.index {
                0 => Some(Point([a])),
                1 => Some(Point([b])),
                _ => None,
            },
            Triangle([a, b, c]) => match self.index {
                0 => Some(Line([a, b])),
                1 => Some(Line([a, c])),
                2 => Some(Line([b, c])),
                _ => None,
            },
            Tetrahedron([a, b, c, d]) => match self.index {
                0 => Some(Triangle([a, b, c])),
                1 => Some(Triangle([a, b, d])),
                2 => Some(Triangle([a, c, d])),
                3 => Some(Triangle([b, c, d])),
                _ => None,
            },
        };
        self.index += 1;
        simplex
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Null([])
    }
}

impl<T: Into<Vector3d>> From<T> for Simplex {
    fn from(point: T) -> Self {
        let point = point.into();
        Point([point])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_closest() {
        let mut simplex = Simplex::new();
        simplex.add_point([1.0, 1.0, 0.0]);
        simplex.add_point([-1.0, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.clone().nearest_simplex().unwrap();
        assert_eq!(new_simplex, simplex);
        assert_eq!(close_point, [0.0, 1.0, 0.0].into());
    }

    #[test]
    fn line_closest_point() {
        let mut simplex = Simplex::new();
        simplex.add_point([1.0, 1.0, 0.0]);
        simplex.add_point([0.5, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.nearest_simplex().unwrap();
        assert_eq!(new_simplex, [0.5, 1.0, 0.0].into());
        assert_eq!(close_point, [0.5, 1.0, 0.0].into());
    }

    #[test]
    fn line_closest_point2() {
        let mut simplex = Simplex::new();
        simplex.add_point([0.5, 1.0, 0.0]);
        simplex.add_point([1.0, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.nearest_simplex().unwrap();
        assert_eq!(new_simplex, [0.5, 1.0, 0.0].into());
        assert_eq!(close_point, [0.5, 1.0, 0.0].into());
    }
}
