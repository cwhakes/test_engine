use super::Matrix;

pub type Matrix2x2 = Matrix<f32, 2, 2>;

impl Matrix2x2 {
    pub fn identity() -> Self {
        Self([[1.0, 0.0], [0.0, 1.0]])
    }

    pub fn inverse(&self) -> Self {
        Self([[self[(1, 1)], -self[(0, 1)]], [-self[(1, 0)], self[(0, 0)]]]) / self.determinant()
    }

    pub fn determinant(&self) -> f32 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invert_2x2() {
        let a = Matrix([[5.0, 2.0], [-7.0, -3.0]]);

        let b = Matrix([[3.0, 2.0], [-7.0, -5.0]]);

        assert_eq!(a, b.inverse());
        assert_eq!(a.inverse(), b);
    }
}
