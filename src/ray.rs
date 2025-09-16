
use crate::vector::Vector;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {

    #[inline]
    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}
