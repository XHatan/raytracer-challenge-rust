use crate::tuple::{Tuple, TupleProperties};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Sphere {
    origin: Tuple,
    radius: f64
}

impl Sphere {
    fn new(origin: Tuple, radius: f64) -> Sphere {
        Sphere {origin, radius}
    }

    fn radius(&self) -> f64 {
        self.radius
    }

    fn origin(&self) -> Tuple {
        self.origin.clone()
    }

}

pub fn intersect(s: Sphere, r: Ray) -> Vec<f64> {
    // line o + t * dir = x
    // sphere (x - o') ^ 2 = r^2
    // ((o-o')^2 + t^2 v^2 + 2 * (o - o') * t * dir = r^2
    let origin = s.origin();
    let radius = s.radius();
    let sphere_to_ray = r.origin() - origin;
    let a = r.direction().dot(r.direction()); // dir^2
    let b = 2.0 * r.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0; // (o - o')^2

    let discriminant = b * b - 4.0 * a * c;
    return if discriminant < 0.0 {
        let result: Vec<f64> = Vec::new();
        result
    } else {
        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
        let result: Vec<f64> = vec![t1, t2];
        result
    }
}

mod tests {
    use super::*;
    use crate::tuple::Point;

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Tuple::new(0.0, 0.0, 1.0, 0.0));
        let s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let count = intersect(s, r);
        assert_eq!(count.len(), 2);
        assert_eq!(f64::abs(count[0] + 6.0) < 0.001, true);
        assert_eq!(f64::abs(count[1] + 4.0) < 0.001, true);
    }
}