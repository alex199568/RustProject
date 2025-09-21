use crate::pattern::pattern::{Pattern, PatternCommon, LocalPattern};
use crate::pattern::uv_pattern::UvPattern;
use crate::color::Color;

use glam::{Affine3A, Vec2, Vec3A};

pub struct SphericalTexture {
    pub common: PatternCommon,
    pub uv_pattern: UvPattern,
}

impl SphericalTexture {
    pub fn map_point(p: Vec3A) -> Vec2 {
        let theta = libm::atan2f(p.x, p.z);
        let r = p.length();
        let phi = libm::acosf(p.y / r);
        let raw_u = theta / (std::f32::consts::PI * 2.0);
        let u = 1.0 - (raw_u + 0.5);
        let v = 1.0 - phi / std::f32::consts::PI;
        glam::vec2(u, v)
    }

    pub fn new(uv_pattern: UvPattern) -> Self {
        Self {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            uv_pattern: uv_pattern,
        }
    }
}

impl LocalPattern for SphericalTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        self.uv_pattern.uv_pattern_at(Self::map_point(point))
    }
}

impl From<SphericalTexture> for Pattern {
    fn from(t: SphericalTexture) -> Self {
        Pattern::SphericalTexture(t)
    }
}
