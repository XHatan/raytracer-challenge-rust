use crate::*;
use crate::shape_props::ShapeProperties;
use crate::intersection::Intersection;
use std::rc::Rc;
use std::mem::swap;

#[derive(Clone)]
pub struct Cone {
    pub material: Material,
    pub transform: Transform,
    pub cylinder_minimum: f64,
    pub cylinder_maximum: f64,
    pub cylinder_closed: bool,
}

impl ShapeProperties for Cone {
    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, p: Point) -> Vector {
        let local_point = self.transform.inverse() * p;
        let mut normal_obj_space: Vector;
        let dist2 = local_point.x() * local_point.x() + local_point.z() * local_point.z();
        let dist = f64::sqrt(dist2);
        let y_abs = f64::abs(local_point.y());

        let max = self.cylinder_maximum;
        let min = self.cylinder_minimum;
        if y_abs <= f64::EPSILON {
            normal_obj_space =  Vector::new(0.0, 0.0, 0.0);
        } else if dist <= y_abs && local_point.y() >= max - f64::EPSILON  {
            normal_obj_space =  Vector::new(0.0, 1.0, 0.0);
        } else if dist <= y_abs && local_point.y() <= min + f64::EPSILON {
            normal_obj_space = Vector::new(0.0, -1.0, 0.0);
        }  else if local_point.y() > 0.0 {
            normal_obj_space = Vector::new(local_point.x(), -dist, local_point.z())
        } else {
            normal_obj_space = Vector::new(local_point.x(), dist, local_point.z())
        }

        let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
        world_normal.data.w = 0.0;
        return world_normal.normalize();
    }

    fn intersect(&self, ray_world: &Ray) -> Vec<Intersection> {
        let ray_obj = self.transform.inverse() * ray_world;
        let closed = self.cylinder_closed;
        let min = self.cylinder_minimum;
        let max = self.cylinder_maximum;
        let mut result: Vec<Intersection> = vec![];

        // check capped
        if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
            let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
            if shape_props::check_cap_cone(&ray_obj, t, min) {
                result.push(
                    Intersection {
                        t,
                        object: self.as_trait()
                    }
                );
            }

            let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
            if shape_props::check_cap_cone(&ray_obj, t, max) {
                result.push(
                    Intersection {
                    t,
                        object: self.as_trait()
                });
            }
        }

        let dx = ray_obj.direction().x();
        let dy = ray_obj.direction().y();
        let dz = ray_obj.direction().z();

        let ox = ray_obj.origin().x();
        let oy = ray_obj.origin().y();
        let oz = ray_obj.origin().z();


        let a = dx * dx - dy * dy + dz * dz;
        let b = 2.0 * ox * dx - 2.0 * oy * dy + 2.0 * oz * dz;
        let c = ox * ox - oy * oy + oz * oz;

        if f64::abs(a) <= f64::EPSILON && f64::abs(b) <= f64::EPSILON {
            return result;
        }

        if f64::abs(a) <= f64::EPSILON {
            let t = -c / (2.0 * b);
            let y = t * dy + oy;
            if y > min && y < max {
                result.push(
                    Intersection {
                        t,
                        object: self.as_trait()
                    }
                );
            }
        }

        let disc = b * b  - 4.0 * a * c;

        if disc >= 0.0 {
            let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
            let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }
            let y0 = t0 * dy + oy;
            if y0 > min && y0 < max {
                result.push(
                    Intersection {
                        t: t0,
                        object: self.as_trait()
                    }
                );
            }

            let y1 = t1 * dy + oy;
            if y1 > min && y1 < max {
                result.push(
                    Intersection {
                        t: t1,
                        object: self.as_trait()
                    }
                );
            }
        }
        return result;
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


mod tests {

    //     #[test]
//     fn intersecting_a_cone_with_a_ray() {
//         let mut shape = Shape::double_napped_cone();
//         let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
//
//         let xs = shape.intersect(&r);
//
//         assert_eq!(xs.len(), 2);
//         assert!(float_eq(xs[0].t, 5.0));
//         assert!(float_eq(xs[1].t, 5.0));
//
//         let r = Ray::new(Point::new(1.0, 1.0, -5.0), Vector::new(-0.5, -1.0, 1.0).normalize());
//         let xs = shape.intersect(&r);
//         assert!(float_eq(xs[0].t, 4.55006));
//         assert!(float_eq(xs[1].t, 49.44994));
//     }
//
//     #[test]
//     fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
//         let mut shape = Shape::double_napped_cone();
//         let r = Ray::new(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 1.0, 1.0).normalize());
//
//         let xs = shape.intersect(&r);
//
//         assert_eq!(xs.len(), 1);
//         assert!(float_eq(xs[0].t, 0.35355));
//     }
//
//     #[test]
//     fn intersecting_a_cone_end_caps() {
//         let mut shape = Shape::double_napped_cone();
//         shape.set_cylinder_closed(true);
//         shape.set_cylinder_truncation(-0.5, 0.5);
//
//         let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
//         let xs = shape.intersect(&r);
//         assert_eq!(xs.len(), 0);
//
//         let r = Ray::new(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0).normalize());
//         let xs = shape.intersect(&r);
//         assert_eq!(xs.len(), 2);
//
//         let r = Ray::new(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0).normalize());
//         let xs = shape.intersect(&r);
//         assert_eq!(xs.len(), 4);
//     }
//
//     #[test]
//     fn computing_normal_vector_on_a_cone() {
//         let shape = Shape::double_napped_cone();
//         let p = Point::new(0.0, 0.0, 0.0);
//         let n = shape.normal_at(p);
//         // TODO: fix zero normalize
//         // assert!(n == Vector::new(0.0, 0.0, 0.0));
//
//         let p = Point::new(1.0, 1.0, 1.0);
//         let n = shape.normal_at(p);
//         assert!(n == Vector::new(1.0, -1.0 * f64::sqrt(2.0), 1.0).normalize());
//
//         let p = Point::new(-1.0, -1.0, 0.0);
//         let n = shape.normal_at(p);
//         assert!(n == Vector::new(-1.0, 1.0, 0.0).normalize());
//     }
}