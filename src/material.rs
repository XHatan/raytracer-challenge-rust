use crate::tuple::{Tuple, TupleProperties, Point, Vector, VectorProperties};
use crate::light::{PointLight, PointLightProperties};
use crate::pattern::{black_pattern, Pattern};
use crate::shape::Shape;

#[derive(Clone)]
pub struct Material {
    color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    has_pattern: bool,
    pattern: Pattern
}

pub fn float_eq(a: f64, b: f64) -> bool {
    return f64::abs(a - b) < 0.00001;
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color && f64::abs(self.ambient - other.ambient) < 0.00001 && float_eq(self.diffuse, other.specular)
        && float_eq(self.shininess, other.shininess)
        && float_eq(self.reflective, other.reflective)
        && float_eq(self.transparency, other.transparency)
        && float_eq(self.refractive_index, other.refractive_index)
        && self.has_pattern == other.has_pattern

    }
}
pub trait MaterialProperties {
    // ambient, diffuse and specular typically [0, 1]
    // shininess typically 10 to 200
    fn new(color: Tuple, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Material;

    // fn color(&self, point: Point) -> Tuple;

    fn default() -> Material;

    fn set_pattern(&mut self, pattern: &Pattern);

    fn color_at_object(&self, shape: &Shape, point: Point) -> Tuple;
}

impl MaterialProperties for Material {
    fn new(color: Tuple, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
            transparency: 0.0,
            refractive_index: 1.0,
            reflective: 0.0,
            has_pattern: false,
            pattern: black_pattern()
        }
    }

    // fn color(&self, point: Point) -> Tuple {
    //     match self.has_pattern {
    //         true => self.pattern.color_at(point),
    //         false => self.color.clone()
    //     }
    // }

    fn default() -> Material {
        Material {color: Tuple::new(1.0, 1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            has_pattern: false,
            pattern: black_pattern()
        }
    }

    fn set_pattern(&mut self, pattern: &Pattern) {
        self.has_pattern = true;
        self.pattern = pattern.clone();
    }

    fn color_at_object(&self, shape: &Shape, point: Point) -> Tuple {
        match self.has_pattern {
            true => self.pattern.color_at_object(shape, point),
            false => self.color.clone()
        }
    }
}

// intersect_point: world coord
pub fn phong_lighting(m: &Material, light: PointLight, intersect_point: Point, eyev: Vector, normalv: Vector, in_shadow: bool, shape: &Shape) -> Tuple {
    let black = Tuple::new(0.0, 0.0, 0.0 ,0.0);
    let effective_color = m.color_at_object(shape, intersect_point).hadamard_product(light.intensity());
    // A = L_a * M_a;
    let ambient = effective_color * m.ambient;
    if in_shadow {
       return ambient;
    }

    // D = L_d * M_d * (L_dir.dot(normal))
    let light_direction = (light.position() - intersect_point).normalize();
    let light_dot_normal = light_direction.dot(normalv);
    let diffuse: Tuple;
    let specular: Tuple;
    if light_dot_normal < 0.0 {
        // light is on the other side of the surface
        diffuse = black;
        specular = black;
    } else {
        diffuse = effective_color * m.diffuse * light_dot_normal;
        let reflectv = (-1.0 * light_direction).reflect(normalv);
        let reflectv_dot_eye = reflectv.dot(eyev);
        if reflectv_dot_eye <= 0.0 {
            specular = black;
        } else {
            let factor = reflectv_dot_eye.powf(m.shininess);
            specular = light.intensity() * m.specular * factor;
        }
    }

    ambient + diffuse + specular

}

mod tests {
    use super::*;
    use crate::light::PointLightProperties;
    use crate::tuple::{PointProperties, VectorProperties};
    use crate::pattern::stripe_pattern;
    use crate::shape::sphere;
    use crate::transformation::{Transform, TransformProperty};

    #[test]
    fn material_construction() {
        let red = Tuple::new(128.0, 0.0, 0.0, 128.0);
        let m = Material::new(red, 0.1, 0.9, 0.9,200.0);
    }

