use crate::*;
use crate::intersection::Intersection;
use std::rc::Rc;
use std::mem::swap;
use crate::sphere::Sphere;
use std::borrow::Borrow;

// pub struct Shape {
//     primitives: Vec<Rc<ShapeNode>>,
// }

// add group  - shape
//    new empty node with transform
//         shape node

pub trait ShapeProperties {
    fn transform(&self) -> Transform;

    fn set_transform(&mut self, t: Transform);

    fn normal_at(&self, p: Point) -> Vector;

    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;

    fn as_trait(&self) -> &dyn ShapeProperties;

    fn material(&self) -> Material;

    fn set_material(&mut self, m: Material);
    // fn add_object(&mut self, obj: Rc<shape::Shape>) -> NodeId;
    //
    // fn add_group(&mut self, obj: Rc<Shape>) -> GroupId;
    // fn world_to_object(&self, )
}


pub(crate) fn check_cap_cone(ray: &Ray, t: f64, y: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    return (x * x + z * z) <= f64::abs(y);
}

pub fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();

    return (x * x + z * z) <= 1.0
}

pub(crate) fn check_axis(origin_in_axis: f64, direction_in_axis: f64) -> Vec<f64> {
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

pub fn hit(ts: Vec<Intersection>) -> Option<Intersection> {
    let ts_new = ts.clone();
    for val in ts_new {
        if val.t > 0.0 {
            return Some(val);
        }
    }

    return None;
}

//
// pub struct ShapeNode {
//     transform: Option<Transform>,
//     parent: Option<NodeId>,
//     // previous_sibling: Option<NodeId>,
//     // next_sibling: Option<NodeId>,
//     // first_child: Option<NodeId>,
//     // last_child: Option<NodeId>,
//
//     /// The actual data which will be stored within the tree
//     pub data: Option<shape::Shape>,
//     pub id: Option<NodeId>
// }
//
// pub struct NodeId {
//     index: usize,
// }
//
// pub struct GroupId {
//     index: usize
// }
//
// impl ShapeProperties for Shape {
//     fn transform(&self) -> Transform {
//         self.transform.clone()
//     }
//
//     fn set_transform(&mut self, t: Transform) {
//         self.transform = t;
//     }
//
//     fn normal_at(&self, p: Point) -> Vector {
//         unimplemented!()
//     }
//
//     fn intersect(&mut self, ray: &Ray) -> Vec<Intersection> {
//         let mut result = vec![];
//
//         for object in &mut self.primitives {
//             for t in object.data.kind.intersections(&ray,
//                                                     self.transform.clone() * object.data.transform.clone(),
//                                                     object.data.cylinder_minimum,
//                                                     object.data.cylinder_maximum,
//                                                     object.data.cylinder_closed) {
//                 result.push(
//                     Intersection {
//                         t,
//                         object: object.data.unwrap()
//                     }
//                 )
//             }
//         }
//
//         return result;
//     }
//
//     fn add_object(&mut self, obj: Rc<ShapeNode>) -> NodeId {
//         let next_index = self.primitives.len();
//
//         obj.parent = Some(NodeId {index: 0});
//         obj.id = Some(NodeId {index: next_index});
//         self.primitives.push(
//             obj
//         );
//
//         return NodeId { index: next_index };
//     }
//
//
//     fn add_group(&mut self, group: Rc<shape_props::Shape>) -> NodeId {
//         let next_index = self.primitives.len();
//
//         group.primitives[0].parent = Some(NodeId {index: 0});
//         self.primitives.push(
//             group.primitives[0].
//         );
//
//         for obj in &group.primitives {
//             let next_index = self.primitives.len();
//
//         }
//         return NodeId {index: next_index};
//     }
// }
//
// pub fn group() -> Shape {
//     Shape {primitives: vec![
//         ShapeNode {
//             transform: Some(Transform::new()),
//             parent: None,
//             data: None
//         }
//     ]}
//
// }
//
// mod tests {
//     use super::*;
//
//
//     #[test]
//     fn intersecting_a_ray_with_an_empty_group() {
//         let mut s = group();
//         let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
//         let xs = s.intersect(&r);
//         assert_eq!(xs.len(), 0);
//     }
//
//     #[test]
//     fn intersecting_a_ray_with_a_nonempty_group() {
//         let mut g = group();
//         let mut s1 = shape::Shape::sphere();
//         let mut s2 = shape::Shape::sphere();
//         s2.set_transform(Transform::new().translate(0.0, 0.0, -3.0));
//         let mut s3 = shape::Shape::sphere();
//         s3.set_transform(Transform::new().translate(5.0, 0.0, 0.0));
//
//         g.add_object(s1);
//         g.add_object(s2);
//         g.add_object(s3);
//
//         let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
//
//         let xs = g.intersect(&r);
//         assert_eq!(xs.len(), 4);
//     }
//
//     #[test]
//     fn intersecting_a_transformed_group() {
//         let mut g = group();
//         g.set_transform(Transform::new().scaling(2.0, 2.0, 2.0));
//         let mut s = shape::Shape::sphere();
//         s.set_transform(Transform::new().translate(5.0, 0.0, 0.0));
//         g.add_object(s);
//
//         let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));
//
//         let xs = g.intersect(&r);
//         assert_eq!(xs.len(), 2);
//     }
//
//     #[test]
//     fn converting_a_point_from_world_to_object_space() {
//         let mut g1 = group();
//         g1.set_transform(Transform::new().rotate_y(PI / 2.0));
//         let mut g2 = group();
//         g2.set_transform(Transform::new().scaling(2.0, 2.0, 2.0));
//         let mut s = shape::Shape::sphere();
//         s.set_transform(Transform::new().translate(5.0, 0.0, 0.0));
//
//         g2.add_object(s);
//     }
// }
