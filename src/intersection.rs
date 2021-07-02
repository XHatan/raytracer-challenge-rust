
use crate::tuple::{VectorProperties, Point, Vector};
use crate::ray::{Ray};
use crate::shape_props::ShapeProperties;

#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn ShapeProperties
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        return self.object.material() == other.object.material()
        && self.object.transform() == other.object.transform()
        && self.t == other.t;
    }
}

#[derive(Clone)]
pub struct AugIntersection<'a> {
    pub t: f64,
    pub object:&'a dyn ShapeProperties,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
    // the reflected ray direction
    pub reflectv: Vector,
    pub n1: f64,
    pub n2: f64,
    pub under_point: Point
}

impl PartialEq for AugIntersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.object.material() == other.object.material()
            && self.object.transform() == other.object.transform()
            && self.t == other.t
            && self.point == other.point
            && self.eyev == other.eyev
        && self.normalv == other.normalv
        && self.inside == other.inside
        && self.over_point == other.over_point
        && self.reflectv == other.reflectv
        && self.n1 == other.n1
        && self.n2 == other.n2
        && self.under_point == other.under_point;
    }
}

pub(crate) const EPSILON: f64 = 0.00001;

pub fn schlick(comps: &AugIntersection) -> f64 {
    let mut cos = comps.eyev.dot(comps.normalv);
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1.0 - cos * cos);
        if sin2_t > 1.0 {
            return 1.0;
        }
        let cos_t = f64::sqrt(1.0 - sin2_t);

        cos = cos_t;
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powf(2.0);

    return r0 + (1.0 - r0) * (1.0 - cos).powf(5.0);
}

pub fn prepare_computations<'a>(hit: &'a Intersection, r: &'a Ray, xs: &'a Vec<Intersection>) -> AugIntersection<'a> {
    let mut containers: Vec<&dyn ShapeProperties> = vec![];

    let mut n1 = 1.0;
    let mut n2 = 1.0;
    for item in xs {
        if item == hit {
            if containers.len() != 0 {
                n1 = containers.last().unwrap().material().refractive_index;
            }
        }

        let index_of_item_obj = containers.iter().position(|x|
            *x as *const _ == item.object as *const _
        );

        if index_of_item_obj == None {
            containers.push(item.clone().object);
        } else {
            containers.remove(index_of_item_obj.unwrap());
        }


        if item == hit {
            if containers.len() != 0 {
                n2 = containers.last().unwrap().material().refractive_index;
            }

            break;
        }
    }

    let point = r.position_at(hit.t);
    let mut normalv = hit.object.normal_at(point);
    let eyev =  r.direction() * (-1.0);
    let mut inside: bool = false;
    // obtuse angle
    if eyev.dot(normalv) < 0.0 {
        inside = true;
        normalv = -1.0 * normalv;
    }
    let over_point = point + normalv * EPSILON;
    let under_point = point - normalv * EPSILON;
    let reflectv = r.direction().reflect(normalv);

    AugIntersection {
        t: hit.t,
        object: hit.object,
        point,
        eyev,
        normalv,
        inside,
        over_point,
        reflectv,
        n1,
        n2,
        under_point
    }
}

mod tests {
    use super::*;
    use crate::tuple::{Point, PointProperties, Vector, VectorProperties};
    use crate::transformation::{Transform, TransformProperty};
    use crate::material::float_eq;
    use crate::sphere::sphere;
    use crate::sphere;
    use crate::plane;

