use crate::*;

#[derive(PartialEq, Debug, Clone)]
enum Kind {
    Sphere,
    Plane,
    Cube,
    Cylinder
}

pub trait KindProperties {
    fn normal_at(&self, point: Point) -> Vector;

    fn intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn sphere_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn plane_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn cube_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform) -> Vec<f64>;
}

impl KindProperties for Kind {

    fn normal_at(&self, local_point: Point) -> Vector {
        match self {
            Sphere => local_point - Point::new(0.0, 0.0, 0.0),
            Plane => Vector::new(0.0, 1.0, 0.0),
            Cube => {
                let abs_x = f64::abs(local_point.x());
                let abs_y = f64::abs(local_point.y());
                let abs_z = f64::abs(local_point.z());
                let maxc = f64::max(abs_x, f64::max(abs_y, abs_z));
                return if maxc == abs_x {
                    Vector::new(local_point.x(), 0.0, 0.0)
                } else if maxc == abs_y {
                    Vector::new(0.0, local_point.y(), 0.0)
                } else {
                    Vector::new(0.0, 0.0, local_point.z())
                }
            }
            Cylinder => {
                Vector::new(local_point.x, 0.0, local_point.z)
            }
        }
    }

    // pub intersections
    fn intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64> {
        match self {
            Sphere => self.sphere_intersections(ray, transform),
            Plane => self.plane_intersections(ray, transform),
            Cube => self.cube_intersections(ray, transform),
            Cylinder => unimplemented!()
        }
    }

    fn sphere_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64> {
        let origin = Point::new(0.0, 0.0, 0.0);

        // (origin + dir * t).mag == 1
        let r_t = transform.inverse() * ray;
        let sphere_to_ray = r_t.origin() - origin;
        let a = r_t.direction().dot(r_t.direction()); // dir^2
        let b = 2.0 * r_t.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0; // (o - o')^2

        let discriminant = b * b - 4.0 * a * c;
        return if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
            vec![t1, t2]
        }
    }

    fn plane_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64> {
        let r_t = transform.inverse() * ray;
        match r_t.direction().y().abs() > 0.0001 {
            true => vec![-r_t.origin().y() / r_t.direction().y()],
            false => vec![]
        }
    }

    fn cube_intersections(&self, ray_world: &Ray, transform: Transform) -> Vec<f64> {
        let r_object = transform.inverse() * ray_world;
        let x_t = check_axis(r_object.origin().x(), r_object.direction().x());
        let y_t = check_axis(r_object.origin().y(), r_object.direction().y());
        let z_t = check_axis(r_object.origin().z(), r_object.direction().z());

        let t_min = f64::max(f64::max(x_t[0], y_t[0]), z_t[0]);
        let t_max = f64::min(f64::min(x_t[1], y_t[1]), z_t[1]);

        match t_min > t_max {
            true => vec![],
            false => vec![t_min, t_max]
        }
    }

    fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform) -> Vec<f64> {
        let ray_obj = transform.inverse() * ray_world;
        // point = (t * dir + origin)
        // p2 = point(x, z)
        // dir^2 + origin^2 + 2 * t * dir * origin
        //
        let a = f64::powf(ray_obj.direction().x(), 2.0) + f64::powf(ray_obj.direction().z(), 2.0);
        if f64::abs(a) < 0.00001 {
            vec![]
        }
        let b = 2 * ray_obj.origin().x() * ray_obj.direction().x() + 2 * ray_obj.origin().z() * ray_obj.direction().z();
        let c = f64::powf(ray_obj.origin().x(), 2.0) + f64::powf(ray_obj.origin().z(), 2.0) - 1.0;
        let disc = f64::powf(b, 2.0) - 4.0 * a * c;
        return if disc < 0.0 {
            vec![]
        } else {
            let t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
            let t1 = (-b + f64::sqrt(disc)) / (2.0 * a);
            vec![t0, t1]
        }
    }
}

