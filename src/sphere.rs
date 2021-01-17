use crate::tuple::{Tuple, TupleProperties, VectorProperties, Point, Vector, PointProperties};
use crate::ray::Ray;
use crate::transformation::{Transform, TransformProperty};
use crate::material::{Material, MaterialProperties};
use crate::intersection::Intersection;

#[derive(Clone)]
pub struct Sphere {
    origin: Point,
    radius: f64,
    transform: Transform,
    material: Material
}

pub trait SphereProperties {
    fn new(origin: Point, radius: f64) -> Sphere;

    fn radius(&self) -> f64;

    fn origin(&self) -> Point;

    fn transform(&self) -> Transform;

    fn set_transform(&mut self, t: Transform);

    fn normal_at(&self, point: Point) -> Vector;

    fn set_material(&mut self, material: Material);

    fn material(&self) -> Material;
}

impl SphereProperties for Sphere {
    fn new(origin: Point, radius: f64) -> Sphere {
        let black = Tuple::new(0.0, 0.0, 0.0, 1.0);
        Sphere {origin, radius,
            transform: Transform::new(),
            material: Material::new(black, 0.1, 0.9, 0.9, 200.0)}
    }

    fn radius(&self) -> f64 {
        self.radius
    }

    fn origin(&self) -> Point {
        self.origin.clone()
    }

    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, point: Point) -> Vector {
        let point_obj_space = self.transform.inverse() * point;
        let normal_obj_space = point_obj_space - self.origin;
        // transforming normal is like transforming plane
        // normal * point = normal^T * point  normal = [a  b  c d], plane ax + by + cz + d = 0
        let mut normal_world = self.transform.inverse().transpose() * normal_obj_space;
        normal_world.data.w = 0.0;

        return normal_world.normalize();
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn material(&self) -> Material {
        self.material.clone()
    }
}
// intersect inverse transform the ray to sphere local coordinates and find intersection point
pub fn intersect_sphere(s: &Sphere, r: Ray) -> Vec<Intersection> {
    // line o + t * dir = x
    // sphere (x - o') ^ 2 = r^2
    // ((o-o')^2 + t^2 v^2 + 2 * (o - o') * t * dir = r^2
    let origin = s.origin();
    let radius = s.radius();
    let transform = s.transform();

    let r_t = transform.inverse() * r;
    let sphere_to_ray = r_t.origin() - origin;
    let a = r_t.direction().dot(r_t.direction()); // dir^2
    let b = 2.0 * r_t.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0; // (o - o')^2

    let discriminant = b * b - 4.0 * a * c;
    return if discriminant < 0.0 {
        let result: Vec<Intersection> = Vec::new();
        result
    } else {
        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
        let i1 = Intersection{t: t1,  object: s.clone() };
        let i2 = Intersection{t: t2, object: s.clone()};
        let result: Vec<Intersection> = vec![i1, i2];
        result
    }
}

pub fn default_sphere() -> Sphere {
    Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0)
}

pub fn hit(ts: Vec<Intersection>) -> Intersection {
    let ts_new = ts.clone();
    for val in ts_new {
        if val.t > 0.0 {
            return val;
        }
    }

    Intersection{t: -1.0, object: Sphere::new(Point::new(0.0, 0.0, 0.0), 0.0)}
}

mod tests {
    use super::*;
    use crate::tuple::{Point, PointProperties, Vector};

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let count = intersect_sphere(&s, r);
        assert_eq!(count.len(), 2);
        assert_eq!(f64::abs(count[0].t + 6.0) < 0.001, true);
        assert_eq!(f64::abs(count[1].t + 4.0) < 0.001, true);
    }

    #[test]
    fn sphere_has_transform() {
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
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
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let mut transform = s.transform();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.set_transform(transform.scaling(2.0, 2.0, 2.0));
        let res = intersect_sphere(&s, r);
        assert_eq!(res.len(), 2);
        assert_eq!(f64::abs(res[0].t), 3.0);
        assert_eq!(f64::abs(res[1].t - 7.0) < 0.01, true);
    }

    #[test]
    fn translated_ray_with_a_ray() {
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let mut transform = s.transform();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.transform = transform.translate(5.0, 0.0, 0.0);
        let new_transform = s.transform();
        let res = intersect_sphere(&s, r);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn sphere_normal_calculation() {
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n == Vector::new(1.0, 0.0, 0.0), true);

        let n2 = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n2 == Vector::new(0.0, 0.0, 1.0), true);
    }

    #[test]
    fn translated_sphere_normal_calculation() {
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let transform = Transform::new();
        let t2 = transform.translate(0.0, 1.0, 0.0);
        s.set_transform(t2);
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n == Vector::new(0.0, 0.70711, -0.70711), true);
    }
}