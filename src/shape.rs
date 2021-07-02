// use crate::*;
//
// #[derive(PartialEq, Debug, Clone)]
// pub enum Kind {
//     Sphere,
//     Plane,
//     Cube,
//     Cylinder,
//     DoubleNappedCone, // radius at y is abs(y)
//     Group
// }
//
// pub trait KindProperties {
//     fn normal_at(&self, point: Point, min: f64, max: f64) -> Vector;
//
//     fn intersections(&self, ray: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;
//
//     fn sphere_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;
//
//     fn plane_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;
//
//     fn cube_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64>;
//
//     fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;
//
//     fn double_napped_cone_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64>;
// }
//
// impl KindProperties for Kind {
//     fn normal_at(&self, local_point: Point, min: f64, max: f64) -> Vector {
//         match self {
//             Sphere => local_point - Point::new(0.0, 0.0, 0.0),
//             Plane => Vector::new(0.0, 1.0, 0.0),
//             Cube => {
//                 let abs_x = f64::abs(local_point.x());
//                 let abs_y = f64::abs(local_point.y());
//                 let abs_z = f64::abs(local_point.z());
//                 let maxc = f64::max(abs_x, f64::max(abs_y, abs_z));
//                 return if maxc == abs_x {
//                     Vector::new(local_point.x(), 0.0, 0.0)
//                 } else if maxc == abs_y {
//                     Vector::new(0.0, local_point.y(), 0.0)
//                 } else {
//                     Vector::new(0.0, 0.0, local_point.z())
//                 }
//             }
//             DoubleNappedCone => {
//                 let dist2 = local_point.x() * local_point.x() + local_point.z() * local_point.z();
//                 let dist = f64::sqrt(dist2);
//                 let y_abs = f64::abs(local_point.y());
//                 if y_abs <= f64::EPSILON {
//                     return Vector::new(0.0, 0.0, 0.0);
//                 }
//                 if dist <= y_abs && local_point.y() >= max - f64::EPSILON  {
//                     return Vector::new(0.0, 1.0, 0.0);
//                 }
//
//                 if dist <= y_abs && local_point.y() <= min + f64::EPSILON {
//                     return Vector::new(0.0, -1.0, 0.0);
//                 }
//
//                 return if local_point.y() > 0.0 {
//                     Vector::new(local_point.x(), -dist, local_point.z())
//                 } else {
//                     Vector::new(local_point.x(), dist, local_point.z())
//                 }
//
//             },
//             Cylinder => {
//                 let dist = local_point.x() * local_point.x() + local_point.z() * local_point.z();
//                 if dist <= 1.0 && local_point.y() >= max - f64::EPSILON  {
//                     return Vector::new(0.0, 1.0, 0.0);
//                 }
//
//                 if dist <= 1.0 && local_point.y() <= min + f64::EPSILON {
//                     return Vector::new(0.0, -1.0, 0.0);
//                 }
//
//                 return Vector::new(local_point.x(), 0.0, local_point.z())
//
//
//             },
//             Group => {
//                 unimplemented!();
//             }
//         }
//     }
//
//     // pub intersections
//     fn intersections(&self, ray: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
//         match self {
//             Sphere => self.sphere_intersections(ray, transform),
//             Plane => self.plane_intersections(ray, transform),
//             Cube => self.cube_intersections(ray, transform),
//             Cylinder => self.cylinder_intersections(ray, transform, min, max, closed),
//             DoubleNappedCone => self.double_napped_cone_intersections(ray, transform, min, max, closed),
//             Group => unimplemented!()
//         }
//     }
//
//     fn sphere_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64> {
//         let origin = Point::new(0.0, 0.0, 0.0);
//
//         // (origin + dir * t).mag == 1
//         let r_t = transform.inverse() * ray;
//         let sphere_to_ray = r_t.origin() - origin;
//         let a = r_t.direction().dot(r_t.direction()); // dir^2
//         let b = 2.0 * r_t.direction().dot(sphere_to_ray); // 2 * dir * (o-o')
//         let c = sphere_to_ray.dot(sphere_to_ray) - 1.0; // (o - o')^2
//
//         let discriminant = b * b - 4.0 * a * c;
//         let mut result: Vec<f64> = vec![];
//         return if discriminant < 0.0 {
//             result
//         } else {
//             let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
//             let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
//             result.push(t1);
//             result.push(t2);
//             result
//         }
//     }
//
//     fn plane_intersections(&self, ray: &Ray, transform: Transform) -> Vec<f64> {
//         let r_t = transform.inverse() * ray;
//         match r_t.direction().y().abs() > 0.0001 {
//             true => vec![-r_t.origin().y() / r_t.direction().y()],
//             false => vec![]
//         }
//     }
//
//     fn cube_intersections(&self, ray_world: &Ray, transform: Transform) -> Vec<f64> {
//         let r_object = transform.inverse() * ray_world;
//         let x_t = check_axis(r_object.origin().x(), r_object.direction().x());
//         let y_t = check_axis(r_object.origin().y(), r_object.direction().y());
//         let z_t = check_axis(r_object.origin().z(), r_object.direction().z());
//
//         let t_min = f64::max(f64::max(x_t[0], y_t[0]), z_t[0]);
//         let t_max = f64::min(f64::min(x_t[1], y_t[1]), z_t[1]);
//
//         match t_min > t_max {
//             true => vec![],
//             false => vec![t_min, t_max]
//         }
//     }
//
//     fn cylinder_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
//         let ray_obj = transform.inverse() * ray_world;
//         let mut result: Vec<f64> = vec![];
//
//         // check capped
//         if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
//             let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
//             if check_cap(&ray_obj, t) {
//                 result.push(t);
//             }
//
//             let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
//             if check_cap(&ray_obj, t) {
//                 result.push(t);
//             }
//         }
//
//         // point = (t * dir + origin)
//         // p2 = point(x, z)
//         // dir^2 + origin^2 + 2 * t * dir * origin
//         let a = f64::powf(ray_obj.direction().x(), 2.0) + f64::powf(ray_obj.direction().z(), 2.0);
//         if f64::abs(a) < 0.00001 {
//             return result;
//         }
//         let b = 2.0 * ray_obj.origin().x() * ray_obj.direction().x() + 2.0 * ray_obj.origin().z() * ray_obj.direction().z();
//         let c = f64::powf(ray_obj.origin().x(), 2.0) + f64::powf(ray_obj.origin().z(), 2.0) - 1.0;
//         let disc = f64::powf(b, 2.0) - 4.0 * a * c;
//
//         if disc < 0.0 {
//             return result;
//         } else {
//             let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
//             let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);
//             if t0 > t1 {
//                 swap(&mut t0, &mut t1);
//             }
//             let y0 = t0 * ray_obj.direction().y() + ray_obj.origin().y();
//             if y0 > min && y0 < max {
//                 result.push(t0);
//             }
//
//             let y1 = t1 * ray_obj.direction().y() + ray_obj.origin().y();
//             if y1 > min && y1 < max {
//                 result.push(t1);
//             }
//
//             return result;
//         }
//     }
//
//     fn double_napped_cone_intersections(&self, ray_world: &Ray, transform: Transform, min: f64, max: f64, closed: bool) -> Vec<f64> {
//         let ray_obj = transform.inverse() * ray_world;
//         let mut result: Vec<f64> = vec![];
//
//         // check capped
//         if closed && f64::abs(ray_obj.direction().y()) > 0.0001 {
//             let t = (min - ray_obj.origin().y()) / ray_obj.direction().y();
//             if check_cap_cone(&ray_obj, t, min) {
//                 result.push(t);
//             }
//
//             let t = (max - ray_obj.origin().y()) / ray_obj.direction().y();
//             if check_cap_cone(&ray_obj, t, max) {
//                 result.push(t);
//             }
//         }
//
//         let dx = ray_obj.direction().x();
//         let dy = ray_obj.direction().y();
//         let dz = ray_obj.direction().z();
//
//         let ox = ray_obj.origin().x();
//         let oy = ray_obj.origin().y();
//         let oz = ray_obj.origin().z();
//
//
//         let a = dx * dx - dy * dy + dz * dz;
//         let b = 2.0 * ox * dx - 2.0 * oy * dy + 2.0 * oz * dz;
//         let c = ox * ox - oy * oy + oz * oz;
//
//         if f64::abs(a) <= f64::EPSILON && f64::abs(b) <= f64::EPSILON {
//             return result;
//         }
//
//         if f64::abs(a) <= f64::EPSILON {
//             let t = -c / (2.0 * b);
//             let y = t * dy + oy;
//             if y > min && y < max {
//                 result.push(t);
//             }
//         }
//
//         let disc = b * b  - 4.0 * a * c;
//
//         if disc >= 0.0 {
//             let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
//             let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);
//             if t0 > t1 {
//                 swap(&mut t0, &mut t1);
//             }
//             let y0 = t0 * dy + oy;
//             if y0 > min && y0 < max {
//                 result.push(t0);
//             }
//
//             let y1 = t1 * dy + oy;
//             if y1 > min && y1 < max {
//                 result.push(t1);
//             }
//         }
//         return result;
//     }
// }
//

