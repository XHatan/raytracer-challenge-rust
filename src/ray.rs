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

    #[test]
    fn test_compute_point_from_distance() {
        let r = Ray::new(Tuple { x: 2.0, y: 3.0, z: 4.0, w: 1.0 }, Tuple::new(1.0, 0.0, 0.0, 0.0));
        let position = position_from_ray(r, 0.0);
        assert_eq!(f64::abs(position.x - 2.0) < 0.001, true);
        assert_eq!(f64::abs(position.y - 3.0) < 0.001, true);
        let position2 = position_from_ray(r, 1.0);
        assert_eq!(f64::abs(position2.x - 3.0) < 0.001, true);
    }
}