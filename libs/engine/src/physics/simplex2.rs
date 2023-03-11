#![allow(clippy::many_single_char_names)]

use std::fmt::{self, Debug};
use std::mem::MaybeUninit;
use std::ops::Index;

use crate::math::{Vector, Vector3d};
use crate::prelude::*;

pub struct Simplex<T, const N: usize> {
    count: u8,
    origin: MaybeUninit<Vector<T, N>>,
    points: [MaybeUninit<Vector<T, N>>; N],
}

impl<T, const N: usize> Simplex<T, N> {
    pub fn new(iter: impl IntoIterator<Item = Vector<T, N>>) -> Self {
        let mut simplex = Self::default();
        for point in iter.into_iter() {
            simplex.add_point(point);
        }
        simplex
    }

    pub fn add_point(&mut self, new_point: impl Into<Vector<T, N>>) -> &mut Self {
        let new_point = new_point.into();
        if self.count == 0 {
            self.origin.write(new_point);
        } else if (1..=N).contains(&(self.count as usize)) {
            self.points[self.count as usize - 1].write(new_point);
        } else {
            panic!("Too many points")
        }
        self.count += 1;
        self
    }

    pub fn points(&self) -> impl Iterator<Item = &Vector<T, N>> {
        (0..self.count).map(|i| &self[i])
    }
}

impl Simplex<f32, 3> {
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
            }
            3 => {
                let [a, b, c] = [self[0], self[1], self[2]];
                let (u, v) = Vector3d::ORIGIN.projection_along_2d([a, b, c]);
                if 0.0 < u && u < 1.0 && 0.0 < v && v < 1.0 && u + v < 1.0 {
                    Some(a.lerp(b, u).lerp(c, v))
                } else {
                    None
                }
            }
            4 => {
                let volume = [self[0], self[1], self[2], self[3]];
                if Vector3d::ORIGIN.contained_by_3d(volume) {
                    Some(Vector3d::ORIGIN)
                } else {
                    None
                }
            }
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

impl<T: Clone, const N: usize> Clone for Simplex<T, N> {
    fn clone(&self) -> Self {
        let mut new = Self::default();
        for index in 0..self.count {
            new.add_point(self[index].clone());
        }
        new
    }
}

impl<T, const N: usize> Default for Simplex<T, N> {
    fn default() -> Self {
        Self {
            count: 0,
            origin: MaybeUninit::uninit(),
            // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
            points: unsafe {
                MaybeUninit::<[MaybeUninit<Vector<T, N>>; N]>::uninit().assume_init()
            },
        }
    }
}

impl<T: Debug, const N: usize> fmt::Debug for Simplex<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.count).map(|i| &self[i]))
            .finish()
    }
}

impl<T, const N: usize> Index<u8> for Simplex<T, N> {
    type Output = Vector<T, N>;

    fn index(&self, index: u8) -> &Self::Output {
        if index >= self.count {
            panic!("Index out of bounds!");
        }

        if index == 0 {
            // SAFETY: if index < count, origin is valid.
            unsafe { self.origin.assume_init_ref() }
        } else {
            // SAFETY: if index < count, points[index - 1] is valid.
            unsafe { self.points[index as usize - 1].assume_init_ref() }
        }
    }
}

impl<T: PartialEq, const N: usize> PartialEq for Simplex<T, N> {
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

impl<T: Into<Vector3d>> From<T> for Simplex<f32, 3> {
    fn from(point: T) -> Self {
        Self::new([point.into()])
    }
}

pub struct SubSimplexes {
    simplex: Simplex<f32, 3>,
    index: usize,
}

impl Iterator for SubSimplexes {
    type Item = Simplex<f32, 3>;

    fn next(&mut self) -> Option<Self::Item> {
        let simplex = match self.simplex.count {
            0 => None,
            1 => match self.index {
                0 => Some(Simplex::new([])),
                _ => None,
            },
            2 => match self.index {
                0 => Some(Simplex::new([self.simplex[0]])),
                1 => Some(Simplex::new([self.simplex[1]])),
                _ => None,
            },
            3 => match self.index {
                0 => Some(Simplex::new([self.simplex[0], self.simplex[1]])),
                1 => Some(Simplex::new([self.simplex[0], self.simplex[2]])),
                2 => Some(Simplex::new([self.simplex[1], self.simplex[2]])),
                _ => None,
            },
            4 => match self.index {
                0 => Some(Simplex::new([
                    self.simplex[0],
                    self.simplex[1],
                    self.simplex[2],
                ])),
                1 => Some(Simplex::new([
                    self.simplex[0],
                    self.simplex[1],
                    self.simplex[3],
                ])),
                2 => Some(Simplex::new([
                    self.simplex[0],
                    self.simplex[2],
                    self.simplex[3],
                ])),
                3 => Some(Simplex::new([
                    self.simplex[1],
                    self.simplex[2],
                    self.simplex[3],
                ])),
                _ => None,
            },
            _ => unimplemented!(),
        };
        self.index += 1;
        simplex
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_closest() {
        let mut simplex = Simplex::default();
        simplex.add_point([1.0, 1.0, 0.0]);
        simplex.add_point([-1.0, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.clone().nearest_simplex().unwrap();
        assert_eq!(new_simplex, simplex);
        assert_eq!(close_point, [0.0, 1.0, 0.0].into());
    }

    #[test]
    fn line_closest_point() {
        let mut simplex = Simplex::default();
        simplex.add_point([1.0, 1.0, 0.0]);
        simplex.add_point([0.5, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.nearest_simplex().unwrap();
        assert_eq!(new_simplex, [0.5, 1.0, 0.0].into());
        assert_eq!(close_point, [0.5, 1.0, 0.0].into());
    }

    #[test]
    fn line_closest_point2() {
        let mut simplex = Simplex::default();
        simplex.add_point([0.5, 1.0, 0.0]);
        simplex.add_point([1.0, 1.0, 0.0]);

        let (new_simplex, close_point) = simplex.nearest_simplex().unwrap();
        assert_eq!(new_simplex, [0.5, 1.0, 0.0].into());
        assert_eq!(close_point, [0.5, 1.0, 0.0].into());
    }
}
