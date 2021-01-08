use crate::tuple::{Point, Tuple};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {origin, direction}
    }

    pub fn origin(&self) -> Tuple {
        self.origin.clone()
    }

    pub fn direction(&self) -> Tuple {
        self.direction.clone()
    }
}



fn position_from_ray(r: Ray, distance: f64) -> Tuple {
    r.origin() + r.direction() * distance
}

mod tests {
    use super::*;
    use crate::transformation::Transform;
    use crate::transformation::TransformProperty;

    #[test]
    fn test_compute_point_from_distance() {
        let r = Ray::new(Tuple { x: 2.0, y: 3.0, z: 4.0, w: 1.0 }, Tuple::new(1.0, 0.0, 0.0, 0.0));
        let position = position_from_ray(r, 0.0);
        assert_eq!(f64::abs(position.x - 2.0) < 0.001, true);
        assert_eq!(f64::abs(position.y - 3.0) < 0.001, true);
        let position2 = position_from_ray(r, 1.0);
        assert_eq!(f64::abs(position2.x - 3.0) < 0.001, true);
    }

    #[test]
    fn test_translate_ray() {
        let r = Ray::new(Tuple {x: 1.0, y: 2.0, z: 3.0, w: 1.0}, Tuple::new(0.0, 1.0, 0.0, 0.0));
        let mut t = Transform::new();
        let translation = t.translate(3.0, 4.0, 5.0);
        let new_ray = translation * r;
        assert_eq!(f64::abs(new_ray.origin().x - 4.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().y - 6.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().z - 8.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().w - 1.0) < 0.01, true);

        assert_eq!(f64::abs(new_ray.direction().x) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().y - 1.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().z) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().w) < 0.01, true);
    }

    #[test]
    fn test_translate_ray_2() {
        let r = Ray::new(Tuple {x: 1.0, y: 2.0, z: 3.0, w: 1.0}, Tuple::new(0.0, 1.0, 0.0, 0.0));
        let mut t = Transform::new();
        let translation = t.scaling(2.0, 3.0, 4.0);
        let new_ray = translation * r;
        assert_eq!(f64::abs(new_ray.origin().x - 2.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().y - 6.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().z - 12.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.origin().w - 1.0) < 0.01, true);

        assert_eq!(f64::abs(new_ray.direction().x) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().y - 3.0) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().z) < 0.01, true);
        assert_eq!(f64::abs(new_ray.direction().w) < 0.01, true);
    }
}