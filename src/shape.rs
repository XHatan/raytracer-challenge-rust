use crate::*;

#[derive(PartialEq, Debug, Clone)]
enum Kind {
    Sphere,
    Plane,
    Cube,
    Cylinder,
    DoubleNappedCone // radius at y is abs(y)
}

pub trait KindProperties {
    fn normal_at(&self, point: Point, min: f64, max: f64) -> Vector;

    fn intersections(&self, ray: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;

    fn sphere_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn plane_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn cube_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;

    fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;

    fn double_napped_cone_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;
}

impl KindProperties for Kind {
    fn normal_at(&self, local_point: Point, min: f64, max: f64) -> Vector {
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
            DoubleNappedCone => {
                let dist2 = local_point.x() * local_point.x() + local_point.z() * local_point.z();
                let dist = f64::sqrt(dist2);
                let y_abs = f64::abs(local_point.y());
                if y_abs <= f64::EPSILON {
                    return Vector::new(0.0, 0.0, 0.0);
                }
                if dist <= y_abs && local_point.y() >= max - f64::EPSILON  {
                    return Vector::new(0.0, 1.0, 0.0);
                }

                if dist <= y_abs && local_point.y() <= min + f64::EPSILON {
                    return Vector::new(0.0, -1.0, 0.0);
                }

                return if local_point.y() > 0.0 {
                    Vector::new(local_point.x(), -dist, local_point.z())
                } else {
                    Vector::new(local_point.x(), dist, local_point.z())
                }

            },
            Cylinder => {
                let dist = local_point.x() * local_point.x() + local_point.z() * local_point.z();
                if dist <= 1.0 && local_point.y() >= max - f64::EPSILON  {
                    return Vector::new(0.0, 1.0, 0.0);
                }

                if dist <= 1.0 && local_point.y() <= min + f64::EPSILON {
                    return Vector::new(0.0, -1.0, 0.0);
                }

                return Vector::new(local_point.x(), 0.0, local_point.z())
            }
        }
    }

    // pub intersections
    fn intersections(&self, ray: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
        match self {
            Sphere => self.sphere_intersections(ray, transform),
            Plane => self.plane_intersections(ray, transform),
            Cube => self.cube_intersections(ray, transform),
            Cylinder => self.cylinder_intersections(ray, transform, min, max, closed),
            DoubleNappedCone => self.double_napped_cone_intersections(ray, transform, min, max, closed)
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
        let mut result: Vec<f64> = vec![];
        return if discriminant < 0.0 {
            result
        } else {
            let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
            let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
            result.push(t1);
            result.push(t2);
            result
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

    fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
        let ray_obj = transform.inverse() * ray_world;
        let mut result: Vec<f64> = vec![];

        // check capped
        if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
            let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
            if check_cap(&ray_obj, t) {
                result.push(t);
            }

            let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
            if check_cap(&ray_obj, t) {
                result.push(t);
            }
        }

        // point = (t * dir + origin)
        // p2 = point(x, z)
        // dir^2 + origin^2 + 2 * t * dir * origin
        let a = f64::powf(ray_obj.direction().x(), 2.0) + f64::powf(ray_obj.direction().z(), 2.0);
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
                result.push(t0);
            }

            let y1 = t1 * ray_obj.direction().y() + ray_obj.origin().y();
            if y1 > min && y1 < max {
                result.push(t1);
            }

            return result;
        }
    }

    fn double_napped_cone_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
        let ray_obj = transform.inverse() * ray_world;
        let mut result: Vec<f64> = vec![];

        // check capped
        if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
            let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
            if check_cap_cone(&ray_obj, t, min) {
                result.push(t);
            }

            let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
            if check_cap_cone(&ray_obj, t, max) {
                result.push(t);
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
                result.push(t);
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
                result.push(t0);
            }

            let y1 = t1 * dy + oy;
            if y1 > min && y1 < max {
                result.push(t1);
            }
        }
        return result;
    }
}