fn check_axis(origin_in_axis: f64, direction_in_axis: f64) -> Vec<f64> {
    let mut tmin_numerator = -1.0 - origin_in_axis;
    let mut tmax_numerator = 1.0 - origin_in_axis;
    let mut tmin: f64;
    let mut tmax: f64;
    if f64::abs(direction_in_axis) >= intersection::EPSILON {
        tmin = tmin_numerator / direction_in_axis;
        tmax = tmax_numerator / direction_in_axis;
    } else {
        // tmin/max_numerator * INFINITY
        if tmin_numerator > 0.0 {
            tmin = f64::MAX;
        } else {
            tmin = f64::MIN;
        }
        if tmax_numerator > 0.0 {
            tmax = f64::MAX;
        } else {
            tmax = f64::MIN;
        }
    }
    if tmin > tmax {
        swap(&mut tmin, &mut tmax);
    }

    return vec![tmin, tmax];
}

use self::Kind::*;
use crate::intersection::Intersection;
use std::mem::swap;

pub struct Shape {
    pub material: Material,
    pub transform: Transform,
    kind: Kind,
}

impl Shape {
    fn new(kind: Kind) -> Shape {
        Self {
            material: Material::default(),
            transform: Transform::new(),
            kind,
        }
    }

    pub fn sphere() -> Shape {
        Shape::new(Sphere)
    }

    pub fn plane() -> Shape {
        Shape::new(Plane)
    }

    pub fn cube() -> Shape {
        Shape::new(Cube)
    }

    pub fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn transform(&self) -> Transform {
        self.transform.clone()
    }

    pub fn material(&self) -> Material {
        self.material.clone()  
    }

    // pub fn append_intersections<'a>(
    //     &'a self,
    //     ray: &Ray,
    //     intersections: &mut Vec<Intersection<'a>>,
    // ) {
    //     let local_ray = ray.transform(self.inverse);
    //     self.append_local_intersections(&local_ray, intersections);
    // }

    // pub fn append_local_intersections<'a>(
    //     &'a self,
    //     local_ray: &Ray,
    //     intersections: &mut Vec<Intersection<'a>>,
    // ) {
    //     for t in self.kind.intersections(local_ray) {
    //         intersections.push(Intersection {
    //             t,
    //             object: self,
    //         });
    //     }
    // }

    // expect ray in world coord
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut result = vec![];

        for t in self.kind.intersections(ray, self.transform.clone()) {
            result.push(
                Intersection {
                    t,
                    object: self.clone()
                }
            )
        }

        result
        // match result.is_empty() {
        //     true => resul,
        //     false => Some(result),
        // }
    }

    // pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
    //     let mut result = vec![];

    //     self.append_local_intersections(ray, &mut result);

    //     match result.is_empty() {
    //         true => None,
    //         false => Some(result),
    //     }
    // }

    pub fn normal_at(&self, p: Point) -> Vector {
        let point_obj_space = self.transform.inverse() * p;
        let normal_obj_space = self.kind.normal_at(point_obj_space);
        let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
        world_normal.data.w = 0.0;
        return world_normal.normalize();
    }
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Self {
            material: self.material.clone(),
            transform: self.transform.clone(),
            kind: self.kind.clone()
        }
    }
}

pub fn sphere() -> Shape {
    Shape::sphere()
}

pub fn glass_sphere() -> Shape {
    let mut s = Shape::sphere();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;

    return s;
}

pub fn plane() -> Shape {
    Shape::plane()
}

pub fn test_shape() -> Shape {
    Shape::sphere()
}

pub fn hit(ts: Vec<Intersection>) -> Intersection {
    let ts_new = ts.clone();
    for val in ts_new {
        if val.t > 0.0 {
            return val;
        }
    }

    Intersection{t: -1.0, object: sphere()}
}

pub fn cube() -> Shape {
    Shape::cube()
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.material == other.material
    }
}

mod tests {
    use super::*;
    use crate::tuple::{Point, PointProperties, Vector};
    use crate::material::float_eq;

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = sphere();
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
        let mut s = test_shape();
        s.set_transform(
            Transform::new().scaling(2.0, 2.0, 2.0)
        );

        let xs = s.intersect(&r);

    }

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