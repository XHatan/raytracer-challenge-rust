use crate::tuple::{Tuple, TupleProperties};
use crate::ray::Ray;
use crate::transformation::{Transform, TransformProperty};

#[derive(Clone)]
pub struct Sphere {
    origin: Tuple,
    radius: f64,
    transform: Transform
}

impl Sphere {
    pub(crate) fn new(origin: Tuple, radius: f64) -> Sphere {
        Sphere {origin, radius, transform: Transform::new()}
    }

    fn radius(&self) -> f64 {
        self.radius
    }

    fn origin(&self) -> Tuple {
        self.origin.clone()
    }

    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    pub(crate) fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }
}

pub fn intersect(s: Sphere, r: Ray) -> Vec<f64> {
    // line o + t * dir = x
    // sphere (x - o') ^ 2 = r^2
    // ((o-o')^2 + t^2 v^2 + 2 * (o - o') * t * dir = r^2
    let origin = s.origin();
    let radius = s.radius();
    let transform = s.transform();
    // TODO: need to apply the inverse and refactor reverse
    let r_t = transform * r;
    let sphere_to_ray = r_t.origin() - origin;
    let a = r_t.direction().dot(r_t.direction()); // dir^2
    let b = 2.0 * r_t.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
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

pub fn hit(ts: Vec<f64>) -> f64 {
    let mut ts_new = ts.clone();
    for val in ts_new {
        if val > 0.0 {
            return val;
        }
    }

    -1.0
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

    #[test]
    fn sphere_has_transform() {
        let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let mut transform = s.transform();
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.transform = transform.translate(2.0, 3.0, 4.0);
        let new_transform = s.transform();
        assert_eq!(f64::abs(new_transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(new_transform[(0, 3)] - 2.0) < 0.001, true);
        assert_eq!(f64::abs(new_transform[(1, 3)] - 3.0) < 0.001, true);
        assert_eq!(f64::abs(new_transform[(2, 3)] - 4.0) < 0.001, true);
    }

    #[test]
    fn sphere_scale_intersect() {
        let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let mut transform = s.transform();
        let r = Ray::new(Tuple::new(0.0, 0.0, -5.0, 1.0), Tuple::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.set_transform(transform.scaling(1.0/2.0, 1.0/2.0, 1.0/2.0));
        let res = intersect(s, r);
        assert_eq!(res.len(), 2);
        assert_eq!(f64::abs(res[0]), 3.0);
        assert_eq!(f64::abs(res[1] - 7.0) < 0.01, true);
    }

    #[test]
    fn translated_ray_with_a_ray() {
        let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
        let mut transform = s.transform();
        let r = Ray::new(Tuple::new(0.0, 0.0, -5.0, 1.0), Tuple::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.transform = transform.translate(-5.0, 0.0, 0.0);
        let new_transform = s.transform();
        let res = intersect(s, r);
        assert_eq!(res.len(), 0);
    }
}