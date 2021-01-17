use crate::tuple::{Tuple, TupleProperties, Point, Vector, VectorProperties};
use crate::light::{PointLight, PointLightProperties};

#[derive(Clone, Copy)]
pub struct Material {
    color: Tuple,
    pub(crate) ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64
}

pub trait MaterialProperties {
    // ambient, diffuse and specular typically [0, 1]
    // shininess typically 10 to 200
    fn new(color: Tuple, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Material;

    fn color(&self) -> Tuple;
}

impl MaterialProperties for Material {
    fn new(color: Tuple, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Material {
        Material {color, ambient, diffuse, specular, shininess}
    }

    fn color(&self) -> Tuple {
        self.color.clone()
    }
}

pub fn phong_lighting(m: Material, light: PointLight, intersect_point: Point, eyev: Vector, normalv: Vector, in_shadow: bool) -> Tuple {
    let black = Tuple::new(0.0, 0.0, 0.0 ,0.0);
    let effective_color = m.color().hadamard_product(light.intensity());
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
        let result = phong_lighting(m, light, p, eyev, normalv, false);
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
        let result = phong_lighting(m, light, p, eyev, normalv, false);
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
        let result = phong_lighting(m, light, p, eyev, normalv, false);
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
        let result = phong_lighting(m, light, p, eyev, normalv, in_shadow);
        // should just be 0.1 * 1.0
        assert_eq!(result == Tuple::new(0.1, 0.1, 0.1, 0.1), true);
    }
}


