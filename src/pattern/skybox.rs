use crate::pattern::cube_face::CubeFace;
use crate::pattern::pattern::{LocalPattern, Pattern, PatternCommon};

use crate::color::Color;

use glam::Affine3A;
use glam::Vec3A;

use crate::pattern::uv_image::UvImage;

pub struct Skybox {
    pub common: PatternCommon,
    left: UvImage,
    right: UvImage,
    back: UvImage,
    front: UvImage,
    bottom: UvImage,
    top: UvImage,
}

impl Skybox {
    pub fn new(
        left: UvImage,
        right: UvImage,
        back: UvImage,
        front: UvImage,
        bottom: UvImage,
        top: UvImage,
    ) -> Self {
        Self {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            left: left,
            right: right,
            back: back,
            front: front,
            bottom: bottom,
            top: top,
        }
    }
}

impl LocalPattern for Skybox {
    fn local_at(&self, point: Vec3A) -> Color {
        let face = CubeFace::from_point(point);
        let uv = CubeFace::map_point(point);

        match face {
            CubeFace::Left => self.left.uv_pattern_at(uv),
            CubeFace::Right => self.right.uv_pattern_at(uv),
            CubeFace::Back => self.back.uv_pattern_at(uv),
            CubeFace::Front => self.front.uv_pattern_at(uv),
            CubeFace::Bottom => self.bottom.uv_pattern_at(uv),
            CubeFace::Top => self.top.uv_pattern_at(uv),
        }
    }
}
impl From<Skybox> for Pattern {
    fn from(s: Skybox) -> Self {
        Pattern::Skybox(s)
    }
}
