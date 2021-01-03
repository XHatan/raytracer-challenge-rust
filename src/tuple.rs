use std::ops;
use crate::matrix::Matrix;
use nalgebra::DMatrix;

#[derive(Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple {x, y, z, w}
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub(crate) w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {x, y, z, w: 1.0 }
    }
}

pub trait TupleProperties {
    fn is_point(&self) -> bool;

    fn dot(&self, rhs: Tuple) -> f64;

    fn cross(&self, rhs: Tuple) -> Tuple;

    fn neg(&self) -> Tuple;

    fn mag(&self) -> f64;

    fn normalize(&self) -> Tuple;

    fn hadamard_product(&self, rhs: Tuple) -> Tuple;

    fn to_matrix(&self) -> Matrix;
}

impl TupleProperties for Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn dot(&self, rhs: Tuple) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    fn cross(&self, rhs: Tuple) -> Tuple {
        // only 3d vector
        Tuple::new(self.y * rhs.z - self.z * rhs.y, self.z * rhs.x - self.x * rhs.z, self.x * rhs.y - self.y * rhs.x, 0.0)
    }

    fn neg(&self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }

    fn mag(&self) -> f64 {
        let val = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        val.sqrt()
    }

    fn normalize(&self) -> Tuple {
        let norm = self.mag();
        Tuple::new(self.x / norm, self.y / norm, self.z / norm, self.w / norm)
    }

    fn hadamard_product(&self, rhs: Tuple) -> Tuple {
        Tuple::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }

    fn to_matrix(&self) -> Matrix {
        let mut data = DMatrix::zeros(4, 1);
        data[(0, 0)] = self.x;
        data[(1, 0)] = self.y;
        data[(2, 0)] = self.z;
        data[(3, 0)] = self.w;
        Matrix {data}
    }
}

impl ops::Add for Tuple {
    type  Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple {
        return Tuple::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z, self.w + _rhs.w)
    }
}

impl ops::Sub for Tuple {
    type  Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        return Tuple::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z, self.w - _rhs.w)
    }
}

impl ops::Mul<f64> for Tuple {
    type  Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs , self.w * rhs)
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_can_be_a_point() {
        let vector = Tuple::new(2.0, 3.0, 4.0, 1.0);
        assert_eq!(vector.is_point(), true);

        let vector2 = Tuple::new(2.1, 3.0, 4.4, 0.0);
        assert_eq!(vector2.is_point(), false);

        let point1 = Point::new(1.0, 1.0, 1.0);
        assert_eq!(point1.is_point(), true);
    }

    #[test]
    fn test_tuple_add() {
        let vector = Tuple::new(2.0, 3.0, 4.0, 1.0);
        let vector2 = Tuple::new(2.1, 3.0, 4.4, 0.0);

        let vector3 = vector + vector2;
        assert_eq!(vector3.x, 4.1);

        let point = Point::new(2.0, 1.0, 3.0);
        let p2 = point + vector2;
        assert_eq!(p2.is_point(), true);
    }

    #[test]
    fn test_tuple_subtract() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 1.0);
        let vector2 = Tuple::new(2.1, 3.0, 4.4, 0.0);

        let vector3 = vector - vector2;
        assert_eq!(f64::abs(vector3.x - 0.4) < 0.001, true);
    }

    #[test]
    fn test_tuple_negate() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 1.0);

        let vector3 =  vector.neg();
        assert_eq!(vector3.x, -2.5);
        assert_eq!(vector3.y, -3.0);
        assert_eq!(vector3.z, -4.0);
        assert_eq!(vector3.w, -1.0);
    }

    #[test]
    fn test_tuple_multiply() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 1.0);

        let vector3 =  vector * (3.0 as f64);
        assert_eq!(vector3.x, 7.5);
        assert_eq!(vector3.y, 9.0);
        assert_eq!(vector3.z, 12.0);
        assert_eq!(vector3.w, 3.0);
    }

    #[test]
    fn test_tuple_division() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 1.0);
        let vector3 =  vector / (2.0 as f64);
        assert_eq!(f64::abs(vector3.x - 1.25) < 0.001, true);
    }


    #[test]
    fn test_tuple_magnitude() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 1.0);
        assert_eq!(f64::abs(vector.mag() - 5.678908345800274) < 0.01, true);
    }

    #[test]
    fn test_tuple_dot() {
        let vector = Tuple::new(2.5, 3.0, 4.0, 0.0);
        let vector2 = Tuple::new(1.0, 2.0, 1.0, 0.0);
        assert_eq!(f64::abs(vector.dot(vector2) - 12.5) < 0.01, true);
        assert_eq!(vector2.y, 2.0);
    }

    #[test]
    fn test_tuple_normalize() {
        let vector = Tuple::new(2.0, 2.0, 2.0, 2.0);
        assert_eq!(f64::abs(vector.normalize().x - 0.5) < 0.01, true);
    }

}