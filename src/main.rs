mod vector;
mod point;
mod color;
mod matrix;
mod ray;
mod intersection;
mod shape;
mod img;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::img::Img;

fn main() {
    let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;

    let mut img = Img::new(1920, 1080);
    for y in 100..300 {
        for x in 400..1600 {
            img.set(x, y, Color::RED);
        }
    }
    img.save("renders/image.png");
}
