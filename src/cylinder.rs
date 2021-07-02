use crate::*;
use crate::shape_props::ShapeProperties;
use crate::intersection::Intersection;
use std::rc::Rc;
use std::mem::swap;


#[derive(Clone)]
pub struct Cylinder {
    pub material: Material,
    pub transform: Transform,
    pub cylinder_minimum: f64,
    pub cylinder_maximum: f64,
    pub cylinder_closed: bool,
}

impl ShapeProperties for Cylinder {
    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.transform.inverse() * p;
        let mut normal_obj_space: Vector;
        let max = self.cylinder_maximum;
        let min = self.cylinder_minimum;
        let dist = local_point.x() * local_point.x() + local_point.z() * local_point.z();

        if dist <= 1.0 && local_point.y() >= max - f64::EPSILON  {
            normal_obj_space =  Vector::new(0.0, 1.0, 0.0);
        } else if dist <= 1.0 && local_point.y() <= min + f64::EPSILON {
            normal_obj_space =  Vector::new(0.0, -1.0, 0.0);
        } else {
            normal_obj_space =  Vector::new(local_point.x(), 0.0, local_point.z());
        }

        let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
        world_normal.data.w = 0.0;
        return world_normal.normalize();
    }

    fn intersect(&self, ray_world: &Ray) -> Vec<Intersection> {
        let ray_obj = self.transform.inverse() * ray_world;
        let min = self.cylinder_minimum;
        let max = self.cylinder_maximum;
        let closed = self.cylinder_closed;

        let mut result: Vec<Intersection> = vec![];

        // check capped
        if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
            let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
            if shape_props::check_cap(&ray_obj, t) {
                result.push(Intersection { t, object: self.as_trait() });
            }

            let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
            if shape_props::check_cap(&ray_obj, t) {
                result.push(Intersection {t, object: self.as_trait()});
            }
        }

        // point = (t * dir + origin)
        // p2 = point(x, z)
        // dir^2 + origin^2 + 2 * t * dir * origin
        let a = f64::powf(ray_obj.direction().x(), 2.0)
            + f64::powf(ray_obj.direction().z(), 2.0);

        if f64::abs(a) < 0.00001 {
            return result;
        }

        let b = 2.0 * ray_obj.origin().x() * ray_obj.direction().x() + 2.0 * ray_obj.origin().z() * ray_obj.direction().z();
        let c = f64::powf(ray_obj.origin().x(), 2.0) + f64::powf(ray_obj.origin().z(), 2.0) - 1.0;
        let disc = f64::powf(b, 2.0) - 4.0 * a * c;

        if disc < 0.0 {
            return result;
        } else {
            let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
            let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }
            let y0 = t0 * ray_obj.direction().y() + ray_obj.origin().y();
            if y0 > min && y0 < max {
                result.push(Intersection { t: t0, object: self.as_trait() });
            }

            let y1 = t1 * ray_obj.direction().y() + ray_obj.origin().y();
            if y1 > min && y1 < max {
                result.push(Intersection { t: t1, object: self.as_trait() });
            }

            return result;
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

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {
            transform: Transform::new(),
            material: Material::default(),
            cylinder_maximum: f64::MAX,
            cylinder_minimum: f64::MIN,
            cylinder_closed: false
        }
    }

    pub fn set_cylinder_truncation(&mut self, min: f64, max: f64) {
        self.cylinder_minimum = min;
        self.cylinder_maximum = max;
    }

    pub fn set_cylinder_closed(&mut self, closed: bool) {
        self.cylinder_closed = closed;
    }
}

pub fn cylinder() -> Cylinder {
    Cylinder::new()
}

mod tests {
    use super::*;


    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = cylinder();
        assert_eq!(cyl.cylinder_minimum, f64::MIN);
        assert_eq!(cyl.cylinder_maximum, f64::MAX);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let mut cyl = cylinder();
        cyl.set_cylinder_truncation(1.0, 2.0);

        let ray = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = cyl.intersect(&ray);

        assert_eq!(xs.len(), 0);

        let ray2 = Ray::new(Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0));
        let xs2 = cyl.intersect(&ray2);
        assert_eq!(xs2.len(), 2);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let mut cyl = cylinder();
        cyl.set_cylinder_truncation(1.0, 2.0);
        cyl.set_cylinder_closed(true);

        let r = Ray::new(Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0).normalize());
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0).normalize());
        let xs = cyl.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn the_normal_vector_on_a_cylinder_end_caps() {
        let mut cyl = cylinder();
        cyl.set_cylinder_truncation(1.0, 2.0);
        cyl.set_cylinder_closed(true);

        let p = Point::new(0.0, 1.0, 0.0);
        let n = cyl.normal_at(p);

        assert_eq!(n == Vector::new(0.0, -1.0, 0.0), true);
    }
}