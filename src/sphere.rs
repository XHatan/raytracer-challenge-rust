use crate::*;
use crate::shape_props::ShapeProperties;
use crate::intersection::Intersection;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub transform: Transform,
    pub material: Material
}

impl ShapeProperties for Sphere {
    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, p: Point) -> Vector {
        let point_obj_space = self.transform.inverse() * p;
        let normal_obj_space = point_obj_space - Point::new(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
        world_normal.data.w = 0.0;
        return world_normal.normalize();
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let origin = Point::new(0.0, 0.0, 0.0);

        // (origin + dir * t).mag == 1
        let r_t = self.transform.inverse() * ray;
        let sphere_to_ray = r_t.origin() - origin;
        let a = r_t.direction().dot(r_t.direction()); // dir^2
        let b = 2.0 * r_t.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0; // (o - o')^2

        let discriminant = b * b - 4.0 * a * c;
        let mut result: Vec<Intersection> = vec![];
        return if discriminant < 0.0 {
            result
        } else {
            let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
            result.push(
                Intersection {
                    t: t1,
                    object: self.as_trait()
                }
            );
            result.push(Intersection {
                t: t2,
                object: self.as_trait()
            });
            result
        }
    }

    fn as_trait(&self) -> &dyn ShapeProperties {
        self
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            transform: Transform::new(),
            material: Material::default()
        }
    }
}

pub fn sphere() -> Sphere {
    Sphere::default()
}

pub fn glass_sphere() -> Sphere {
    let mut s = Sphere::default();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;

    return s;
}

mod tests {
    use super::*;
    use crate::tuple::{Point, PointProperties, Vector};
    use crate::material::float_eq;

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let count = s.intersect(&r);
        assert_eq!(count.len(), 2);
        assert_eq!(f64::abs(count[0].t + 6.0) < 0.001, true);
        assert_eq!(f64::abs(count[1].t + 4.0) < 0.001, true);
    }


    #[test]
    fn sphere_has_transform() {
        let mut s = sphere();
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
        let mut s = sphere();
        let mut transform = s.transform();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.set_transform(transform.scaling(2.0, 2.0, 2.0));
        let res = s.intersect(&r);
        assert_eq!(res.len(), 2);
        assert_eq!(f64::abs(res[0].t), 3.0);
        assert_eq!(f64::abs(res[1].t - 7.0) < 0.01, true);
    }

    #[test]
    fn translated_ray_with_a_ray() {
        let mut s = sphere();
        let mut transform = s.transform();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(f64::abs(transform[(0, 0)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(1, 1)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(2, 2)] - 1.0) < 0.001, true);
        assert_eq!(f64::abs(transform[(3, 3)] - 1.0) < 0.001, true);
        s.transform = transform.translate(5.0, 0.0, 0.0);
        let new_transform = s.transform();
        let res = s.intersect(&r);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn sphere_normal_calculation() {
        let mut s = sphere();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        assert_eq!(n == Vector::new(1.0, 0.0, 0.0), true);

        let n2 = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n2 == Vector::new(0.0, 0.0, 1.0), true);
    }

    #[test]
    fn translated_sphere_normal_calculation() {
        let mut s = sphere();
        let transform = Transform::new();
        let t2 = transform.translate(0.0, 1.0, 0.0);
        s.set_transform(t2);
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));

        assert_eq!(n == Vector::new(0.0, 0.70711, -0.70711), true);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.set_transform(
            Transform::new().scaling(2.0, 2.0, 2.0)
        );

        let xs = s.intersect(&r);
    }
}