fn check_cap_cone(ray: &Ray, t: f64, y: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    return (x * x + z * z) <= f64::abs(y);
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    return (x * x + z * z) <= 1.0
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
use std::thread::yield_now;
use nalgebra::abs;

pub struct Shape {
    pub material: Material,
    pub transform: Transform,
    kind: Kind,
    cylinder_minimum: f64,
    cylinder_maximum: f64,
    cylinder_closed: bool
}

impl Shape {
    fn new(kind: Kind) -> Shape {
        Self {
            material: Material::default(),
            transform: Transform::new(),
            kind,
            cylinder_minimum: f64::MIN,
            cylinder_maximum: f64::MAX,
            cylinder_closed: false
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

    pub fn cylinder() -> Shape {
        Shape::new(Cylinder)
    }

    pub fn double_napped_cone() -> Shape {
        Shape::new(DoubleNappedCone)
    }

    pub fn set_cylinder_truncation(&mut self, min: f64, max: f64) {
        match self.kind {
            Cylinder => {
                self.cylinder_minimum = min;
                self.cylinder_maximum = max;
            }
            DoubleNappedCone => {
                self.cylinder_minimum = min;
                self.cylinder_maximum = max;
            }
            _ => unimplemented!()
        }
    }

    pub fn set_cylinder_closed(&mut self, closed: bool) {
        match self.kind {
            Cylinder => {
                self.cylinder_closed = closed;
            }
            DoubleNappedCone => {
                self.cylinder_closed = closed;
            }
            _ => unimplemented!()
        }
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

        for t in self.kind.intersections(ray, self.transform.clone(),
                                         self.cylinder_minimum, self.cylinder_maximum, self.cylinder_closed) {
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
        let normal_obj_space = self.kind.normal_at(point_obj_space, self.cylinder_minimum, self.cylinder_maximum);
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
            kind: self.kind.clone(),
            cylinder_minimum: self.cylinder_minimum,
            cylinder_maximum: self.cylinder_maximum,
            cylinder_closed: self.cylinder_closed
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

pub fn cylinder() -> Shape {
    Shape::cylinder()
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

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let mut shape = Shape::double_napped_cone();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = shape.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 5.0));
        assert!(float_eq(xs[1].t, 5.0));

        let r = Ray::new(Point::new(1.0, 1.0, -5.0), Vector::new(-0.5, -1.0, 1.0).normalize());
        let xs = shape.intersect(&r);
        assert!(float_eq(xs[0].t, 4.55006));
        assert!(float_eq(xs[1].t, 49.44994));
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let mut shape = Shape::double_napped_cone();
        let r = Ray::new(Point::new(0.0, 0.0, -1.0), Vector::new(0.0, 1.0, 1.0).normalize());

        let xs = shape.intersect(&r);

        assert_eq!(xs.len(), 1);
        assert!(float_eq(xs[0].t, 0.35355));
    }

    #[test]
    fn intersecting_a_cone_end_caps() {
        let mut shape = Shape::double_napped_cone();
        shape.set_cylinder_closed(true);
        shape.set_cylinder_truncation(-0.5, 0.5);

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0).normalize());
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0).normalize());
        let xs = shape.intersect(&r);
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn computing_normal_vector_on_a_cone() {
        let shape = Shape::double_napped_cone();
        let p = Point::new(0.0, 0.0, 0.0);
        let n = shape.normal_at(p);
        // TODO: fix zero normalize
        // assert!(n == Vector::new(0.0, 0.0, 0.0));

        let p = Point::new(1.0, 1.0, 1.0);
        let n = shape.normal_at(p);
        assert!(n == Vector::new(1.0, -1.0 * f64::sqrt(2.0), 1.0).normalize());

        let p = Point::new(-1.0, -1.0, 0.0);
        let n = shape.normal_at(p);
        assert!(n == Vector::new(-1.0, 1.0, 0.0).normalize());
    }





}