pub mod tuple;
mod canvas;
mod matrix;
mod transformation;
mod ray;
mod intersection;
mod light;
mod material;
mod world;
mod camera;
mod shape;
mod pattern;

use canvas::Canvas;
use crate::canvas::CanvasProperties;
use crate::tuple::{Tuple, TupleProperties, Point, PointProperties, VectorProperties, Vector};
use crate::ray::Ray;
use crate::transformation::{Transform, TransformProperty, ViewTransform};
use crate::light::{PointLight, PointLightProperties};
use crate::material::{Material, MaterialProperties};
use std::f64::consts::PI;
use crate::world::{World, WorldProperties};
use crate::camera::{Camera, CameraProperties, render};
use crate::pattern::gradient_pattern;

// fn run_chapter_5() {
//     let canvas_width = 100;
//     let canvas_height = 100;
//     let wall_size: f64 = 10.0;
//     let wall_z = 10.0;
//     let half = wall_size / 2.0;
//     let pixel_size = wall_size / (canvas_width as f64) as f64;
//     let mut canvas = Canvas::new(canvas_width, canvas_height);
//     let color = tuple::Tuple::new(125.0, 0.0, 0.3, 125.0);
//     let ray_origin = Point::new(0.0, 0.0, -5.0);
//     let mut transform = transformation::Transform::new();
//     let t2 = transform.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
//     let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
//     s.set_transform(t2);
//
//     for y in 0..canvas_width-1 {
//         let world_y = half - pixel_size * y as f64; // from 5 to -5 : top to bottom
//         // println!("{}", x); // x: i32
//         for x in 0..canvas_height -1 {
//             let world_x = -half + pixel_size * (x as f64); // from -5 to 5
//             let position = Point::new(world_x, world_y, wall_z);
//             let dir = position - ray_origin;
//             let r = Ray::new(ray_origin, dir.normalize());
//
//             let xs = intersect_sphere(&s, r);
//
//             if hit(xs).t > 0.0 {
//                 canvas.write_pixel(x as u32, y as u32, color);
//             }
//         }
//     }
//     canvas.to_ppm();
// }

// fn run_chapter_6() {
//     let canvas_width = 100;
//     let canvas_height = 100;
//     let wall_size: f64 = 10.0;
//     let wall_z = 10.0;
//     let half = wall_size / 2.0;
//     let pixel_size = wall_size / (canvas_width as f64) as f64;
//     let mut canvas = Canvas::new(canvas_width, canvas_height);
//     let ray_origin = Point::new(0.0, 0.0, -5.0);
//
//     let light_position = Tuple::new(-10.0, 10.0, -10.0, 1.0);
//     let light_color = Tuple::new(1.0, 1.0, 1.0, 1.0);
//     let light = PointLight::new(light_position, light_color);
//
//     let mut transform = transformation::Transform::new();
//     // let t2 = transform.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
//     // let t2 = transform.scaling(1.)
//     let mut s = Sphere::new(Tuple::new(0.0, 0.0, 0.0, 1.0), 1.0);
//     // s.set_transform(t2);
//
//     for y in 0..canvas_width-1 {
//         let world_y = half - pixel_size * y as f64; // from 5 to -5 : top to bottom
//
//         for x in 0..canvas_height -1 {
//             let world_x = -half + pixel_size * (x as f64); // from -5 to 5
//             let position = Point::new(world_x, world_y, wall_z);
//             let dir = position - ray_origin;
//             let r = Ray::new(ray_origin, dir.normalize());
//
//
//             let xs = intersect_sphere(&s, r);
//             if hit(xs.clone()).t > 0.0 {
//                 let point = r.position_at(hit(xs.clone()).t);
//                 let normal = s.normal_at(point);
//                 let eye = r.direction() * (-1.0);
//                 let color = material::phong_lighting(s.material(), light, point, eye.data, normal);
//                 canvas.write_pixel(x as u32, y as u32, color);
//             }
//         }
//     }
//     canvas.to_ppm();
// }

