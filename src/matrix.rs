use nalgebra;

pub struct Matrix {
    pub data: nalgebra::DMatrix<f64>
}

pub trait MatrixProperties {
    fn dot(&self, m2: Matrix) -> Matrix;
    fn transpose(&self) -> Matrix;
    fn inverse(&self) -> Matrix;
}

impl MatrixProperties for Matrix {
    fn dot(&self, m2: Matrix) -> Matrix {
        Matrix {data: self.data.clone() * m2.data}
    }

    fn transpose(&self) -> Matrix {
        Matrix {data: self.data.transpose()}
    }

    fn inverse(&self) -> Matrix {
        let data = self.data.clone().try_inverse().unwrap();
        Matrix {data }
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_matrix_index() {
        let mut matrix = Matrix {data: DMatrix::zeros(2, 2)};
        matrix[(0, 0)] = 1.0;
        let mut m2 = Matrix {data: DMatrix::zeros(2, 2)};
        m2[(0, 0)] = 1.0;
        m2[(1, 1)] = 1.0;
        let m3 = m2.dot(matrix);
        assert_eq!(f64::abs(m3[(0, 0)] - 1.0) < 0.001, true);
    }

    #[test]
    fn test_matrix_multiply_matrix() {
        let mut matrix = Matrix {data: DMatrix::zeros(2, 2)};
        matrix[(0, 0)] = 1.0;
        let mut m2 = Matrix {data: DMatrix::zeros(2, 2)};
        m2[(0, 0)] = 1.0;
        m2[(1, 1)] = 1.0;
        let m3 = m2.dot(matrix);
        assert_eq!(f64::abs(m3[(0, 0)] - 1.0) < 0.001, true);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut matrix = Matrix { data: DMatrix::zeros(3, 2) };
        assert_eq!(matrix.data.shape(), (3, 2));
        matrix.data[(0, 0)] = 1.0;
        let m2 = matrix.transpose();
        assert_eq!(m2.data.shape(), (2, 3));
    }

    #[test]
    fn test_matrix_inverse() {
        let matrix = Matrix {
            data: DMatrix::<f64>::identity(2, 2)
        };

        let inv: Matrix = matrix.inverse();
        assert!(f64::abs(inv[(0, 0)] - 1.0) < 0.01);
    }

    #[test]
    fn test_matrix_inverse_2() {
        let mut matrix = Matrix {
            data: DMatrix::<f64>::identity(2, 2)
        };
        matrix[(0, 1)] = 3.0;
        matrix[(1, 1)] = 5.0;
        let inv: Matrix = matrix.inverse();
        let product = matrix.dot(inv);
        assert!(f64::abs(product[(0, 0)] - 1.0) < 0.01);
    }


}
