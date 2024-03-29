use crate::matrix::{Matrix, MatrixProperties};
use nalgebra;
use crate::tuple::{Tuple, TupleProperties, Vector, Point, VectorProperties, PointProperties};
use crate::ray::Ray;

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

impl std::ops::IndexMut<(usize, usize)> for Transform {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl std::ops::Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        let data = self.dot(rhs.data);
        Vector {data}
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        let internal_matrix = self.matrix.data.clone() * rhs.matrix.data.clone();
        Transform { matrix: Matrix{ data: internal_matrix}}
    }
}

impl std::ops::Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        let data = self.dot(rhs.data);
        Point {data}
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
        let direction = Vector {data: self.dot(rhs.direction().data) };
        let origin = Point {data: self.dot(rhs.origin().data)};
        Ray::new(origin, direction)
    }
}

// impl  C

impl std::ops::Mul<&Ray> for Transform {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Ray {
        let direction = Vector {data: self.dot(rhs.direction().data) };
        let origin = Point {data: self.dot(rhs.origin().data)};
        Ray::new(origin, direction)
    }
}

impl PartialEq<Transform> for Transform {
    fn eq(&self, other: &Transform) -> bool {
        self.matrix == other.matrix
    }
}



pub fn ViewTransform(from: Point, to: Point, up: Vector) -> Transform {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);

    let mut transform = Transform::new();
    transform[(0, 0)] = left.x();
    transform[(0, 1)] = left.y();
    transform[(0, 2)] = left.z();
    transform[(1, 0)] = true_up.x();
    transform[(1, 1)] = true_up.y();
    transform[(1, 2)] = true_up.z();
    transform[(2, 0)] = -forward.x();
    transform[(2, 1)] = -forward.y();
    transform[(2, 2)] = -forward.z();

    let translation = Transform::new().translate(-from.x(), -from.y(), -from.z());

    transform * translation
}




impl Clone for Transform {
    fn clone(&self) -> Self {
        Transform {matrix: Matrix {data: self.matrix.data.clone()}}
    }
}



mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::tuple::{Point, PointProperties};

    #[test]
    fn test_transform_construction() {
        let t = Transform::new();
        assert_eq!(f64::abs(t[(0, 0)] - 1.0) < 0.001, true);
    }

    #[test]
    fn test_transform_rotate_x() {
        let mut t = Transform::new();
        assert_eq!(f64::abs(t[(0, 0)] - 1.0) < 0.001, true);
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
        // assert_eq!(f64::abs(p2.x - 6.0) < 0.001, true);
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

        // let p2 = A * p;
        // assert_eq!(f64::abs(p2.data.x - 1.0) < 0.001, true);
        // let p3 = B * p2;
        // assert_eq!(f64::abs(p3.x - 5.0) < 0.001, true);
        // let p4 = C * p3;
        // assert_eq!(f64::abs(p4.x - 15.0) < 0.01, true);
    }

    #[test]
    fn test_transform_chained_2() {
        let p = Point::new(1.0, 0.0, 1.0);
        let mut original = Transform::new();
        let mut A = original.rotate_x(PI/2.0).scaling(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0);

        let p2 = A * p;
        // assert_eq!(f64::abs(p2.x - 15.0) < 0.01, true);
    }

    #[test]
    fn test_view_transform_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = ViewTransform(from, to, up);
        let true_t = Transform::new();

        assert_eq!(t[(0, 0)], true_t[(0, 0)]);
        assert_eq!(t[(1, 1)], true_t[(1, 1)]);
        assert_eq!(t[(2, 2)], true_t[(2, 2)]);
    }

    #[test]
    fn test_view_transform_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = ViewTransform(from, to, up);
        let true_t = Transform::new().scaling(-1.0, 1.0, -1.0);

        assert_eq!(t[(0, 0)], true_t[(0, 0)]);
        assert_eq!(t[(1, 1)], true_t[(1, 1)]);
        assert_eq!(t[(2, 2)], true_t[(2, 2)]);
    }

    #[test]
    fn test_view_transform_looking_in_arbitrary_direction() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = ViewTransform(from, to, up);

        assert_eq!(f64::abs(t[(0, 0)] + 0.50709) < 0.001, true);
        assert_eq!(f64::abs(t[(0, 1)] - 0.50709) < 0.001, true);
        assert_eq!(f64::abs(t[(0, 2)] - 0.67612) < 0.001, true);
        assert_eq!(f64::abs(t[(0, 3)] + 2.36643) < 0.001, true);

        assert_eq!(f64::abs(t[(1, 0)] - 0.76772) < 0.001, true);
        assert_eq!(f64::abs(t[(1, 1)] - 0.60609) < 0.001, true);
        assert_eq!(f64::abs(t[(1, 2)] - 0.12122) < 0.001, true);
        assert_eq!(f64::abs(t[(1, 3)] + 2.82843) < 0.001, true);

        assert_eq!(f64::abs(t[(2, 0)] + 0.35857) < 0.001, true);
        assert_eq!(f64::abs(t[(2, 1)] - 0.59761) < 0.001, true);
        assert_eq!(f64::abs(t[(2, 2)] + 0.71714) < 0.001, true);
        assert_eq!(f64::abs(t[(2, 3)]) < 0.001, true);
    }
}