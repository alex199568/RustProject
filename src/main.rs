
use std::ops::Add;

struct Vector {
    x: f64,
    y: f64,
    z: f64
}

macro_rules! vector {
    ($x:expr, $y:expr, $z:expr) => {
        Vector { x: $x as f64, y: $y as f64, z: $z as f64 }
    };
}

impl Vector {

    fn print(&self) {
        println!("Vector({} {} {})", self.x, self.y, self.z);
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        vector!(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

fn main() {
    let a = vector!(1, 2, 3);
    let b = vector!(1, 2.3, 4);
    let sum = a + b;
    sum.print();
}
