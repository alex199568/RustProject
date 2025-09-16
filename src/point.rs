
use std::ops::Add;
use std::ops::Sub;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::vector::Vector;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, v: Vector) -> Point {
        Point {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, v: Vector) -> Point {
        Point {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Display for Point {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {}, {})", self.x, self.y, self.z)
    }
}
