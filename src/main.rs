mod vector;
mod point;
mod color;
mod matrix;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
// use crate::matrix;

fn main() {
    let a = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vector {
        x: 2.0,
        y: 1.5,
        z: 3.0,
    };
    let c = a.cross(b);
    println!("a cross b = {c}");
    let nc = -c;
    println!("-c = {nc}");

    let p = Point {
        x: -1.0,
        y: 2.0,
        z: -3.0
    };
    let result = p + a;
    println!("p = {p}");
    println!("p + a = {result}");

    println!("White = {}", Color::WHITE);

    let tr = matrix::scale(4.0, 5.0, 6.0);
    println!("Scale = \n{:#}", matrix::MatrixFmt(&tr));
}
