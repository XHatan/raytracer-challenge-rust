use crate::*;
use crate::shape_props::ShapeProperties;
use crate::intersection::Intersection;
use std::rc::Rc;

#[derive(Clone)]
pub struct Plane {
    pub material: Material,
    pub transform: Transform,
}

impl ShapeProperties for Plane {
    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, p: Point) -> Vector {
        return Vector::new(0.0, 1.0, 0.0);
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let r_t = self.transform.inverse() * ray;

        match r_t.direction().y().abs() > 0.0001 {
            true => vec![
                Intersection {
                    t: -r_t.origin().y() / r_t.direction().y(),
                    object: self.as_trait()
                }
            ],
            false => vec![]
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


pub fn plane() -> Plane {
    plane::Plane {
        transform: Transform::new(),
        material: Material::default()
    }
}

mod tests {

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = plane();
        let n1 = p.normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.normal_at(Point::new(10.0, 0.0, -10.0));
        let n3 = p.normal_at(Point::new(-5.0, 0.0, 150.0));
        let true_v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(n1 == true_v, true);
        assert_eq!(n2 == true_v, true);
        assert_eq!(n3 == true_v, true);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plan() {
        let p = plane();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = plane();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = plane();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(f64::abs(xs[0].t - 1.0) < 0.001, true);
    }

    #[test]
    fn a_ray_intersecting_a_plan_from_below() {
        let p = plane();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(f64::abs(xs[0].t - 1.0) < 0.001, true);
    }
}