    #[test]
    fn test_lighting() {
        let m = Material::new(Tuple::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 0.0));
        let p = Point::new(0.0, 0.0, 0.0);
        let result = phong_lighting(&m, light, p, eyev, normalv, false, &sphere());
        assert_eq!(f64::abs(result.x - 1.9) < 0.01, true);
        assert_eq!(f64::abs(result.y - 1.9) < 0.01, true);
    }

    #[test]
    fn test_lighting_2() {
        let m = Material::new(Tuple::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 0.0));
        let p = Point::new(0.0, 0.0, 0.0);
        let result = phong_lighting(&m, light, p, eyev, normalv, false, &sphere());
        assert_eq!(f64::abs(result.x - 0.7364) < 0.01, true);
        assert_eq!(f64::abs(result.y - 0.7364) < 0.01, true);
        assert_eq!(f64::abs(result.z - 0.7364) < 0.01, true);
    }

    #[test]
    fn test_lighting_3() {
        let m = Material::new(Tuple::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0);
        let eyev = Vector::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 0.0));
        let p = Point::new(0.0, 0.0, 0.0);
        let result = phong_lighting(&m, light, p, eyev, normalv, false, &sphere());
        assert_eq!(f64::abs(result.x - 1.6364) < 0.01, true);
        assert_eq!(f64::abs(result.y - 1.6364) < 0.01, true);
        assert_eq!(f64::abs(result.z - 1.6364) < 0.01, true);
    }

    #[test]
    fn test_lighting_the_surface_in_shadow() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 1.0));
        let in_shadow = true;
        let m = Material::new(Tuple::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0);
        let p = Point::new(0.0, 0.0, 0.0);
        let result = phong_lighting(&m, light, p, eyev, normalv, in_shadow, &sphere());
        // should just be 0.1 * 1.0
        assert_eq!(result == Tuple::new(0.1, 0.1, 0.1, 0.1), true);
    }


    #[test]
    fn lighting_with_a_pattern_applied() {
        let pattern = stripe_pattern(Tuple::new(1.0, 1.0, 1.0, 1.0), Tuple::new(0.0, 0.0, 0.0, 0.0));

        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 1.0));

        let mut m = Material::new(Tuple::new(1.0, 1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0);
        m.set_pattern(&pattern);
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;

        let p = Point::new(0.9, 0.0, 0.0);
        let result1 = phong_lighting(&m, light, p, eyev, normalv, false, &sphere());

        let result2 = phong_lighting(&m, light, Point::new(1.1, 0.0, 0.0), eyev, normalv, false, &sphere());
        assert_eq!(result1 == Tuple::new(1.0, 1.0, 1.0, 1.0), true);
        assert_eq!(result2 == Tuple::new(0.0, 0.0, 0.0, 0.0), true);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = sphere();
        object.set_transform(
            Transform::new().scaling(2.0, 2.0, 2.0)
        );
        let white = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let black = Tuple::new(0.0, 0.0, 0.0, 0.0);
        let pattern = stripe_pattern(white, black);
        let c = pattern.color_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c == white, true);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let mut object = sphere();
        let white = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let black = Tuple::new(0.0, 0.0, 0.0, 0.0);
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(
          &Transform::new().scaling(2.0, 2.0, 2.0)
        );
        let c = pattern.color_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c == white, true);
    }

    #[test]
    fn stripes_with_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.set_transform(
          Transform::new().scaling(2.0, 2.0, 2.0)
        );
        let white = Tuple::new(1.0, 1.0, 1.0, 1.0);
        let black = Tuple::new(0.0, 0.0, 0.0, 0.0);
        let mut pattern = stripe_pattern(white, black);
        pattern.set_transform(
            &Transform::new().translate(0.5, 0.0, 0.0)
        );

        let c = pattern.color_at_object(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c == white, true);
    }

    #[test]
    fn test_reflectivity_for_the_default_material() {
        let m = Material::default();
        assert_eq!(f64::abs(m.reflective) < 0.001, true);
    }
}


