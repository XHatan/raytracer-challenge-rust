
// pub struct Object {}

use crate::sphere::Sphere;

pub struct Intersection {
    t: f64,
    object: Sphere
}

pub struct Intersections {
    data: Vec<Intersection>
}

