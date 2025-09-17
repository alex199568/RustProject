
use crate::color::Color;
use crate::point::Point;
use crate::matrix::Matrix;

pub struct Stripes {
    inv: Matrix<4>,
    a: Color,
    b: Color
}

impl Stripes {

    pub fn new(transform: &Matrix<4>, a: Color, b: Color) -> Self {
        Self {
            inv: transform.inverse(),
            a: a,
            b: b
        }
    }

    pub fn at(&self, shape_inv: &Matrix<4>, point: Point) -> Color {
        let shape_point = shape_inv * point;
        let pattern_point = &self.inv * shape_point;
        self.local_at(pattern_point)
    }

    pub fn local_at(&self, point: Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
