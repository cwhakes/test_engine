use super::Vector;

use std::mem::{self, MaybeUninit};
use std::{iter, ops};

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

impl<const M: usize, const N: usize> Matrix<f32, M, N> {
    pub fn zero() -> Self {
        Self([[0.0; N]; M])
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
