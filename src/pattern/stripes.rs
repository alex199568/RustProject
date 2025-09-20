use crate::color::Color;
use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};

use glam::Affine3A;
use glam::Vec3A;

pub struct Stripes {
    pub common: PatternCommon,
    a: Color,
    b: Color,
}

impl Stripes {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
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
impl From<Stripes> for Pattern {
    fn from(s: Stripes) -> Self {
        Pattern::Stripes(s)
    }
}
