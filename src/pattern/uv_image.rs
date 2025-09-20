use crate::color::Color;
use crate::pattern::uv_pattern::UvPattern;

use glam::Vec2;

use crate::img::Img;

pub struct UvImage {
    img: Img,
    flip: bool,
}

impl UvImage {
    pub fn new(img: Img, flip: bool) -> Self {
        Self {
            img: img,
            flip: flip,
        }
    }

    pub fn uv_pattern_at(&self, uv: Vec2) -> Color {
        let u = uv.x;
        let mut v = uv.y;
        if self.flip {
            v = 1.0 - v;
        }
        let x = u * (self.img.w as f32 - 1.0);
        let y = v * (self.img.h as f32 - 1.0);
        self.img.get(x.round() as usize, y.round() as usize)
    }
}

impl From<UvImage> for UvPattern {
    fn from(ui: UvImage) -> Self {
        UvPattern::UvImage(ui)
    }
}
