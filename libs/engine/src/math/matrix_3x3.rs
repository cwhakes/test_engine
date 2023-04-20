use super::Matrix;

pub type Matrix3x3 = Matrix<f32, 3, 3>;

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