fn run_chapter_7() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let mut floor = shape::plane();
    floor.set_transform(Transform::new().scaling(10.0, 0.01, 10.0));
    let sphere_material = Material::new(
        Tuple::new(1.0 * 250.0, 0.9 * 250.0, 0.9 * 250.0, 255.0), 0.1, 0.7, 0.0, 200.0);

    floor.set_material(&sphere_material);

    let mut left_wall = shape::sphere();
    let transform = Transform::new().scaling(10.0, 0.01, 10.0)
        .rotate_x(PI/2.0)
        .rotate_y(-PI/4.0)
        .translate(0.0, 0.0, 5.0);
    left_wall.set_transform(transform);
    left_wall.set_material(&sphere_material);


    let mut right_wall = shape::sphere();
    right_wall.set_transform(
        Transform::new()
            .scaling(10.0, 0.01, 10.0)
            .rotate_x(PI/2.0)
            .rotate_y(PI/4.0)
            .translate(0.0, 0.0, 5.0)
    );

    right_wall.set_material(
        &sphere_material
    );

    let mut middle = shape::sphere();
    middle.set_transform(
        Transform::new()
            .translate(-0.5, 1.0, 0.5)
    );
    middle.set_material(
        &Material::new(
            Tuple::new(0.1 * 250.0, 1.0 * 250.0, 0.5 * 250.0, 255.0),
            0.1, 0.7, 0.3, 200.0

        )
    );

    let mut right = shape::sphere();
    right.set_transform(
        Transform::new()
            .scaling(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5)
    );
    right.set_material(
        &Material::new(
            Tuple::new(0.5 * 250.0, 1.0 * 250.0, 0.2 * 250.0, 255.0),
            0.1, 0.7, 0.3, 200.0

        )
    );

    let mut world = World::new();
    world.objects.clear();
    world.objects = vec![floor, left_wall, right_wall, middle, right];
    world.light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 1.0));

    let mut camera = Camera::new(200.0, 100.0, PI / 2.0);
    camera.set_transform(
        ViewTransform(Point::new(0.0, 1.5, -5.0),
                      Point::new(0.0, 1.0, 0.0),
                        Vector::new(0.0, 1.0, 0.0)
        )
    );

    let mut canvas = render(camera, world);
    let file: &str = "output.png";
    canvas.to_ppm(file);
}

fn run_chapter_8() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let mut floor = shape::plane();
    floor.set_transform(Transform::new().translate(-0.5, 0.5, 1.0));
    let sphere_material = Material::new(
        Tuple::new(0.8 * 250.0, 0.7 * 250.0, 0.7 * 250.0, 255.0), 0.1, 0.7, 0.0, 200.0);

    floor.set_material(&sphere_material);

    let mut left_wall = shape::sphere();
    let transform = Transform::new().scaling(10.0, 0.01, 10.0)
        .rotate_x(PI/2.0)
        .rotate_y(-PI/4.0)
        .translate(0.0, 0.0, 5.0);
    left_wall.set_transform(transform);
    left_wall.set_material(&sphere_material);


    let mut right_wall = shape::sphere();
    right_wall.set_transform(
        Transform::new()
            .scaling(10.0, 0.01, 10.0)
            .rotate_x(PI/2.0)
            .rotate_y(PI/4.0)
            .translate(0.0, 0.0, 5.0)
    );

    right_wall.set_material(
        &sphere_material
    );

    let mut middle = shape::sphere();
    middle.set_transform(
        Transform::new()
            .translate(-0.5, 1.0, 0.5)
    );
    middle.set_material(
        &Material::new(
            Tuple::new(0.1 * 250.0, 1.0 * 250.0, 0.5 * 250.0, 255.0),
            0.1, 0.7, 0.4, 200.0
        )
    );

    let mut right = shape::sphere();
    right.set_transform(
        Transform::new()
            .scaling(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5)
    );
    right.set_material(
        &Material::new(
            Tuple::new(0.5 * 250.0, 1.0 * 250.0, 0.2 * 250.0, 255.0),
            0.1, 0.7, 0.3, 200.0

        )
    );

    let mut world = World::new();
    world.objects.clear();
    world.objects = vec![floor, left_wall, right_wall, middle, right];
    world.light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 1.0));

    let mut camera = Camera::new(200.0, 100.0, PI / 2.0);
    camera.set_transform(
        ViewTransform(Point::new(0.0, 1.5, -5.0),
                      Point::new(0.0, 1.0, 0.0),
                      Vector::new(0.0, 1.0, 0.0)
        )
    );

    let mut canvas = render(camera, world);
    let file: &str = "output.png";
    canvas.to_ppm(file);
}


