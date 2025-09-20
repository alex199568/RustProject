use crate::color::Color;
use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};

use glam::Affine3A;
use glam::Vec3A;

pub struct Checkers {
    pub common: PatternCommon,
    a: Color,
    b: Color,
}

impl Checkers {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
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
impl From<Checkers> for Pattern {
    fn from(c: Checkers) -> Self {
        Pattern::Checkers(c)
    }
}
