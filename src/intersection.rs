

use crate::sphere::{Sphere, SphereProperties};
use crate::tuple::{Tuple, TupleProperties, VectorProperties, Point, Vector};
use crate::ray::{Ray};

#[derive(Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere
}

#[derive(Clone)]
pub struct AugIntersection {
    pub t: f64,
    pub object: Sphere,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point
}

pub struct Intersections {
    data: Vec<Intersection>
}

const EPSILON: f64 = 0.00001;

pub fn prepare_computations(intersection: &Intersection, r: &Ray) -> AugIntersection {
    let point = r.position_at(intersection.t);
    let mut normalv = intersection.object.normal_at(point);
    let eyev =  r.direction() * (-1.0);
    let mut inside: bool = false;
    // obtuse angle
    if eyev.dot(normalv) < 0.0 {
        inside = true;
        normalv = -1.0 * normalv;
    }
    let over_point = point + normalv * EPSILON;
    AugIntersection {
        t: intersection.t,
        object: intersection.object.clone(),
        point,
        eyev,
        normalv,
        inside,
        over_point
    }
}


mod tests {
    use super::*;
    use crate::tuple::{Point, PointProperties, Vector, VectorProperties};

    #[test]
    fn intersection_on_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let i = Intersection {t: 4.0, object: shape};
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn intersection_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let i = Intersection {t: 1.0, object: shape};
        let comps = prepare_computations(&i, &r);
        // let true_p = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(comps.point == Point::new(0.0, 0.0, 1.0), true);
        assert_eq!(comps.eyev == Vector::new(0.0, 0.0, -1.0), true);
        assert_eq!(comps.normalv == Vector::new(0.0, 0.0, -1.0), true);
    }
}