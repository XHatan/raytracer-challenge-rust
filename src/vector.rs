use std::ops;
use crate::matrix::Matrix;
use nalgebra::DMatrix;

#[derive(Copy, Clone)]
pub struct TupleData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl TupleData {
    fn new(x: f64, y: f64, z: f64, w: f64) -> TupleData {
        TupleData {x, y, z, w}
    }
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn dot(&self, rhs: TupleData) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    fn cross(&self, rhs: TupleData) -> TupleData {
        // only 3d vector
        TupleData {x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x, w: 0.0}
    }

    fn neg(&self) -> TupleData {
        TupleData::new(-self.x, -self.y, -self.z, -self.w)
    }

    fn mag(&self) -> f64 {
        let val = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        val.sqrt()
    }

    fn normalize(&self) -> TupleData {
        let norm = self.mag();
        TupleData::new(self.x / norm, self.y / norm, self.z / norm, self.w / norm)
    }

    fn hadamard_product(&self, rhs: TupleData) -> TupleData {
        TupleData::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
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

#[derive(Copy, Clone)]
pub struct Tuple {
    data: TupleData
}

#[derive(Copy, Clone)]
pub struct Vector {
    data: TupleData
}

#[derive(Copy, Clone)]
pub struct Point {
    data: TupleData
}

pub trait TupleDataProperties {
    fn data(&self) -> TupleData;
}

// for vector / point
trait TupleProperties {
    fn is_point(&self) -> bool;

    fn dot(&self, rhs: Box<dyn TupleDataProperties>) -> f64;

    // fn cross(&self, rhs: Tuple) -> Tuple;
    //
    // fn neg(&self) -> Tuple;
    //
    // fn mag(&self) -> f64;
    //
    // fn normalize(&self) -> Tuple;
    //
    // fn hadamard_product(&self, rhs: Tuple) -> Tuple;
    //
    // fn to_matrix(&self) -> Matrix;
}


impl<T> TupleProperties for T where T: TupleDataProperties {
    fn is_point(&self) -> bool {
        self.data().is_point()
    }

    fn dot(&self, rhs: Box<dyn TupleDataProperties>) -> f64 {
        self.data().dot(rhs.data())
    }

    // fn cross(&self, rhs: T) -> Tuple {
    //     unimplemented!()
    // }
    //
    // fn neg(&self) -> T {
    //     unimplemented!()
    // }
    //
    // fn mag(&self) -> f64 {
    //     self.data().mag()
    // }
    //
    // fn normalize(&self) -> T {
    //     self.data().normalize()
    // }
    //
    // fn hadamard_product(&self, rhs: Tuple) -> Tuple {
    //     unimplemented!()
    // }
    //
    // fn to_matrix(&self) -> Matrix {
    //     self.data().to_matrix()
    // }
}

impl TupleDataProperties for Point {
    fn data(&self) -> TupleData {
        self.data.clone()
    }
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {data: TupleData{x, y, z, w: 1.0 }}
    }
}


impl TupleDataProperties for Vector {
    fn data(&self) -> TupleData {
        self.data.clone()
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {data: TupleData{x, y, z, w: 0.0 }}
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_point_construction() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.is_point(), true);
    }

    #[test]
    fn test_vector_construction() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.is_point(), false);
    }

    // #[test]
    // fn test_vector_dot_product() {
    //     let v = Vector::new(1.0, 2.0, 3.0);
    //     let v2 = Vector::new(2.0, 3.0, 1.0);
    //
    //     let inner_product = v.dot(v2);
    //
    // }
}