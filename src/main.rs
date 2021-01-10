pub mod tuple;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod vector;
mod sphere;
mod intersection;
mod light;
mod material;

use canvas::Canvas;
use crate::canvas::CanvasProperties;
use crate::sphere::{Sphere, intersect, hit, SphereProperties};
use crate::tuple::{Tuple, TupleProperties};
use crate::ray::Ray;
use crate::transformation::{Transform, TransformProperty};
use crate::light::{PointLight, PointLightProperties};


fn run_chapter_5() {
    let canvas_width = 100;
    let canvas_height = 100;
    let wall_size: f64 = 10.0;
    let wall_z = 10.0;
    let half = wall_size / 2.0;
    let pixel_size = wall_size / (canvas_width as f64) as f64;
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let color = tuple::Tuple::new(125.0, 0.0, 0.3, 125.0);
    let ray_origin = Tuple::new(0.0, 0.0, -5.0, 1.0);
    for y in 0..canvas_width-1 {
        let world_y = half - pixel_size * y as f64; // from 5 to -5 : top to bottom
        // println!("{}", x); // x: i32
        for x in 0..canvas_height -1 {
            let world_x = -half + pixel_size * (x as f64); // from -5 to 5
            let position = Tuple::new(world_x, world_y, wall_z, 1.0);
            let dir = position - ray_origin;
            let r = Ray::new(ray_origin, dir.normalize());
            let mut transform = transformation::Transform::new();
            let t2 = transform.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
            s.set_transform(t2);
            let xs = intersect(s, r);

            if hit(xs) > 0.0 {
                canvas.write_pixel(x as u32, y as u32, color);
            }
        }
    }
    canvas.to_ppm();
}

fn run_chapter_6() {
    let canvas_width = 100;
    let canvas_height = 100;
    let wall_size: f64 = 10.0;
    let wall_z = 10.0;
    let half = wall_size / 2.0;
    let pixel_size = wall_size / (canvas_width as f64) as f64;
    let mut canvas = Canvas::new(canvas_width, canvas_height);
    let ray_origin = Tuple::new(0.0, 0.0, -5.0, 1.0);

    let light_position = Tuple::new(-10.0, 10.0, -10.0, 1.0);
    let light_color = Tuple::new(1.0, 1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);
    for y in 0..canvas_width-1 {
        let world_y = half - pixel_size * y as f64; // from 5 to -5 : top to bottom

        for x in 0..canvas_height -1 {
            let world_x = -half + pixel_size * (x as f64); // from -5 to 5
            let position = Tuple::new(world_x, world_y, wall_z, 1.0);
            let dir = position - ray_origin;
            let r = Ray::new(ray_origin, dir.normalize());
            let mut transform = transformation::Transform::new();
            let t2 = transform.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
            s.set_transform(t2);
            let xs = intersect(s.clone(), r);

            if hit(xs.clone()) > 0.0 {
                let point = ray::position_from_ray(r, hit(xs.clone()));
                let normal = s.normal_at(point);
                let eye = r.direction() * (-1.0);
                let color = material::lighting(s.material(), light, point, eye, normal);
                canvas.write_pixel(x as u32, y as u32, color);
            }
        }
    }
    canvas.to_ppm();
}

fn main() {
    println!("Hello, world!");
    run_chapter_6();
}
