use crate::color::Color;
use crate::pattern::uv_pattern::UvPattern;

use glam::Vec2;

pub struct AlignCheck {
    pub main: Color,
    pub ul: Color,
    pub ur: Color,
    pub bl: Color,
    pub br: Color,
}

impl AlignCheck {
    pub fn uv_pattern_at(&self, uv: Vec2) -> Color {
        let u = uv.x.abs();
        let v = uv.y.abs();
        if v > 0.8 {
            if u < 0.2 {
                return self.ul;
            }
            if u > 0.8 {
                return self.ur;
            }
        }
        if v < 0.2 {
            if u < 0.2 {
                return self.bl;
            }
            if u > 0.8 {
                return self.br;
            }
        }
        self.main
    }
}
impl From<AlignCheck> for UvPattern {
    fn from(ac: AlignCheck) -> Self {
        UvPattern::AlignCheck(ac)
    }
}
