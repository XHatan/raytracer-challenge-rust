pub mod tuple;
mod canvas;
use canvas::Canvas;
use crate::canvas::CanvasProperties;

fn main() {
    println!("Hello, world!");
    let canvas = Canvas::new(50, 50);
    canvas.to_ppm();
}
