use crate::matrix::{Matrix, MatrixProperties};
use nalgebra;
use crate::tuple::{Tuple, TupleProperties};
use crate::ray::Ray;
use std::ops::Mul;

pub struct Transform {
    matrix: Matrix,
}

pub trait  TransformProperty {
    fn new() -> Transform;
    fn identity(&mut self) -> Transform;
    fn rotate_x(&mut self, radian: f64) -> Transform;
    fn rotate_y(&mut self, radian: f64) -> Transform;
    fn rotate_z(&mut self, radian: f64) -> Transform;
    fn translate(&self, x: f64, y: f64, z: f64) -> Transform;
    fn scaling(&mut self, x: f64, y: f64, z: f64) -> Transform;
    fn shear(&mut self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Transform;
    fn dot(&self, rhs: Tuple) -> Tuple;
    fn inverse(&self) -> Transform;
    fn transpose(&self) -> Transform;
}

impl TransformProperty for Transform {
    fn new() -> Transform {
        Transform {matrix: Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4) } }
    }

    fn identity(&mut self) -> Transform {
        self.matrix.data = nalgebra::base::DMatrix::<f64>::identity(4, 4);
        Transform {matrix: Matrix {data: self.matrix.data.clone()}}
    }

    fn rotate_x(&mut self, radian: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(1, 1)] = radian.cos();
        matrix[(1, 2)] = -radian.sin();
        matrix[(2, 1)] = radian.sin();
        matrix[(2, 2)] = radian.cos();

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn rotate_y(&mut self, radian: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(0, 0)] = radian.cos();
        matrix[(0, 2)] = radian.sin();
        matrix[(2, 0)] = -radian.sin();
        matrix[(2, 2)] = radian.cos();

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn rotate_z(&mut self, radian: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(0, 0)] = radian.cos();
        matrix[(0, 1)] = -radian.sin();
        matrix[(1, 0)] = radian.sin();
        matrix[(1, 1)] = radian.cos();

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn translate(&self, x: f64, y: f64, z: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(0, 3)] = x;
        matrix[(1, 3)] = y;
        matrix[(2, 3)] = z;

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn scaling(&mut self, x: f64, y: f64, z: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(0, 0)] = x;
        matrix[(1, 1)] = y;
        matrix[(2, 2)] = z;

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn shear(&mut self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Transform {
        let mut matrix = Matrix {data: nalgebra::base::DMatrix::<f64>::identity(4, 4)};
        matrix[(0, 1)] = x_y;
        matrix[(0, 2)] = x_z;
        matrix[(1, 0)] = y_x;
        matrix[(1, 2)] = y_z;
        matrix[(2, 0)] = z_x;
        matrix[(2, 1)] = z_y;

        // self.matrix.data = matrix.data * self.matrix.data.clone();
        Transform {matrix: Matrix {data: matrix.data * self.matrix.data.clone()}}
    }

    fn dot(&self, rhs: Tuple) -> Tuple {
        let rhs_in_matrix = rhs.to_matrix();
        let m2 = self.matrix.dot(&rhs_in_matrix);
        Tuple::new(m2.data[(0, 0)], m2.data[(1, 0)], m2.data[(2, 0)], m2.data[(3, 0)])
    }

    fn inverse(&self) -> Transform {
        let data = self.matrix.inverse();

        Transform {matrix: data}
    }

    fn transpose(&self) -> Transform {
        let data = self.matrix.transpose();
        Transform {matrix: data}
    }
}

impl std::ops::Index<(usize, usize)> for Transform {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &f64 {
        &self.matrix[index]
    }
}

// impl std::ops::IndexMut<(usize, usize)> for Matrix {
//     fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
//         &mut self.data[index]
//     }
// }


impl Clone for Transform {
    fn clone(&self) -> Self {
        Transform {matrix: Matrix {data: self.matrix.data.clone()}}
    }
}

