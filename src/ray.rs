use crate::tuple::{Point, Vector};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {origin, direction}
    }

    pub fn origin(&self) -> Point {
        self.origin.clone()
    }

    pub fn direction(&self) -> Vector {
        self.direction.clone()
    }

    pub fn position_at(&self, distance: f64) -> Point {
        self.origin() + self.direction() * distance
    }
}

mod tests {
    use super::*;
    use crate::transformation::Transform;
    use crate::transformation::TransformProperty;
    use crate::tuple::{PointProperties, VectorProperties};

    #[test]
    fn test_compute_point_from_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        // let position = position_from_ray(&r, 0.0);
        let position = r.position_at(0.0);
        assert_eq!(f64::abs(position.x() - 2.0) < 0.001, true);

        assert_eq!(f64::abs(position.y() - 3.0) < 0.001, true);
        // let position2 = position_from_ray(&r, 1.0);
        let position2 = r.position_at(1.0);
        assert_eq!(f64::abs(position2.x() - 3.0) < 0.001, true);

    }

    #[test]
    fn test_translate_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let mut t = Transform::new();
        let translation = t.translate(3.0, 4.0, 5.0);
        let new_ray = translation * r;
        // assert_eq!(f64::abs(new_ray.origin().x - 4.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().y - 6.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().z - 8.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().w - 1.0) < 0.01, true);

        assert_eq!(new_ray.origin() == Point::new(4.0, 6.0, 8.0), true);

        // assert_eq!(f64::abs(new_ray.direction().x) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().y - 1.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().z) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().w) < 0.01, true);
        assert_eq!(new_ray.direction() == Vector::new(0.0, 1.0, 0.0), true);
    }

    #[test]
    fn test_translate_ray_2() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let mut t = Transform::new();
        let translation = t.scaling(2.0, 3.0, 4.0);
        let new_ray = translation * r;
        // assert_eq!(f64::abs(new_ray.origin().x - 2.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().y - 6.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().z - 12.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.origin().w - 1.0) < 0.01, true);

        assert_eq!(new_ray.origin() == Point::new(2.0, 6.0, 12.0), true);
        //
        // assert_eq!(f64::abs(new_ray.direction().x) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().y - 3.0) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().z) < 0.01, true);
        // assert_eq!(f64::abs(new_ray.direction().w) < 0.01, true);
        assert_eq!(new_ray.direction() == Vector::new(0.0, 3.0, 0.0), true);
    }
}