use crate::color::Color;
use crate::pattern::uv_pattern::UvPattern;

use glam::Vec2;

use crate::img::Img;

pub struct UvImage {
    img: Img,
    flip: bool,
    scale: Vec2,
}

impl UvImage {
    pub fn new(img: Img, flip: bool) -> Self {
        Self {
            img: img,
            flip: flip,
            scale: glam::vec2(1.0, 1.0),
        }
    }

    pub fn scaled(img: Img, scale: Vec2) -> Self {
        Self {
            img: img,
            flip: false,
            scale: scale,
        }
    }

    pub fn uv_pattern_at(&self, uv: Vec2) -> Color {
        let u = uv.x;
        let mut v = uv.y;
        if self.flip {
            v = 1.0 - v;
        }
        let x = u * (self.img.w as f32 - 1.0) * self.scale.x;
        let y = v * (self.img.h as f32 - 1.0) * self.scale.y;
        self.img.get(
            x.round() as usize % self.img.w,
            y.round() as usize % self.img.h,
        )
    }
}

impl From<UvImage> for UvPattern {
    fn from(ui: UvImage) -> Self {
        UvPattern::UvImage(ui)
    }
}
