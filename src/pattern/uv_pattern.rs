use crate::pattern::align_check::AlignCheck;
use crate::pattern::uv_checkers::UvCheckers;
use crate::pattern::uv_image::UvImage;

use glam::Vec2;

use crate::color::Color;

pub enum UvPattern {
    Checkers(UvCheckers),
    AlignCheck(AlignCheck),
    UvImage(UvImage),
}

impl UvPattern {
    pub fn uv_pattern_at(&self, uv: Vec2) -> Color {
        match self {
            UvPattern::Checkers(c) => c.uv_pattern_at(uv),
            UvPattern::AlignCheck(ac) => ac.uv_pattern_at(uv),
            UvPattern::UvImage(ui) => ui.uv_pattern_at(uv),
        }
    }
}
