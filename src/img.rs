
use crate::color::Color;
use crate::color::AccColor;

use std::path::Path;
use image::{ColorType, ImageResult};

pub struct Img {
    pub w: usize,
    pub h: usize,
    pub colors: Vec<Color>
}

impl Img {

    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            colors: vec![Color::BLACK; w * h]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.colors[y * self.w + x] = color;
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> ImageResult<()> {
        let mut buf = Vec::with_capacity(self.w * self.h * 3);
        for c in &self.colors {
            buf.extend_from_slice(&c.srgb8());
        }

        image::save_buffer_with_format(
            path,
            &buf,
            self.w as u32,
            self.h as u32,
            ColorType::Rgb8,
            image::ImageFormat::Png
        )
    }
}

pub struct AccImg {
    pub w: usize,
    pub h: usize,
    pub colors: Vec<AccColor>
}

impl AccImg {

    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            colors: vec![AccColor::new(); w * h]
        }
    }

    pub fn set(&mut self, x: f32, y: f32, color: Color) {
        let xi = (x as usize).clamp(0, self.w);
        let yi = (y as usize).clamp(0, self.h);
        self.colors[yi * self.w + xi] += color;
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.colors[y * self.w + x].get()
    }

    pub fn img(&self) -> Img {
        let mut result = Img::new(self.w, self.h);

        for y in 0..self.h {
            for x in 0..self.w {
                result.set(x, y, self.get(x, y));
            }
        }

        result
    }
}
