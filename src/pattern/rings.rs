use crate::color::Color;
use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};

use glam::Affine3A;
use glam::Vec3A;

pub struct Rings {
    pub common: PatternCommon,
    a: Color,
    b: Color,
}

impl Rings {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
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
impl From<Rings> for Pattern {
    fn from(r: Rings) -> Self {
        Pattern::Rings(r)
    }
}
