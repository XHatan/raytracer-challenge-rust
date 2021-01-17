use crate::sphere::{Sphere, SphereProperties, intersect_sphere, hit};
use crate::light::{PointLight, PointLightProperties};
use crate::tuple::{Tuple, TupleProperties, Point, PointProperties, VectorProperties};
use crate::ray::Ray;
use std::cmp::Ordering::Equal;
use crate::transformation::{Transform, TransformProperty};
use crate::intersection::{Intersection, prepare_computations};
use crate::intersection::AugIntersection;
use crate::material::{phong_lighting, Material, MaterialProperties};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight
}

pub trait WorldProperties {
    fn new() -> World;

    fn color_at_ray(&self, r: &Ray) -> Tuple;

    fn is_shadow(&self, p: Point) -> bool;
}

impl WorldProperties for World {
    fn new() -> World {
        let light_origin = Point::new(-10.0, 10.0, -10.0);
        let color = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let light = PointLight::new(light_origin, color);

        let s_origin = Point::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere_material = Material::new(Tuple::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0);
        let mut s1 = Sphere::new(s_origin, radius);
        s1.set_material(sphere_material.clone());
        let mut s2 = Sphere::new(s_origin, radius);
        s2.set_material(sphere_material.clone());
        let mut t = Transform::new();

        let t2 = t.scaling(0.5, 0.5, 0.5);
        s2.set_transform(t2);
        let objs = vec![s1, s2];

        World {objects: objs, light}
    }

    fn color_at_ray(&self, r: &Ray) -> Tuple {
        let objs = self.objects.clone();
        let mut result: Vec<Intersection> = vec![];
        for s in objs.iter() {
            let mut vals = intersect_sphere(s, r.clone());
            result.append(&mut vals);
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
        let intersection = hit(result);
        return if intersection.t < 0.0 {
            Tuple::new(0.0, 0.0, 0.0, 0.0)
        } else {
            let aug_inter = prepare_computations(&intersection, &r);
            shade_hit(self, &aug_inter)
        }
    }

    fn is_shadow(&self, point: Point) -> bool {
        let shadow_ray_dir = self.light.position() - point;
        let distance = shadow_ray_dir.mag();
        let r = Ray::new(point, shadow_ray_dir.normalize());
        let objs = &self.objects;
        let mut result: Vec<Intersection> = vec![];
        for s in objs.iter() {
            let mut vals = intersect_sphere(s, r);
            result.append(&mut vals);
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
        let intersection = hit(result);
        if intersection.t >= 0.0 && intersection.t < distance {
            return true;
        }

        return false;
    }
}

pub fn intersect_world(w: World, r: Ray) -> Vec<Intersection> {
    let objs = w.objects;
    let mut result: Vec<Intersection> = vec![];
    for s in objs.iter() {
        let mut vals = intersect_sphere(s, r);
        result.append(&mut vals);
    }
    result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
    return result;
}

pub fn shade_hit(w: &World, comps: &AugIntersection) -> Tuple {
    let is_shadow: bool = w.is_shadow(comps.over_point);
    phong_lighting(comps.object.material(), w.light, comps.point, comps.eyev, comps.normalv, is_shadow)
}

mod tests {
    use super::*;
    use crate::intersection::prepare_computations;
    use crate::tuple::{Point, PointProperties, Vector, VectorProperties};

    #[test]
    fn test_world_construction() {
        let default_world = World::new();
        let objs = default_world.objects;
        let sphere = objs[0].clone();
        let origin = sphere.origin();
        assert_eq!(origin == Point::new(0.0, 0.0, 0.0), true);
        let material = sphere.material();
    }

    #[test]
    fn test_intersect_world() {
        let default_world = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let intersections = intersect_world(default_world, r);
        assert_eq!(intersections.len(), 4);
        assert_eq!(f64::abs(intersections[0].t - 4.0) < 0.001, true);
        assert_eq!(f64::abs(intersections[1].t - 4.5) < 0.001, true);
        assert_eq!(f64::abs(intersections[2].t - 5.5) < 0.001, true);
        assert_eq!(f64::abs(intersections[3].t - 6.0) < 0.001, true);
    }

    #[test]
    fn test_shade_hit() {
        let w = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.objects[0].clone();
        let i = Intersection {t: 4.0, object: shape};
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.normalv == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.eyev == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.point == Point::new(0.0, 0.0, -1.0), true);
        let color = shade_hit(&w, &comps);
        let true_color = Tuple::new(0.38066, 0.47583, 0.2855, 0.0);
        assert_eq!(f64::abs(color.x - true_color.x) < 0.01, true);
        assert_eq!(f64::abs(color.y - true_color.y) < 0.01, true);
        assert_eq!(f64::abs(color.z - true_color.z) < 0.01, true);

    }

    // an impossible tests
    // #[test]
    // fn test_shade_hit_from_inside() {
    //     let mut w = World::new();
    //     w.light = PointLight::new(Tuple::new(0.0, 0.25, 0.0, 1.0), Tuple::new(1.0, 1.0, 1.0, 0.0));
    //     let r = Ray::new(Tuple::new(0.0, 0.0, 0.0, 1.0), Tuple::new(0.0, 0.0, 1.0, 0.0));
    //     let shape = w.objects[1].clone();
    //     let i = Intersection {t: 0.5, object: shape};
    //     let comps = prepare_computations(&i, &r);
    //     assert_eq!(comps.normalv == Tuple::new(0.0, 0.0, -1.0, 0.0), true);
    //     assert_eq!(comps.eyev == Tuple::new(0.0, 0.0, -1.0, 0.0), true);
    //     assert_eq!(comps.point == Tuple::new(0.0, 0.0, 0.5, 1.0), true);
    //     assert_eq!(comps.inside, true);
    //     let color = shade_hit(&w, &comps);
    //     let true_color = Tuple::new(0.90498, 0.90498, 0.90498, 0.0);
    //
    //     assert_eq!(color.x, true_color.x);
    // }

    #[test]
    fn test_color_when_a_ray_misses() {
        let w = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at_ray(&r);
        assert_eq!(c == Tuple::new(0.0, 0.0, 0.0, 0.0), true);
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let w = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at_ray(&r);
        assert_eq!(f64::abs(c.x - 0.38066) < 0.001, true);
        assert_eq!(f64::abs(c.y - 0.47583) < 0.001, true);
        assert_eq!(f64::abs(c.z - 0.2855) < 0.001, true);
    }

    // #[test]
    // fn test_color_with_intersection_behind_ray() {
    //     let w = World::new();
    //     let mut outer = w.objects[0].clone();
    //     let mut m = outer.material();
    //     m.ambient = 1.0;
    //     outer.set_material(m);
    //
    //     let mut inner = w.objects[1].clone();
    //     inner.set_material(m);
    //     let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
    //     let c = w.color_at_ray(&r);
    //
    //     let true_color = inner.material().color();
    //     assert_eq!(f64::abs(c.x - true_color.x) < 0.001, true);
    // }

    #[test]
    fn test_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::new();
        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadow(p), false);
    }

    #[test]
    fn test_shadow_when_object_is_collinear_with_point_and_light() {
        let w = World::new();
        let p = Point::new(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadow(p), true);
    }

    #[test]
    fn test_no_shadow_when_object_is_behind_the_point() {
        let w = World::new();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadow(p), false);
    }
}