use super::Vector;

use std::mem::{self, MaybeUninit};
use std::{iter, ops};

use float_cmp::approx_eq;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T, const M: usize, const N: usize>(pub [[T; N]; M]);

pub type Matrix2x2 = Matrix<f32, 2, 2>;
pub type Matrix3x3 = Matrix<f32, 3, 3>;

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
            .any(|row| row.iter().all(|ele| approx_eq!(f32, 0.0, *ele)))
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

    #[inline(always)]
    pub fn determinant(&self) -> f32 {
        match M {
            0 => 1.0,
            1 => self[(0, 0)],
            2 => self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)],
            3 => {
                self[(0, 0)] * self.minor((0, 0)) - self[(0, 1)] * self.minor((0, 1))
                    + self[(0, 2)] * self.minor((0, 2))
            }
            4 => {
                self[(0, 0)] * self.minor((0, 0)) - self[(0, 1)] * self.minor((0, 1))
                    + self[(0, 2)] * self.minor((0, 2))
                    - self[(0, 3)] * self.minor((0, 3))
            }
            _ => self.determinant_slow(),
        }
    }

    fn determinant_slow(&self) -> f32 {
        let mut mat = self.clone();
        let mut i = 0;
        let mut d = 1.0;

        'outer: while i < M {
            if approx_eq!(f32, 0.0, mat[(i, i)]) {
                // Search for non-zero row
                for i_p in (i + 1)..M {
                    if !approx_eq!(f32, 0.0, mat[(i_p, i)]) {
                        mat.0.swap(i, i_p);
                        d *= -1.0;
                        continue 'outer;
                    }
                }
                // If we can't find a non-zero element in this column, return zero
                return 0.0;
            }

            // reduce other rows
            for i_p in (i + 1)..M {
                let f = mat[(i_p, i)] / mat[(i, i)];
                for j_p in 0..M {
                    mat[(i_p, j_p)] -= f * mat[(i, j_p)];
                }
            }

            // advance pivot
            i += 1;
        }
        (0..M).map(|i| mat[(i, i)]).product::<f32>() / d
    }

    #[inline(always)]
    pub fn minor(&self, (i, j): (usize, usize)) -> f32 {
        assert!(i < M);
        assert!(j < M);

        match M {
            0 => unreachable!(),
            1 => 1.0,
            2 => self[(1 - i, 1 - j)],
            3 => {
                let [i0, i1] = [0, 1].map(|ix| ix + (ix >= i) as usize);
                let [j0, j1] = [0, 1].map(|jx| jx + (jx >= j) as usize);
                self[(i0, j0)] * self[(i1, j1)] - self[(i0, j1)] * self[(i1, j0)]
            }
            4 => {
                let [i0, i1, i2] = [0, 1, 2].map(|ix| ix + (ix >= i) as usize);
                let [j0, j1, j2] = [0, 1, 2].map(|jx| jx + (jx >= j) as usize);
                let a = self; // Make things shorter
                (a[(i0, j0)] * (a[(i1, j1)] * a[(i2, j2)] - a[(i1, j2)] * a[(i2, j1)]))
                    - (a[(i0, j1)] * (a[(i1, j0)] * a[(i2, j2)] - a[(i1, j2)] * a[(i2, j0)]))
                    + (a[(i0, j2)] * (a[(i1, j0)] * a[(i2, j1)] - a[(i1, j1)] * a[(i2, j0)]))
            }
            _ => self.minor_slow((i, j)),
        }
    }

    fn minor_slow(&self, (i, j): (usize, usize)) -> f32 {
        let mut mat = self.clone();
        let mut ii = 0;
        let mut d = 1.0;

        'outer: while ii < M - 1 {
            let iii = ii + (ii >= i) as usize;
            let jjj = ii + (ii >= j) as usize;

            if approx_eq!(f32, 0.0, mat[(iii, jjj)]) {
                // Search for non-zero row
                for i_p in ((iii + 1)..M).filter(|&i_p| i_p != i) {
                    if !approx_eq!(f32, 0.0, mat[(i_p, jjj)]) {
                        mat.0.swap(iii, i_p);
                        d *= -1.0;
                        continue 'outer;
                    }
                }
                // If we can't find a non-zero element in this column, return zero
                return 0.0;
            }

            // reduce other rows
            for i_p in ((iii + 1)..M).filter(|&i_p| i_p != i) {
                let f = mat[(i_p, jjj)] / mat[(iii, jjj)];
                for j_p in (0..M).filter(|&j_p| j_p != j) {
                    mat[(i_p, j_p)] -= f * mat[(iii, j_p)];
                }
            }

            // advance pivot
            ii += 1;
        }
        (0..(M - 1))
            .map(|ii| mat[(ii + (ii >= i) as usize, ii + (ii >= j) as usize)])
            .product::<f32>()
            / d
    }

    pub fn adjugate(&self) -> Self {
        // SAFETY: All values are written to
        unsafe {
            let mut adjugate = Self::uninit();
            for i in 0..M {
                for j in 0..M {
                    let sign = (-1.0f32).powi((i + j) as i32);
                    adjugate[(j, i)].write(sign * self.minor((i, j)));
                }
            }
            adjugate.assume_init()
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.is_nan() || approx_eq!(f32, 0.0, det) {
            return None;
        }
        Some(self.adjugate() / det)
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
    use float_cmp::assert_approx_eq;

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

    #[test]
    fn invert_2x2() {
        let a = Matrix([[5.0, 2.0], [-7.0, -3.0]]);
        let b = Matrix([[3.0, 2.0], [-7.0, -5.0]]);

        let a_inv = a.inverse().unwrap();
        let b_inv = b.inverse().unwrap();

        for i in 0..2 {
            for j in 0..2 {
                assert_approx_eq!(f32, a[(i, j)], b_inv[(i, j)], epsilon = 0.00001);
                assert_approx_eq!(f32, a_inv[(i, j)], b[(i, j)], epsilon = 0.00001);
            }
        }
    }

    #[test]
    fn det_3x3() {
        let a = Matrix([[1.0, 5.0, 3.0], [2.0, 4.0, 7.0], [4.0, 6.0, 2.0]]);
        assert_eq!(a.determinant(), 74.0);
    }

    #[test]
    fn invert_3x3() {
        let a = Matrix([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);
        let b = Matrix([[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]]);

        let a_inv = a.inverse().unwrap();
        let b_inv = b.inverse().unwrap();

        for i in 0..3 {
            for j in 0..3 {
                assert_approx_eq!(f32, a[(i, j)], b_inv[(i, j)], epsilon = 0.00001);
                assert_approx_eq!(f32, a_inv[(i, j)], b[(i, j)], epsilon = 0.00001);
            }
        }
    }
}