impl std::ops::Mul<Tuple> for Transform {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let rhs_in_matrix = rhs.to_matrix();
        let m2 = self.matrix.dot(&rhs_in_matrix);
        Tuple::new(m2.data[(0, 0)], m2.data[(1, 0)], m2.data[(2, 0)], m2.data[(3, 0)])
    }
}

impl std::ops::Mul<Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Ray {
        let direction = self.dot(rhs.direction());
        let origin = self.dot(rhs.origin());
        Ray::new(origin, direction)
    }
}


mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::tuple::Point;

    #[test]
    fn test_transform_construction() {
        let t = Transform::new();
        assert_eq!(f64::abs(t.matrix.data[(0, 0)] - 1.0) < 0.001, true);
    }

    #[test]
    fn test_transform_rotate_x() {
        let mut t = Transform::new();
        assert_eq!(f64::abs(t.matrix.data[(0, 0)] - 1.0) < 0.001, true);
        let t2 = t.rotate_x(PI/2.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(t2.matrix.data[(0, 1)]) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(1, 1)]) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(1, 2)] + 1.0) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(2, 1)] - 1.0) < 0.0001, true);
    }

    #[test]
    fn test_transform_rotate_y() {
        let mut t = Transform::new();
        assert_eq!(f64::abs(t.matrix.data[(0, 0)] - 1.0) < 0.001, true);
        let t2 = t.rotate_y(PI/2.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)]) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(0, 2)] - 1.0) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(2, 0)] + 1.0) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(2, 2)]) < 0.0001, true);
    }

    #[test]
    fn test_transform_rotate_z() {
        let mut t = Transform::new();
        assert_eq!(f64::abs(t.matrix.data[(0, 0)] - 1.0) < 0.001, true);
        let t2 = t.rotate_z(PI/2.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)]) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(0, 1)] + 1.0) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(1, 0)] - 1.0) < 0.0001, true);
        assert_eq!(f64::abs(t2.matrix.data[(1, 1)]) < 0.0001, true);
    }

    #[test]
    fn test_transform_translate() {
        let mut t = Transform::new();
        let t2 = t.translate(5.0, -3.0, 2.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(t2.matrix.data[(0, 3)] - 5.0) < 0.001, true);
        let p = Point::new(1.0, 2.0, 3.0);
        let p2 = t2 * p;
        assert_eq!(f64::abs(p2.x - 6.0) < 0.001, true);
    }

    #[test]
    fn test_transform_shear() {
        let mut t = Transform::new();
        let t2 = t.shear(1.0, 2.0, 3.0, 4.0, 1.0, 1.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)] - 1.0) < 0.001, true);
    }

    #[test]
    fn test_transform_scaling() {
        let mut t = Transform::new();
        let t2 = t.scaling(2.5, 2.0, 3.0);
        assert_eq!(f64::abs(t2.matrix.data[(0, 0)] - 2.5) < 0.001, true);
    }

    #[test]
    fn test_transform_chained() {
        let p = Point::new(1.0, 0.0, 1.0);
        let mut original = Transform::new();
        let mut A = original.rotate_x(PI/2.0);
        let mut B = original.scaling(5.0, 5.0, 5.0);
        let mut C = original.translate(10.0, 5.0, 7.0);

        let p2 = A.dot(p);
        assert_eq!(f64::abs(p2.x - 1.0) < 0.001, true);
        let p3 = B.dot(p2);
        assert_eq!(f64::abs(p3.x - 5.0) < 0.001, true);
        let p4 = C.dot(p3);
        assert_eq!(f64::abs(p4.x - 15.0) < 0.01, true);
    }

    #[test]
    fn test_transform_chained_2() {
        let p = Point::new(1.0, 0.0, 1.0);
        let mut original = Transform::new();
        let mut A = original.rotate_x(PI/2.0).scaling(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0);

        let p2 = A.dot(p);
        assert_eq!(f64::abs(p2.x - 15.0) < 0.01, true);
    }
}