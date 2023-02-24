#![allow(clippy::many_single_char_names)]

use std::fmt;
use std::mem::MaybeUninit;
use std::ops::Index;

use crate::math::Vector3d;
use crate::prelude::*;

#[derive(Clone)]
pub struct Simplex {
    count: u8,
    origin: MaybeUninit<Vector3d>,
    points: [MaybeUninit<Vector3d>; 3],
}

impl Simplex {
    pub fn new() -> Self {
        Self {
            count: 0,
            origin: MaybeUninit::uninit(),
            points: [
                MaybeUninit::uninit(),
                MaybeUninit::uninit(),
                MaybeUninit::uninit(),
            ]
        }
    }

    pub fn from_iter(iter: impl IntoIterator<Item = Vector3d>) -> Self {
        let mut simplex = Self::new();
        for point in iter.into_iter() {
            simplex.add_point(point);
        }
        simplex

    }

    pub fn add_point(&mut self, new_point: impl Into<Vector3d>) -> &mut Self {
        let new_point = new_point.into();
        match self.count {
            0 => {self.origin.write(new_point);},
            i @ 1..=3 => {self.points[i as usize - 1].write(new_point);},
            4.. => unimplemented!(),
        }
        self.count += 1;
        self
    }

    pub fn subsimplexes(&self) -> SubSimplexes {
        SubSimplexes {
            simplex: self.clone(),
            index: 0,
        }
    }

    fn nearest_point_within_simplex(&self) -> Option<Vector3d> {
        match self.count {
            0 => None,
            1 => Some(self[0]),
            2 => {
                let [a, b] = [self[0], self[1]];
                let proj = Vector3d::ORIGIN.projection_along_1d([a, b]);
                if 0.0 < proj && proj < 1.0 {
                    Some(a.lerp(b, proj))
                } else {
                    None
                }
            },
            3 => {
                let [a, b, c] = [self[0], self[1], self[2]];
                let (u, v) = Vector3d::ORIGIN.projection_along_2d([a, b, c]);
                if 0.0 < u && u < 1.0 && 0.0 < v && v < 1.0 && u + v < 1.0 {
                    Some(a.lerp(b, u).lerp(c, v))
                } else {
                    None
                }
            },
            4 => {
                let volume = [self[0], self[1], self[2], self[3]];
                if Vector3d::ORIGIN.contained_by_3d(volume) {
                    Some(Vector3d::ORIGIN)
                } else {
                    None
                }
            },
            _ => unimplemented!(),
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
        match self.count {
            1 => self[0].magnitude_squared() < 0.001,
            2 => Vector3d::ORIGIN.bounded_by_1d([self[0], self[1]]),
            3 => Vector3d::ORIGIN.bounded_by_2d([self[0], self[1], self[2]]),
            4 => Vector3d::ORIGIN.contained_by_3d([self[0], self[1], self[2], self[4]]),
            _ => false,
        }
    }
}

impl Index<u8> for Simplex {
    type Output = Vector3d;

    fn index(&self, index: u8) -> &Self::Output {
        if index >= self.count {
            panic!("Index out of bounds!");
        }

        if index == 0 {
            unsafe { self.origin.assume_init_ref() }
        } else {
            unsafe { self.points[index as usize - 1].assume_init_ref() }
        }
    }
}

impl PartialEq for Simplex {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false;
        }
        
        for i in 0..self.count {
            if self[i] != other[i] {
                return false;
            }
        }

        true
    }
}

impl fmt::Debug for Simplex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries((0..self.count).map(|i| self[i])).finish()
    }
}

pub struct SubSimplexes {
    simplex: Simplex,
    index: usize,
}

impl Iterator for SubSimplexes {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        let simplex = match self.simplex.count {
            0 => None,
            1 => match self.index {
                0 => Some(Simplex::new()),
                _ => None,
            },
            2 => match self.index {
                0 => Some(Simplex::from_iter([self.simplex[0]])),
                1 => Some(Simplex::from_iter([self.simplex[1]])),
                _ => None,
            },
            3 => match self.index {
                0 => Some(Simplex::from_iter([self.simplex[0], self.simplex[1]])),
                1 => Some(Simplex::from_iter([self.simplex[0], self.simplex[2]])),
                2 => Some(Simplex::from_iter([self.simplex[1], self.simplex[2]])),
                _ => None,
            },
            4 => match self.index {
                0 => Some(Simplex::from_iter([self.simplex[0], self.simplex[1], self.simplex[2]])),
                1 => Some(Simplex::from_iter([self.simplex[0], self.simplex[1], self.simplex[3]])),
                2 => Some(Simplex::from_iter([self.simplex[0], self.simplex[2], self.simplex[3]])),
                3 => Some(Simplex::from_iter([self.simplex[1], self.simplex[2], self.simplex[3]])),
                _ => None,
            },
            _ => unimplemented!(),
        };
        self.index += 1;
        simplex
    }
}

impl Default for Simplex {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Into<Vector3d>> From<T> for Simplex {
    fn from(point: T) -> Self {
        Self::from_iter([point.into()])
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
