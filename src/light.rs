use crate::tuple::{Tuple, Point};

#[derive(Clone, Copy)]
pub struct PointLight {
    position: Point,
    intensity: Tuple
}


pub trait PointLightProperties {
    fn new(position: Point, intensity: Tuple) -> PointLight;

    fn position(&self) -> Point;

    fn intensity(&self) -> Tuple;
}

impl PointLightProperties for PointLight {
    fn new(position: Point, intensity: Tuple) -> PointLight {
        PointLight {position, intensity}
    }

    fn position(&self) -> Point {
        self.position.clone()
    }

    fn intensity(&self) -> Tuple {
        self.intensity.clone()
    }
}

mod tests {
    use super::*;
    use crate::tuple::PointProperties;


    #[test]
    fn pointlight_construction() {
        let intensity = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        let intensity_clone = light.intensity();
        let position_clone = light.position();

        assert_eq!(intensity == intensity_clone, true);
        assert_eq!(position_clone == position, true);
    }
}
