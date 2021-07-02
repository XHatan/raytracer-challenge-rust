use crate::*;
use crate::shape_props::{ShapeProperties, check_axis};
use crate::intersection::Intersection;


#[derive(Clone)]
pub struct Cube {
    pub material: Material,
    pub transform: Transform,
}

impl ShapeProperties for Cube {
    fn transform(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, t: Transform) {
        self.transform = t;
    }

    fn normal_at(&self, p: Point) -> Vector {
        let point_obj_space = self.transform.inverse() * p;
        let local_point = point_obj_space;
        let abs_x = f64::abs(local_point.x());
        let abs_y = f64::abs(local_point.y());
        let abs_z = f64::abs(local_point.z());
        let maxc = f64::max(abs_x, f64::max(abs_y, abs_z));
        let mut normal_obj_space: Vector;
        if maxc == abs_x {
            normal_obj_space = Vector::new(local_point.x(), 0.0, 0.0)
        } else if maxc == abs_y {
            normal_obj_space = Vector::new(0.0, local_point.y(), 0.0)
        } else {
            normal_obj_space= Vector::new(0.0, 0.0, local_point.z())
        }
        let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
        world_normal.data.w = 0.0;
        return world_normal.normalize();
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let r_object = self.transform.inverse() * ray;
        let x_t = check_axis(r_object.origin().x(), r_object.direction().x());
        let y_t = check_axis(r_object.origin().y(), r_object.direction().y());
        let z_t = check_axis(r_object.origin().z(), r_object.direction().z());

        let t_min = f64::max(f64::max(x_t[0], y_t[0]), z_t[0]);
        let t_max = f64::min(f64::min(x_t[1], y_t[1]), z_t[1]);

        return if t_min > t_max {
            vec![]
        } else {
            vec![
                Intersection {
                    t: t_min,
                    object: self.as_trait()
                },
                Intersection {
                    t: t_max,
                    object: self.as_trait()
                }
            ]
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

impl Cube {
    pub fn default() -> Self {
        Cube {
            transform: Transform::new(),
            material: Material::default()
        }
    }
}
pub fn cube() -> Cube {
    Cube::default()
}

mod tests {
    use super::*;
    use crate::material::float_eq;


    #[test]
    fn a_ray_intersects_a_cube() {
        let c = cube();
        let r = Ray::new(Point::new(5.0, 0.5, 0.0), Vector::new(-1.0, 0.0, 0.0));
        let xs = c.intersect(&r);

        assert!(float_eq(xs[0].t, 4.0));
        assert!(float_eq(xs[1].t, 6.0));

        let r1 = Ray::new(Point::new(0.0, 0.5, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs1 = c.intersect(&r1);
        assert!(float_eq(xs1[0].t, -1.0));
        assert!(float_eq(xs1[1].t, 1.0));
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = cube();
        let r = Ray::new(Point::new(-2.0, 0.0, 0.0), Vector::new(0.2673, 0.5345, 0.8018));
        let x = c.intersect(&r);
        assert_eq!(x.len(), 0);
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = cube();
        let p = Point::new(1.0, 0.5, -0.8);
        let n1 = c.normal_at(p);
        let n2 = c.normal_at(Point::new(-1.0, -0.2, 0.9));
        assert!(n1 == Vector::new(1.0, 0.0, 0.0));
        assert!(n2 == Vector::new(-1.0, 0.0, 0.0));
    }
}