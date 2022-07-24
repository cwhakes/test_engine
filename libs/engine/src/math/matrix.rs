use super::Vector;

use std::ops;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

impl<const M: usize, const N: usize> Matrix<f32, M, N> {
    pub fn zero() -> Self {
        Self([[0.0; N]; M])
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
        let mut new = Self::Output::zero();
        for i in 0..M {
            new.0[i] = (0..N).map(|k| self.0[i][k] * rhs.0[k]).sum();
        }
        new
    }
}

impl<const M: usize, const N: usize, const O: usize> ops::Mul<Matrix<f32, N, O>>
    for Matrix<f32, M, N>
{
    type Output = Matrix<f32, M, O>;

    fn mul(self, rhs: Matrix<f32, N, O>) -> Self::Output {
        let mut new = Self::Output::zero();
        for i in 0..M {
            for j in 0..O {
                new[(i, j)] = (0..N).map(|k| self[(i, k)] * rhs[(k, j)]).sum();
            }
        }
        new
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
