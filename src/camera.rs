use crate::transformation::{Transform, TransformProperty};
use crate::ray::Ray;
use crate::tuple::{Point, PointProperties, VectorProperties};
use crate::world::{World, WorldProperties};
use crate::canvas::{Canvas, CanvasProperties};

#[derive(Clone)]
pub struct Camera {
    // horizontal size
    hsize: f64,
    // vertical size
    vsize: f64,

    // angle that describes how much the camera can see
    field_of_view: f64,

    // how world should be oriented relative to the camera
    transform: Transform,

    // assume canvas is one unit away
    distance_to_canvas: f64,

    pixel_size: f64,

    half_width: f64,

    half_height: f64
}

pub trait CameraProperties {
    // default transform is identity
    fn new(hsize: f64, vsize: f64, fov: f64) -> Camera;

    fn pixel_size(&self) -> f64;

    fn ray_at_pixel(&self, x: usize, y: usize) -> Ray;

    fn set_transform(&mut self, transform: Transform);
}


impl CameraProperties for Camera {
    fn new(hsize: f64, vsize: f64, fov: f64) -> Camera {

        let half_view = f64::tan(fov / 2.0);
        let aspect = hsize / vsize;
        let mut half_width = half_view; // * distance
        let mut half_height = half_width / aspect;
        if aspect < 1.0 {
            half_height = half_view; // * distance
            half_width = half_height * aspect;
        }

        let pixel_size = (half_width * 2.0 ) / hsize;

        Camera {
            hsize,
            vsize,
            field_of_view: fov,
            transform: Transform::new(),
            distance_to_canvas: 1.0,
            pixel_size,
            half_width,
            half_height
        }
    }

    fn pixel_size(&self) -> f64 {
        let half_view = f64::tan(self.field_of_view / 2.0);
        let aspect = self.hsize / self.vsize;
        let mut half_width = half_view * self.distance_to_canvas;
        let mut half_height = half_width / aspect;
        if aspect < 1.0 {
            half_height = half_view * self.distance_to_canvas;
            half_width = half_height * aspect;
        }

        (half_width * 2.0 ) / self.hsize
    }

    fn ray_at_pixel(&self, x: usize, y: usize) -> Ray {
        let pixel_size = self.pixel_size();
        let x_offset = ( x as f64 + 0.5) * pixel_size;
        let y_offset = (y as f64 + 0.5) * pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);
        let dir = (pixel - origin).normalize();

        Ray::new(origin, dir)
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

pub fn render(camera: Camera, world: World) -> Canvas {
    let mut image = Canvas::new(camera.hsize as u32, camera.vsize as u32);
    for y in 0..(camera.vsize - 1.0) as u32 {
        for x in 0..(camera.hsize - 1.0) as u32 {
            let ray = camera.ray_at_pixel(x as usize, y as usize);
            let color = world.color_at_ray(&ray, 1);
            // println!("color {} {} {}", color.x, color.y, color.z);
            image.write_pixel(x, y, color);
        }
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::tuple::Vector;

    #[test]
    fn test_camera_construction() {
        let hsize = 160.0;
        let vsize = 120.0;
        let fov = PI / 2.0;
        let camera = Camera::new(hsize, vsize, fov);
        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
        assert_eq!(camera.field_of_view, fov);
        assert_eq!(camera.transform == Transform::new(), true);
    }

    #[test]
    fn test_camera_pixel_size() {
        let c = Camera::new(200.0, 125.0, PI / 2.0);
        assert_eq!(f64::abs(c.pixel_size - 0.01) < 0.0001, true);

        let c = Camera::new(125.0, 200.0, PI / 2.0);
        assert_eq!(f64::abs(c.pixel_size - 0.01) < 0.0001, true);
    }

    #[test]
    fn test_render_world_with_camera() {
        let c = Camera::new(201.0, 101.0, PI/2.0);
        let r = c.ray_at_pixel(100, 50);
        assert_eq!(r.origin() == Point::new(0.0, 0.0, 0.0), true);
        assert_eq!(r.direction() == Vector::new(0.0, 0.0, -1.0), true);
    }

    #[test]
    fn test_render_world_with_camera_at_corner() {
        let c = Camera::new(201.0, 101.0, PI/2.0);
        let r = c.ray_at_pixel(0, 0);
        assert_eq!(r.origin() == Point::new(0.0, 0.0, 0.0), true);
        assert_eq!(r.direction() == Vector::new(0.66519, 0.33259, -0.66851), true);
    }

    #[test]
    fn test_render_world_with_transformed_camera() {
        let mut c = Camera::new(201.0, 101.0, PI/2.0);
        let mut transform = Transform::new();
        // translate first and rotate next;
        transform = transform.translate(0.0, -2.0, 5.0).rotate_y( PI /4.0);
        c.set_transform(transform);

        let r = c.ray_at_pixel(100, 50);
        assert_eq!(r.origin() == Point::new(0.0, 2.0, -5.0), true);
        assert_eq!(r.direction() == Vector::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0), true);
    }

    // #[test]
    // fn test_render() {
    //     let w = World::new();
    //     let mut c = Camera::new(11.0, 11.0, PI / 2.0);
    //     let from = Point::new(0.0, 0.0, -5.0);
    //     let to = Point::new(0.0, 0.0, 0.0);
    //     let up = Vector::new(0.0, 1.0, 0.0);
    //     c.transform = ViewTransform(from, to, up);
    //     let image = render(c, w);
    //     assert_eq!(image.pixel_at(4, 4).x, 0.38066);
    //
    // }
}

