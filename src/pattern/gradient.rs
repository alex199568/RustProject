use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};

use glam::{Affine3A, Vec3A};

use crate::color::Color;

pub struct Gradient {
    pub common: PatternCommon,
    a: Color,
    b: Color,
}

impl Gradient {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
        }
    }
}

impl LocalPattern for Gradient {
    fn local_at(&self, point: Vec3A) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}
impl From<Gradient> for Pattern {
    fn from(g: Gradient) -> Self {
        Pattern::Gradient(g)
    }
}
