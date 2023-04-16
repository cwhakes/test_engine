use super::Vector;

use std::mem::{self, MaybeUninit};
use std::{iter, ops};

use float_cmp::approx_eq;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn uninit() -> Matrix<MaybeUninit<T>, M, N> {
        // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
        unsafe { Matrix(MaybeUninit::<[[MaybeUninit<T>; N]; M]>::uninit().assume_init()) }
    }
}

impl<T, const M: usize, const N: usize> Matrix<MaybeUninit<T>, M, N> {
    /// SAFETY: Caller must make sure Matrix is initialized
    pub unsafe fn assume_init(self) -> Matrix<T, M, N> {
        let ret = (&self as *const _ as *const Matrix<T, M, N>).read();
        mem::forget(self);
        ret
    }
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn transpose(self) -> Matrix<T, N, M> {
        // SAFETY: All elements are written to
        unsafe {
            let mut new = Matrix::uninit();
            for (i, row) in self.0.into_iter().enumerate() {
                for (j, ele) in row.into_iter().enumerate() {
                    new[(j, i)].write(ele);
                }
            }
            new.assume_init()
        }
    }
}

impl<const M: usize, const N: usize> Matrix<f32, M, N> {
    pub fn zero() -> Self {
        Self([[0.0; N]; M])
    }

    pub fn rref(mut self) -> Self {
        let (mut i, mut j) = (0, 0);
        'outer: while i < M && j < N {
            // Swap rows if pivot is zero
            if approx_eq!(f32, 0.0, self[(i, j)]) {
                for i_p in (i + 1)..M {
                    if !approx_eq!(f32, 0.0, self[(i_p, j)]) {
                        self.0.swap(i, i_p);
                        continue 'outer;
                    }
                }
                // If we can't find a non-zero element in this column, advance pivot column
                j += 1;
                continue 'outer;
            }
            // Reduce this row. f can't be zero because of previous check
            let f = self[(i, j)];
            for j_p in j..N {
                self[(i, j_p)] /= f;
            }
            // Reduce other rows
            for i_p in 0..M {
                if i_p != i {
                    let f = self[(i_p, j)];
                    for j_p in j..N {
                        self[(i_p, j_p)] -= f * self[(i, j_p)];
                    }
                }
            }
            // Advance pivot
            i += 1;
            j += 1;
        }
        self
    }

    pub fn no_nonzero_rows(&self) -> bool {
        !self
            .0
            .iter()
            .any(|row| row.into_iter().all(|ele| approx_eq!(f32, 0.0, *ele)))
    }
}

impl<const M: usize> Matrix<f32, M, M> {
    pub fn identity() -> Self {
        // SAFETY: All values are written to
        unsafe {
            let mut identity = Self::uninit();
            for i in 0..M {
                for j in 0..M {
                    identity[(i, j)].write((i == j) as u8 as f32);
                }
            }
            identity.assume_init()
        }
    }
}

impl<T, const M: usize, const N: usize> ops::Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < M);
        assert!(index.1 < N);

        &self.0[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> ops::IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        assert!(index.0 < M);
        assert!(index.1 < N);

        &mut self.0[index.0][index.1]
    }
}

impl<T, const M: usize, const N: usize> From<[Vector<T, M>; N]> for Matrix<T, M, N> {
    fn from(value: [Vector<T, M>; N]) -> Self {
        // SAFETY: All values are written to
        unsafe {
            let mut matrix = Self::uninit();
            for (j, Vector(col)) in value.into_iter().enumerate() {
                for (i, ele) in col.into_iter().enumerate() {
                    matrix[(i, j)].write(ele);
                }
            }
            matrix.assume_init()
        }
    }
}

impl<T, Rhs, const M: usize, const N: usize> ops::Mul<Vector<Rhs, N>> for Matrix<T, M, N>
where
    T: ops::Mul<Rhs>,
    Rhs: Copy,
    <T as ops::Mul<Rhs>>::Output: iter::Sum,
{
    type Output = Vector<<T as ops::Mul<Rhs>>::Output, M>;

    fn mul(self, rhs: Vector<Rhs, N>) -> Self::Output {
        // SAFETY: All values are written to
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, row) in self.0.into_iter().enumerate() {
                new.0[i].write(Vector::<T, N>::from(row).dot(rhs));
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const M: usize, const N: usize, const O: usize> ops::Mul<Matrix<Rhs, N, O>>
    for Matrix<T, M, N>
where
    T: Copy + ops::Mul<Rhs>,
    Rhs: Copy,
    <T as ops::Mul<Rhs>>::Output: iter::Sum,
{
    type Output = Matrix<<T as ops::Mul<Rhs>>::Output, M, O>;

    fn mul(self, rhs: Matrix<Rhs, N, O>) -> Self::Output {
        // SAFETY: All values are written to
        unsafe {
            let mut new = Self::Output::uninit();
            for i in 0..M {
                for j in 0..O {
                    new[(i, j)].write((0..N).map(|k| self[(i, k)] * rhs[(k, j)]).sum());
                }
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const M: usize> ops::MulAssign<Matrix<Rhs, M, M>> for Matrix<T, M, M>
where
    T: Copy + ops::Mul<Rhs, Output = T> + std::iter::Sum,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Matrix<Rhs, M, M>) {
        *self = self.clone() * rhs;
    }
}

impl<T, Rhs, const M: usize, const N: usize> ops::Div<Rhs> for Matrix<T, M, N>
where
    T: ops::Div<Rhs>,
    Rhs: Copy,
{
    type Output = Matrix<<T as ops::Div<Rhs>>::Output, M, N>;

    fn div(self, rhs: Rhs) -> Self::Output {
        // SAFETY: All values are written to
        unsafe {
            let mut new = Self::Output::uninit();
            for (i, row) in self.0.into_iter().enumerate() {
                for (j, ele) in row.into_iter().enumerate() {
                    new[(i, j)].write(ele / rhs);
                }
            }
            new.assume_init()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rref() {
        let mat = Matrix([
            [1.0, 3.0, 1.0, 9.0],
            [1.0, 1.0, -1.0, 1.0],
            [3.0, 11.0, 5.0, 35.0],
        ]);

        let rref = Matrix([
            [1.0, 0.0, -2.0, -3.0],
            [0.0, 1.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(mat.rref(), rref);
    }

    #[test]
    fn no_nonzero_rows() {
        let mat = Matrix([
            [1.0, 3.0, 1.0, 9.0],
            [1.0, 1.0, -1.0, 1.0],
            [3.0, 11.0, 5.0, 35.0],
        ]);

        let rref = Matrix([
            [1.0, 0.0, -2.0, -3.0],
            [0.0, 1.0, 1.0, 4.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert!(mat.no_nonzero_rows());
        assert!(!rref.no_nonzero_rows());
    }
}
