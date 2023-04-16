use super::{Matrix, Matrix2x2};

pub type Matrix3x3 = Matrix<f32, 3, 3>;

impl Matrix3x3 {
    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self.minor((0, 0)) - self[(0, 1)] * self.minor((0, 1))
            + self[(0, 2)] * self.minor((0, 2))
    }

    pub fn minor(&self, (i, j): (usize, usize)) -> f32 {
        const M: usize = 3;
        // SAFETY: All values are written to
        unsafe {
            let mut minor = Matrix2x2::uninit();
            for ii in 0..(M - 1) {
                for jj in 0..(M - 1) {
                    minor[(ii, jj)].write(self[(ii + (ii >= i) as usize, jj + (jj >= j) as usize)]);
                }
            }
            minor.assume_init().determinant()
        }
    }

    pub fn adjugate(&self) -> Self {
        const M: usize = 3;
        // SAFETY: All values are written to
        unsafe {
            let mut adjugate = Matrix3x3::uninit();
            for i in 0..M {
                for j in 0..M {
                    let sign = (-1.0f32).powi((i + j) as i32);
                    adjugate[(j, i)].write(sign * self.minor((i, j)));
                }
            }
            adjugate.assume_init()
        }
    }

    pub fn inverse(&self) -> Self {
        self.adjugate() / self.determinant()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

    #[test]
    fn det_3x3() {
        let a = Matrix([[1.0, 5.0, 3.0], [2.0, 4.0, 7.0], [4.0, 6.0, 2.0]]);
        assert_eq!(a.determinant(), 74.0);
    }

    #[test]
    fn invert_3x3() {
        let a = Matrix([[1.0, 2.0, 3.0], [0.0, 1.0, 4.0], [5.0, 6.0, 0.0]]);
        let b = Matrix([[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]]);

        let a_inv = a.inverse();
        let b_inv = b.inverse();

        for i in 0..3 {
            for j in 0..3 {
                assert_approx_eq!(f32, a[(i, j)], b_inv[(i, j)], epsilon = 0.00001);
                assert_approx_eq!(f32, a_inv[(i, j)], b[(i, j)], epsilon = 0.00001);
            }
        }
    }
}
