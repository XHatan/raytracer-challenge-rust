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

// type supports copy will copy-by-default, otherwise move by default
#[derive(Copy, Clone)]
pub struct Point {
    pub data: Tuple
}

#[derive(Copy, Clone)]
pub struct Vector {
    pub data: Tuple
}

pub trait PointProperties {
    fn new(x: f64, y: f64, z: f64) -> Point;

    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn z(&self) -> f64;
}

impl PointProperties for Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {data: Tuple::new(x, y, z, 1.0)}
    }

    fn x(&self) -> f64 {
        self.data.x
    }

    fn y(&self) -> f64 {
        self.data.y
    }

    fn z(&self) -> f64 {
        self.data.z
    }
}

pub trait VectorProperties {
    fn new(x: f64, y: f64, z: f64) -> Vector;

    fn dot(&self, rhs: Vector) -> f64;

    fn cross(&self, rhs: Vector) -> Vector;

    fn neg(&self) -> Vector;

    fn mag(&self) -> f64;

    fn normalize(&self) -> Vector;

    fn hadamard_product(&self, rhs: Vector) -> Vector;

    fn reflect(&self, rhs: Vector) -> Vector;

    fn to_matrix(&self) -> Matrix;

    fn x(&self) -> f64;

    fn y(&self) -> f64;

    fn z(&self) -> f64;

}

impl VectorProperties for Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {data: Tuple::new(x, y, z, 0.0)}
    }

    fn dot(&self, rhs: Vector) -> f64 {
        self.data.dot(rhs.data)
    }

    fn cross(&self, rhs: Vector) -> Vector {
       Vector{data: self.data.cross(rhs.data)}
    }

    fn neg(&self) -> Vector {
        Vector {data: self.data.neg()}
    }

    fn mag(&self) -> f64 {
        self.data.mag()
    }

    // TODO: XHATAN throw errors
    fn normalize(&self) -> Vector {
        Vector {data: self.data.normalize()}
    }

    fn hadamard_product(&self, rhs: Vector) -> Vector {
        Vector {data: self.data.hadamard_product(rhs.data)}
    }

    fn reflect(&self, rhs: Vector) -> Vector {
        Vector {data: self.data.reflect(rhs.data)}
    }

    fn to_matrix(&self) -> Matrix {
        self.data.to_matrix()
    }

    fn x(&self) -> f64 {
        self.data.x
    }

    fn y(&self) -> f64 {
        self.data.y
    }

    fn z(&self) -> f64 {
        self.data.z
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

    fn reflect(&self, rhs: Tuple) -> Tuple;

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

    fn reflect(&self, rhs: Tuple) -> Tuple {
        let input = self.clone();
        input - rhs * 2.0 * self.dot(rhs)
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

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector {data: self.data - rhs.data}
    }
}

impl ops::Mul<Tuple> for f64 {
    type  Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::new(rhs.x * self, rhs.y * self, rhs.z * self , rhs.w * self)
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

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        return if f64::abs(self.x - other.x) < 0.001 && f64::abs(self.y - other.y) < 0.001 && f64::abs(self.z - other.z) < 0.001 &&
            f64::abs(self.w - other.w) < 0.001 {
            true
        } else {
            false
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {data: self.data * rhs}
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {data: self * rhs.data}
    }
}


impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point {data: self.data + rhs.data}
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {data: self.data + rhs.data}
    }
}

impl PartialEq<Vector> for Vector {
    fn eq(&self, other: &Vector) -> bool {
        return self.data == other.data;
    }
}

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        return self.data == other.data;
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
    }

    #[test]
    fn test_tuple_add() {
        let vector = Vector::new(2.0, 3.0, 4.0);
        let vector2 = Point::new(2.1, 3.0, 4.4);

        let vector3 = vector + vector2;
        assert_eq!(vector3.x(), 4.1);
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

    #[test]
    fn test_reflection() {
        let v = Tuple::new(1.0, -1.0, 0.0, 0.0);
        let normal = Tuple::new(0.0, 1.0, 0.0, 0.0);
        let v = v.reflect(normal);
        assert_eq!(f64::abs(v.x - 1.0) < 0.001, true);
        assert_eq!(f64::abs(v.y - 1.0) < 0.001, true);
        assert_eq!(f64::abs(v.z) < 0.001, true);
        assert_eq!(f64::abs(v.w) < 0.001, true);
    }

}