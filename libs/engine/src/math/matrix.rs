use super::Vector;

use std::mem::{self, MaybeUninit};
use std::ops;

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

impl<const M: usize, const N: usize> ops::Mul<Vector<f32, N>> for Matrix<f32, M, N> {
    type Output = Vector<f32, M>;

    fn mul(self, rhs: Vector<f32, N>) -> Self::Output {
        unsafe {
            let mut new = Self::Output::uninit();
            for i in 0..M {
                new.0[i].write((0..N).map(|k| self.0[i][k] * rhs.0[k]).sum());
            }
            new.assume_init()
        }
    }
}

impl<T, Rhs, const M: usize, const N: usize, const O: usize> ops::Mul<Matrix<Rhs, N, O>>
    for Matrix<T, M, N>
where
    T: ops::Mul<Rhs> + Copy,
    Rhs: Copy,
    <T as ops::Mul<Rhs>>::Output: std::iter::Sum,
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
    T: ops::Mul<Rhs, Output = T> + Copy + std::iter::Sum,
    Rhs: Copy,
{
    fn mul_assign(&mut self, rhs: Matrix<Rhs, M, M>) {
        unsafe {
            let mut new = Self::uninit();
            for i in 0..M {
                for j in 0..M {
                    new[(i, j)].write((0..M).map(|k| self[(i, k)] * rhs[(k, j)]).sum());
                }
            }
            *self = new.assume_init();
        }
    }
}

impl<const M: usize, const N: usize> ops::Div<f32> for Matrix<f32, M, N> {
    type Output = Matrix<f32, M, N>;

    fn div(mut self, rhs: f32) -> Self::Output {
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] /= rhs;
            }
        }
        self
    }
}
