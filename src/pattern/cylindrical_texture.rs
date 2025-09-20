use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};
use crate::pattern::uv_pattern::UvPattern;

use glam::{Affine3A, Vec2, Vec3A};

use crate::color::Color;

// TODO: improve cylindrical mapping

pub struct CylindricalTexture {
    pub common: PatternCommon,
    uv_pattern: UvPattern,
}

impl CylindricalTexture {
    pub fn new(uv_pattern: UvPattern) -> Self {
        Self {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            uv_pattern: uv_pattern,
        }
    }

    fn map_point(point: Vec3A) -> Vec2 {
        let theta = libm::atan2f(point.x, point.z);
        let raw_u = theta / (2.0 * std::f32::consts::PI);
        let u = 1.0 - (raw_u + 0.5);
        let v = point.y.fract();
        glam::vec2(u, v)
    }
}

impl LocalPattern for CylindricalTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        self.uv_pattern.uv_pattern_at(Self::map_point(point))
    }
}
impl From<CylindricalTexture> for Pattern {
    fn from(t: CylindricalTexture) -> Self {
        Pattern::CylindricalTexture(t)
    }
}
