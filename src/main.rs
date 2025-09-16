mod vector;
mod point;
mod color;
mod matrix;
mod ray;
mod intersection;
mod shape;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::matrix::Matrix;

fn main() {
    let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
}
