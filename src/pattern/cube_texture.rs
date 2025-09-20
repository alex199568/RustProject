use crate::pattern::cube_face::CubeFace;
use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};
use crate::pattern::uv_pattern::UvPattern;

use crate::color::Color;

use glam::Affine3A;
use glam::Vec3A;

pub struct CubeTexture {
    pub common: PatternCommon,
    uv_pattern: UvPattern,
}

impl CubeTexture {
    pub fn new(uv_pattern: UvPattern) -> Self {
        CubeTexture {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            uv_pattern: uv_pattern,
        }
    }
}

impl LocalPattern for CubeTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        self.uv_pattern.uv_pattern_at(CubeFace::map_point(point))
    }
}
impl From<CubeTexture> for Pattern {
    fn from(ct: CubeTexture) -> Self {
        Pattern::CubeTexture(ct)
    }
}
