use super::Matrix;

pub type Matrix2x2 = Matrix<f32, 2, 2>;

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::assert_approx_eq;

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
}