//
// use self::Kind::*;
// use crate::intersection::Intersection;
// use std::mem::swap;
// use std::thread::yield_now;
//
// pub struct Shape {
//     pub material: Material,
//     pub transform: Transform,
//     pub kind: Kind,
//     pub cylinder_minimum: f64,
//     pub cylinder_maximum: f64,
//     pub cylinder_closed: bool,
// }
//
// impl Shape {
//     fn new(kind: Kind) -> Shape {
//         Self {
//             material: Material::default(),
//             transform: Transform::new(),
//             kind,
//             cylinder_minimum: f64::MIN,
//             cylinder_maximum: f64::MAX,
//             cylinder_closed: false,
//         }
//     }
//
//     pub fn sphere() -> Shape {
//         Shape::new(Sphere)
//     }
//
//     pub fn plane() -> Shape {
//         Shape::new(Plane)
//     }
//
//     pub fn cube() -> Shape {
//         Shape::new(Cube)
//     }
//
//     pub fn cylinder() -> Shape {
//         Shape::new(Cylinder)
//     }
//
//     pub fn double_napped_cone() -> Shape {
//         Shape::new(DoubleNappedCone)
//     }
//
//     pub fn group() -> Shape {
//         Shape::new(Group)
//     }
//
//     pub fn set_cylinder_truncation(&mut self, min: f64, max: f64) {
//         match self.kind {
//             Cylinder => {
//                 self.cylinder_minimum = min;
//                 self.cylinder_maximum = max;
//             }
//             DoubleNappedCone => {
//                 self.cylinder_minimum = min;
//                 self.cylinder_maximum = max;
//             }
//             _ => unimplemented!()
//         }
//     }
//
//     pub fn set_cylinder_closed(&mut self, closed: bool) {
//         match self.kind {
//             Cylinder => {
//                 self.cylinder_closed = closed;
//             }
//             DoubleNappedCone => {
//                 self.cylinder_closed = closed;
//             }
//             _ => unimplemented!()
//         }
//     }
//
//     pub fn set_material(&mut self, material: &Material) {
//         self.material = material.clone();
//     }
//
//     pub fn set_transform(&mut self, transform: Transform) {
//         self.transform = transform;
//     }
//
//     pub fn transform(&self) -> Transform {
//         self.transform.clone()
//     }
//
//     pub fn material(&self) -> Material {
//         self.material.clone()
//     }
//
//     // pub fn append_intersections<'a>(
//     //     &'a self,
//     //     ray: &Ray,
//     //     intersections: &mut Vec<Intersection<'a>>,
//     // ) {
//     //     let local_ray = ray.transform(self.inverse);
//     //     self.append_local_intersections(&local_ray, intersections);
//     // }
//
//     // pub fn append_local_intersections<'a>(
//     //     &'a self,
//     //     local_ray: &Ray,
//     //     intersections: &mut Vec<Intersection<'a>>,
//     // ) {
//     //     for t in self.kind.intersections(local_ray) {
//     //         intersections.push(Intersection {
//     //             t,
//     //             object: self,
//     //         });
//     //     }
//     // }
//
//     // expect ray in world coord
//     pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
//         let mut result = vec![];
//
//         for t in self.kind.intersections(ray, self.transform.clone(),
//                                          self.cylinder_minimum, self.cylinder_maximum, self.cylinder_closed) {
//             result.push(
//                 Intersection {
//                     t,
//                     object: self.clone()
//                 }
//             )
//         }
//
//         result
//         // match result.is_empty() {
//         //     true => resul,
//         //     false => Some(result),
//         // }
//     }
//
//     // pub fn local_intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
//     //     let mut result = vec![];
//
//     //     self.append_local_intersections(ray, &mut result);
//
//     //     match result.is_empty() {
//     //         true => None,
//     //         false => Some(result),
//     //     }
//     // }
//
//     pub fn normal_at(&self, p: Point) -> Vector {
//         let point_obj_space = self.transform.inverse() * p;
//         let normal_obj_space = self.kind.normal_at(point_obj_space, self.cylinder_minimum, self.cylinder_maximum);
//         let mut world_normal = self.transform.inverse().transpose() * normal_obj_space;
//         world_normal.data.w = 0.0;
//         return world_normal.normalize();
//     }
//
// }
//
// impl Clone for Shape {
//     fn clone(&self) -> Self {
//         Self {
//             material: self.material.clone(),
//             transform: self.transform.clone(),
//             kind: self.kind.clone(),
//             cylinder_minimum: self.cylinder_minimum,
//             cylinder_maximum: self.cylinder_maximum,
//             cylinder_closed: self.cylinder_closed
//         }
//     }
// }
//
// pub fn sphere() -> Shape {
//     Shape::sphere()
// }
//
// pub fn glass_sphere() -> Shape {
//     let mut s = Shape::sphere();
//     s.material.transparency = 1.0;
//     s.material.refractive_index = 1.5;
//
//     return s;
// }
//
// pub fn plane() -> Plane {
//     plane::Plane {
//         transform: Transform::new(),
//         material: Material::default()
//     }
// }
//
// pub fn test_shape() -> Shape {
//     Shape::sphere()
// }
//

//
// pub fn cube() -> Shape {
//     Shape::cube()
// }
//
// pub fn cylinder() -> Shape {
//     Shape::cylinder()
// }
//
// impl PartialEq for Shape {
//     fn eq(&self, other: &Self) -> bool {
//         self.material == other.material
//     }
// }
//