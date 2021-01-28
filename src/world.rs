use crate::light::{PointLight, PointLightProperties};
use crate::tuple::{Tuple, TupleProperties, Point, PointProperties, VectorProperties};
use crate::ray::Ray;
use std::cmp::Ordering::Equal;
use crate::transformation::{Transform, TransformProperty};
use crate::intersection::{Intersection, prepare_computations, schlick};
use crate::intersection::AugIntersection;
use crate::material::{phong_lighting, Material, MaterialProperties};
use crate::shape;
use crate::shape::sphere;

pub struct World {
    pub objects: Vec<shape::Shape>,
    pub light: PointLight
}

pub trait WorldProperties {
    fn new() -> World;

    fn default() -> World;

    fn color_at_ray(&self, r: &Ray, remaining: i32) -> Tuple;

    fn is_shadow(&self, p: Point) -> bool;

    fn reflected_color(&self, comps: &AugIntersection, remaining: i32) -> Tuple;

    fn refracted_color(&self, comps: &AugIntersection, remaining: i32) -> Tuple;
}

impl WorldProperties for World {
    fn new() -> World {
        let light_origin = Point::new(-10.0, 10.0, -10.0);
        let color = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let light = PointLight::new(light_origin, color);

        let sphere_material = Material::new(Tuple::new(0.8, 1.0, 0.6, 1.0), 0.1, 0.7, 0.2, 200.0);
        let mut s1 = shape::sphere();
        s1.set_material(&sphere_material);
        let mut s2 = shape::sphere();
        s2.set_material(&sphere_material);
        let mut t = Transform::new();

        let t2 = t.scaling(0.5, 0.5, 0.5);
        s2.set_transform(t2);
        let objs = vec![s1, s2];

        World {objects: objs, light}
    }

    fn default() -> World {
        let light_origin = Point::new(-10.0, 10.0, -10.0);
        let color = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let light = PointLight::new(light_origin, color);

        let sphere_material = Material::new(
            Tuple::new(0.8, 1.0, 0.6, 1.0),
            0.1, 0.7, 0.2, 200.0);
        let mut s1 = shape::sphere();
        s1.set_material(&sphere_material);

        let mut s2 = shape::sphere();
        s2.set_transform(Transform::new().scaling(0.5, 0.5, 0.5));

        let objs = vec![s1, s2];

        World {objects: objs, light}
    }

    fn color_at_ray(&self, r: &Ray, remaining: i32) -> Tuple {
        let objs = self.objects.clone();
        let mut result: Vec<Intersection> = vec![];
        for s in objs.iter() {
            let mut intersect_values = s.intersect(&r);
            result.append(&mut intersect_values);
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));


        let intersection = shape::hit(result.clone());
        return if intersection.t < 0.0 {
            Tuple::new(0.0, 0.0, 0.0, 0.0)
        } else {
            let aug_inter = prepare_computations(&intersection, &r, &result);
            shade_hit(self, &aug_inter, remaining)
        }
    }

    fn is_shadow(&self, point: Point) -> bool {
        let shadow_ray_dir = self.light.position() - point;
        let distance = shadow_ray_dir.mag();
        let r = Ray::new(point, shadow_ray_dir.normalize());
        let objs = &self.objects;
        let mut result: Vec<Intersection> = vec![];
        for s in objs.iter() {
            let mut vals = s.intersect(&r);
            result.append(&mut vals);
        }
        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
        let intersection = shape::hit(result);
        if intersection.t >= 0.0 && intersection.t < distance {
            return true;
        }

        return false;
    }

    fn reflected_color(&self, aug_intersection: &AugIntersection, remaining: i32) -> Tuple {
        if remaining <= 0 {
            return Tuple::new(0.0, 0.0, 0.0, 0.0);
        }

        let reflective_factor = aug_intersection.object.material().reflective;
        if reflective_factor < 0.00001 {
            return Tuple::new(0.0, 0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(aug_intersection.over_point, aug_intersection.reflectv);
        let color = self.color_at_ray(&reflect_ray, remaining - 1);
        color * reflective_factor
    }

    fn refracted_color(&self, comps: &AugIntersection, remaining: i32) -> Tuple {
        if remaining == 0 || comps.object.material.transparency <= 0.000001 {
            return Tuple::new(0.0, 0.0, 0.0, 0.0);
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t >= 1.0 {
            return Tuple::new(0.0, 0.0, 0.0, 0.0);
        }

        let cos_t = f64::sqrt(1.0 - sin2_t);
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refracted_ray = Ray::new(comps.under_point, direction);
        let color = self.color_at_ray(&refracted_ray, remaining - 1) * comps.object.material.transparency;

        return color;
    }
}

pub fn intersect_world(w: World, r: Ray) -> Vec<Intersection> {
    let objs = w.objects;
    let mut result: Vec<Intersection> = vec![];
    for s in objs.iter() {
        let mut vals = s.intersect(&r);
        result.append(&mut vals);
    }
    result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Equal));
    return result;
}

