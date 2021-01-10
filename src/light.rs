use crate::tuple::Tuple;

#[derive(Clone, Copy)]
pub struct PointLight {
    position: Tuple,
    intensity: Tuple
}


pub trait PointLightProperties {
    fn new(position: Tuple, intensity: Tuple) -> PointLight;

    fn position(&self) -> Tuple;

    fn intensity(&self) -> Tuple;
}

impl PointLightProperties for PointLight {
    fn new(position: Tuple, intensity: Tuple) -> PointLight {
        PointLight {position, intensity}
    }

    fn position(&self) -> Tuple {
        self.position.clone()
    }

    fn intensity(&self) -> Tuple {
        self.intensity.clone()
    }
}

mod tests {
    use super::*;


    #[test]
    fn pointlight_construction() {
        let intensity = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let position = Tuple::new(0.0, 0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        let intensity_clone = light.intensity();
        let position_clone = light.position();

        assert_eq!(intensity_clone.x, intensity.x);
        assert_eq!(intensity_clone.y, intensity.y);
        assert_eq!(intensity_clone.z, intensity.z);
        assert_eq!(intensity_clone.w, intensity.w);

        assert_eq!(position_clone.x, position.x);
        assert_eq!(position_clone.y, position.y);
        assert_eq!(position_clone.z, position.z);
        assert_eq!(position_clone.w, position.w);
    }
}
