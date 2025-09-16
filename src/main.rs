mod vector;
mod point;
mod color;
mod matrix;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::matrix::Matrix;

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

    let tr = Matrix::<4>::translate(2.0, 3.5, -1.1);
    println!("Translation:\n{}", tr);
    let s = Matrix::<4>::scale(0.2, 0.3, 0.4);
    println!("Scale:\n{}", s);
    let transform = tr * s;
    println!("Transform:\n{}", transform);
    let inv = transform.inverse();
    println!("Inverse:\n{}", inv);
}