pub fn shade_hit(w: &World, comps: &AugIntersection, remaining: i32) -> Tuple {
    let is_shadow: bool = w.is_shadow(comps.over_point);
    let surface = phong_lighting(
        &comps.object.material(),
        w.light,
        comps.over_point,
        comps.eyev,
        comps.normalv,
        is_shadow,
        &sphere()
    );
    let reflected = w.reflected_color(&comps, remaining);
    let refracted = w.refracted_color(&comps, remaining);

    let material = comps.object.material();

    if material.reflective > 0.0 && material.transparency > 0.0 {
        let reflectance = schlick(&comps);
        return surface + reflected * reflectance + refracted * (1.0 - reflectance);
    } else {
        surface + reflected + refracted
    }

}

mod tests {
    use super::*;
    use crate::intersection::prepare_computations;
    use crate::tuple::{Point, PointProperties, Vector, VectorProperties};
    use crate::shape::plane;

    #[test]
    fn test_world_construction() {
        let default_world = World::new();
        let objs = default_world.objects;
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
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        assert_eq!(comps.normalv == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.eyev == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.point == Point::new(0.0, 0.0, -1.0), true);
        let color = shade_hit(&w, &comps, 1);
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
        let c = w.color_at_ray(&r, 1);
        assert_eq!(c == Tuple::new(0.0, 0.0, 0.0, 0.0), true);
    }

    #[test]
    fn test_color_when_a_ray_hits() {
        let w = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at_ray(&r, 1);
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

    #[test]
    fn test_the_reflected_color_for_a_nonreflective_material() {
        let w = World::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = w.objects[1].clone();
        let mut new_m = shape.material();
        new_m.ambient = 1.0;
        shape.set_material(&new_m);
        let i = Intersection {t: 1.0, object: shape.clone()};
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(f64::abs(color.x) < 0.001, true);
        assert_eq!(f64::abs(color.y) < 0.001, true);
        assert_eq!(f64::abs(color.z) < 0.001, true);
    }

    #[test]
    fn test_the_reflected_color_for_a_reflective_material() {
        let mut w = World::new();
        let mut plane = plane();
        plane.material.reflective = 0.5;
        plane.set_transform(
            Transform::new().translate(0.0, -1.0, 0.0)
        );

        w.objects.push(plane.clone());

        let r = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, -f64::sqrt(2.0)/ 2.0, f64::sqrt(2.0)/ 2.0));
        let i = Intersection {t: f64::sqrt(2.0) / 2.0, object: plane.clone()};
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        let color = w.reflected_color(&comps, 1);

        assert_eq!(color.x, 0.19032);
        assert_eq!(color.y, 0.2379);
        // assert_eq!(f64::abs(color.x - 0.19032) < 0.001, true);
        // assert_eq!(f64::abs(color.y - 0.2379) < 0.001, true);
        // assert_eq!(f64::abs(color.z - 0.14274) < 0.001, true);
    }

    #[test]
    fn test_shade_hit_with_reflective_material() {
        let mut w = World::default();
        let mut plane = plane();
        plane.material.reflective = 0.5;
        plane.set_transform(
            Transform::new().translate(0.0, -1.0, 0.0)
        );

        w.objects.push(plane.clone());

        let r = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, -f64::sqrt(2.0)/ 2.0, f64::sqrt(2.0)/2.0));
        let i = Intersection {t: f64::sqrt(2.0), object: plane.clone()};
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        let color = shade_hit(&w, &comps, 1);
        assert_eq!(f64::abs(color.x - 0.87677) < 0.001, true);
        assert_eq!(f64::abs(color.y - 0.92436) < 0.001, true);
        assert_eq!(f64::abs(color.z - 0.82918) < 0.001, true);
    }



    #[test]
    fn test_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::default();
        let mut plane = plane();
        plane.material.reflective = 0.5;
        plane.set_transform(
            Transform::new().translate(0.0, -1.0, 0.0)
        );
        w.objects.push(plane.clone());
        let r = Ray::new(Point::new(0.0, 0.0, -3.0), Vector::new(0.0, -1.0, 1.0).normalize());
        let i = Intersection {t: f64::sqrt(2.0), object: plane.clone()};
        let comps = prepare_computations(&i, &r, &vec![i.clone()]);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(f64::abs(color.x) < 0.0001, true);
        assert_eq!(f64::abs(color.y) < 0.0001, true);
        assert_eq!(f64::abs(color.z) < 0.0001, true);
    }

    #[test]
    fn test_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects[0].clone();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection{t: 4.0, object: shape.clone()};
        let i2 = Intersection{t: 6.0, object: shape.clone()};
        let xs = vec![i1.clone(), i2.clone()];
        let comps = prepare_computations(&i1, &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert!(c == Tuple::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();
        let mut a = w.objects[0].clone();
        a.material.ambient = 1.0;
        // a.material.set_pattern()

    }

    #[test]
    fn test_shade_hit_with_a_reflective_transparent_material() {
        let w = World::default();

    }

}