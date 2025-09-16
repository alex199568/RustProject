mod vector;

use crate::vector::Vector;

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
    println!("a cross b = {:?}", c);
    let nc = -c;
    println!("-c = {:?}", nc);
}
