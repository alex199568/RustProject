
use crate::color::Color;
use crate::point::Point;
use crate::matrix::Matrix;

use std::convert::From;

struct PatternCommon {
    inv: Matrix<4>
}

impl PatternCommon {

    fn new(transform: &Matrix<4>) -> Self {
        Self {
            inv: transform.inverse()
        }
    }

    fn at(&self, local: &dyn LocalPattern, shape_inv: &Matrix<4>, point: Point) -> Color {
        let shape_point = shape_inv * point;
        let pattern_point = &self.inv * shape_point;
        local.local_at(pattern_point)
    }
}

trait LocalPattern {

    fn local_at(&self, point: Point) -> Color;
}

pub struct Stripes {
    common: PatternCommon,
    a: Color,
    b: Color
}

impl Stripes {

    pub fn new(transform: &Matrix<4>, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Stripes {

    fn local_at(&self, point: Point) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct Gradient {
    common: PatternCommon,
    a: Color,
    b: Color
}

impl Gradient {

    pub fn new(transform: &Matrix<4>, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Gradient {

    fn local_at(&self, point: Point) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

pub enum Pattern {
    Stripes(Stripes),
    Gradient(Gradient)
}

impl Pattern {

    pub fn at(&self, shape_inv: &Matrix<4>, point: Point) -> Color {
        match self {
            Pattern::Stripes(s) => s.common.at(s, shape_inv, point),
            Pattern::Gradient(g) => g.common.at(g, shape_inv, point)
        }
    }
}

impl From<Stripes> for Pattern {

    fn from(s: Stripes) -> Self {
        Pattern::Stripes(s)
    }
}

impl From<Gradient> for Pattern {

    fn from(g: Gradient) -> Self {
        Pattern::Gradient(g)
    }
}
