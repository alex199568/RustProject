
use crate::color::Color;

use glam::Vec3A;
use glam::Affine3A;

use std::convert::From;

struct PatternCommon {
    inv: Affine3A
}

impl PatternCommon {

    fn new(transform: &Affine3A) -> Self {
        Self {
            inv: transform.inverse()
        }
    }

    fn at(&self, local: &dyn LocalPattern, shape_inv: &Affine3A, point: Vec3A) -> Color {
        let shape_point = shape_inv.transform_point3a(point);
        let pattern_point = &self.inv.transform_point3a(shape_point);
        local.local_at(*pattern_point)
    }
}

trait LocalPattern {

    fn local_at(&self, point: Vec3A) -> Color;
}

pub struct Stripes {
    common: PatternCommon,
    a: Color,
    b: Color
}

impl Stripes {

    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Stripes {

    fn local_at(&self, point: Vec3A) -> Color {
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

    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Gradient {

    fn local_at(&self, point: Vec3A) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

pub struct Rings {
    common: PatternCommon,
    a: Color,
    b: Color
}

impl Rings {

    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Rings {

    fn local_at(&self, point: Vec3A) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct Checkers {
    common: PatternCommon,
    a: Color,
    b: Color
}

impl Checkers {

    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b
        }
    }
}

impl LocalPattern for Checkers {

    fn local_at(&self, point: Vec3A) -> Color {
        if (point.x.round() + point.y.round() + point.z.round()) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub enum Pattern {
    Stripes(Stripes),
    Gradient(Gradient),
    Rings(Rings),
    Checkers(Checkers)
}

impl Pattern {

    pub fn at(&self, shape_inv: &Affine3A, point: Vec3A) -> Color {
        match self {
            Pattern::Stripes(s) => s.common.at(s, shape_inv, point),
            Pattern::Gradient(g) => g.common.at(g, shape_inv, point),
            Pattern::Rings(r) => r.common.at(r, shape_inv, point),
            Pattern::Checkers(c) => c.common.at(c, shape_inv, point)
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

impl From<Rings> for Pattern {

    fn from(r: Rings) -> Self {
        Pattern::Rings(r)
    }
}

impl From<Checkers> for Pattern {

    fn from(c: Checkers) -> Self {
        Pattern::Checkers(c)
    }
}