fn run_chapter_9() {
    let mut floor = shape::plane();
    floor.set_transform(Transform::new().translate(0.0, 0.0, 1.0));
    let mut sphere_material = Material::new(
        Tuple::new(0.8 * 250.0, 0.7 * 250.0, 0.7 * 250.0, 255.0), 0.1, 0.7, 0.0, 200.0);

    sphere_material.transparency = 0.5;
    sphere_material.reflective = 0.7;
    let pattern = gradient_pattern(Tuple::new(0.8 * 250.0, 0.7 * 250.0, 0.7 * 250.0, 255.0),
                                   Tuple::new(0.2 * 250.0, 0.3 * 250.0, 0.3 * 250.0, 255.0));
    sphere_material.set_pattern(&pattern);

    floor.set_material(&sphere_material);

    let mut left_wall = shape::plane();
    let transform = Transform::new()
        .rotate_x(PI/2.0)
        .rotate_y(-PI/4.0)
        .translate(0.0, 0.0, 5.0);
    left_wall.set_transform(transform);
    sphere_material.reflective = 0.1;
    sphere_material.transparency = 0.0;
    left_wall.set_material(&sphere_material);


    let mut right_wall = shape::plane();
    right_wall.set_transform(
        Transform::new()
            .scaling(10.0, 0.01, 10.0)
            .rotate_x(PI/2.0)
            .rotate_y(PI/4.0)
            .translate(0.0, 0.0, 5.0)
    );

    right_wall.set_material(
        &sphere_material
    );

    let mut middle = shape::sphere();
    middle.set_transform(
        Transform::new()
            .translate(-0.5, 1.0, 0.5)
    );
    let mut middle_material = Material::new(
        Tuple::new(0.1 * 250.0, 1.0 * 250.0, 0.5 * 250.0, 255.0),
        0.1, 0.7, 0.4, 200.0
    );
    middle_material.transparency = 0.5;
    middle_material.reflective = 0.7;

    middle.set_material(
        &middle_material
    );

    let mut right = shape::sphere();
    right.set_transform(
        Transform::new()
            .scaling(0.5, 0.5, 0.5)
            .translate(1.5, 0.5, -0.5)
    );

    let mut right_material = Material::new(
        Tuple::new(0.7 * 250.0, 0.7 * 250.0, 0.2 * 250.0, 255.0),
        0.1, 0.7, 0.3, 200.0
    );
    right_material.reflective = 1.0;
    right_material.transparency = 1.0;
    right.set_material(
        &right_material
    );

    let mut world = World::new();
    world.objects.clear();
    world.objects = vec![floor, left_wall, right_wall, middle, right];
    world.light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Tuple::new(1.0, 1.0, 1.0, 1.0));

    let mut camera = Camera::new(400.0, 200.0, PI / 2.0);
    camera.set_transform(
        ViewTransform(Point::new(0.0, 1.5, -5.0),
                      Point::new(0.0, 1.0, 0.0),
                      Vector::new(0.0, 1.0, 0.0)
        )
    );

    let mut canvas = render(camera, world);
    let file: &str = "output.png";
    canvas.to_ppm(file);
}

fn main() {
    println!("Started!");
    // run_chapter_6();
    run_chapter_9();
    println!("Finished!");
}