    #[test]
    fn intersection_on_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = Intersection {t: 4.0, object: &shape};
        let vec = vec![i.clone()];
        let comps = prepare_computations(&i, &r, &vec);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn intersection_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = Intersection {t: 1.0, object: &shape};
        let vec = vec![i.clone()];
        let comps = prepare_computations(&i, &r, &vec);
        // let true_p = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(comps.point == Point::new(0.0, 0.0, 1.0), true);
        assert_eq!(comps.eyev == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.normalv == Vector::new(0.0, 0.0, -1.0), true);
    }

    #[test]
    fn test_precomputing_the_reflection_vector() {
        let shape = plane::plane();
        let r = Ray::new(Point::new(0.0, 1.0, -1.0), Vector::new(0.0, - f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0));
        let i = Intersection { t: f64::sqrt(2.0), object: &shape };
        let vec = vec![i.clone()];
        let comps = prepare_computations(&i, &r, &vec);
        assert_eq!(comps.reflectv == Vector::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0), true);
    }

    #[test]
    fn test_finding_n1_and_n2_at_various_intersections() {
        let mut a = sphere::glass_sphere();
        a.transform = Transform::new().scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;
        let mut b = sphere::glass_sphere();
        b.transform = Transform::new().translate(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;
        let mut c = sphere::glass_sphere();
        c.transform = Transform::new().translate(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let mut xs: Vec<Intersection> = vec![];
        xs.push(Intersection{t: 2.0, object: &a});
        xs.push(Intersection{t: 2.75, object: &b});
        xs.push(Intersection{t: 3.25, object: &c});
        xs.push(Intersection{t: 4.75, object: &b});
        xs.push(Intersection{t: 5.25, object: &c});
        xs.push(Intersection{t: 6.0, object: &a});

        let comps0 = prepare_computations(&xs[0], &r, &xs);
        assert_eq!(float_eq(comps0.n1, 1.0), true);
        assert_eq!(float_eq(comps0.n2, 1.5), true);

        let comps1 = prepare_computations(&xs[1], &r, &xs);
        assert_eq!(float_eq(comps1.n1, 1.5), true);
        assert_eq!(float_eq(comps1.n2, 2.0), true);

        let comps2 = prepare_computations(&xs[2], &r, &xs);
        assert_eq!(float_eq(comps2.n1, 2.0), true);
        assert_eq!(float_eq(comps2.n2, 2.5), true);

        let comps3 = prepare_computations(&xs[3], &r, &xs);
        assert_eq!(float_eq(comps3.n1, 2.5), true);
        assert_eq!(float_eq(comps3.n2, 2.5), true);

        let comps4 = prepare_computations(&xs[4], &r, &xs);
        assert_eq!(float_eq(comps4.n1, 2.5), true);
        assert_eq!(float_eq(comps4.n2, 1.5), true);
    }

    #[test]
    fn test_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = sphere::glass_sphere();
        shape.transform  = Transform::new().translate(0.0, 0.0, 1.0);
        let i = Intersection {t: 5.0, object: &shape};
        let xs = vec![i.clone()];
        let comps = prepare_computations(&i, &r, &xs);
        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn test_schlink_approximation_under_total_internal_reflection() {
        let shape = sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection{t: -f64::sqrt(2.0) / 2.0, object: &shape};
        let i2 = Intersection{t: f64::sqrt(2.0) / 2.0, object: &shape};

        let xs = vec![i1.clone(), i2.clone()];
        let comps = prepare_computations(&xs[1], &r, &xs);
        let reflectance = schlick(&comps);
        // total internal reflection, no refraction
        assert!(float_eq(reflectance, 1.0));
    }

    #[test]
    fn test_the_schlink_approximation_with_a_perpendicular_viewing_angle() {
        let shape = sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection{t: -1.0, object: &shape};
        let i2 = Intersection{t: 1.0, object: &shape};

        let xs = vec![i1.clone(), i2.clone()];
        let comps = prepare_computations(&xs[1], &r, &xs);
        let reflectance = schlick(&comps);
        // small reflection when hitting perpendically to a transparent surface
        assert!(float_eq(reflectance, 0.04));
    }

    #[test]
    fn test_the_schlick_approximation_with_small_angle() {
        let shape = sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection{t: 1.8589, object: &shape};

        let xs = vec![i1.clone()];
        let comps = prepare_computations(&xs[0], &r, &xs);
        let reflectance = schlick(&comps);
        assert!(float_eq(reflectance, 0.48873));
    }
}