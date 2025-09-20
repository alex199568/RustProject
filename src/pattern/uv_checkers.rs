use crate::color::Color;
use crate::pattern::uv_pattern::UvPattern;

use glam::Vec2;

pub struct UvCheckers {
    pub width: usize,
    pub height: usize,
    pub a: Color,
    pub b: Color,
}

impl UvCheckers {
    pub fn uv_pattern_at(&self, uv: Vec2) -> Color {
        let u2 = (uv.x * self.width as f32).floor();
        let v2 = (uv.y * self.height as f32).floor();
        if (u2 + v2) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
impl From<UvCheckers> for UvPattern {
    fn from(c: UvCheckers) -> Self {
        UvPattern::Checkers(c)
    }
}
