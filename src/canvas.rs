use crate::tuple::{TupleProperties, Tuple};
extern crate image;
use image::{GenericImage, ImageBuffer, RgbImage, Rgb, ColorType};
use image::save_buffer_with_format;
use image::EncodableLayout;
use self::image::{Rgba, RgbaImage};

type Color = Tuple;

pub struct Canvas {
    width: i32,
    height: i32,
    pixels: RgbaImage
}

pub trait CanvasProperties {
    fn new(width: i32, height: i32) -> Canvas;

    fn write_pixel(&mut self, x: u32, y: u32, color: Color);

    fn pixel_at(&self, x: u32, y: u32) -> Color;

    fn to_ppm(&self);
}

impl CanvasProperties for Canvas {
    fn new(width: i32, height: i32) -> Canvas {
        Canvas {width, height, pixels: RgbaImage::new(512, 512)}
    }

    fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let pixel = Rgba([(color.x as u8), (color.y as u8), (color.z as u8), (color.w as u8)]);
        self.pixels.put_pixel(x, y, pixel);
    }

    fn pixel_at(&self, x: u32, y: u32) -> Color {
        if (x as i32) >= self.width || (y as i32) >= self.height {
            let pixel = self.pixels.get_pixel((self.width -1 ) as u32, (self.height - 1) as u32);
            return Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64, pixel[3] as f64);
        }
        let pixel = self.pixels.get_pixel(x, y);
        Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64, pixel[3] as f64)
    }

    fn to_ppm(&self) {
        save_buffer_with_format("output.png", self.pixels.as_bytes(), self.pixels.width(), self.pixels.height(),
                                ColorType::Rgba8, image::ImageFormat::Png).unwrap()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::tuple::{TupleProperties, Point, Tuple};
    use std::ptr::null;

    #[test]
    fn test_color_is_not_a_point() {
        let vector = Color::new(2.0, 3.0, 4.0, 0.0);
        assert_eq!(vector.is_point(), false);
    }

    #[test]
    fn test_color_operations() {
        let vector = Color::new(2.0, 3.0, 4.0, 0.0);
        let color = Color::new(1.0, 2.0, 3.0, 0.0);
        let colorSum = vector + color;
        assert_eq!(f64::abs(colorSum.x - 3.0) < 0.001, true);
    }

    #[test]
    fn test_canvas_construction() {
        let canvas = Canvas::new(20, 20);
    }

    #[test]
    fn test_canvas_get_pixel() {
        let mut canvas = Canvas::new(20, 20);
        canvas.write_pixel(1, 1, Color::new(2.0, 2.0, 2.0, 1.0));

        let p = canvas.pixel_at(1,1);
        assert_eq!(f64::abs(p.x - 2.0) <= 0.01, true);
    }


}