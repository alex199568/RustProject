use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};
use crate::pattern::uv_pattern::UvPattern;

use glam::{Affine3A, Vec2, Vec3A};

use crate::color::Color;

pub struct PlanarTexture {
    pub common: PatternCommon,
    pub uv_pattern: UvPattern,
}

impl PlanarTexture {
    pub fn new(uv_pattern: UvPattern) -> Self {
        Self {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            uv_pattern: uv_pattern,
        }
    }

    fn map_point(point: Vec3A) -> Vec2 {
        glam::vec2(point.x.fract().abs(), point.z.fract().abs())
    }
}

impl LocalPattern for PlanarTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        self.uv_pattern.uv_pattern_at(Self::map_point(point))
    }
}
impl From<PlanarTexture> for Pattern {
    fn from(t: PlanarTexture) -> Self {
        Pattern::PlanarTexture(t)
    }
}
