
use crate::color::Color;

use std::path::Path;
use image::{ColorType, ImageResult, ImageError};

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
        // save png using rust image
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

/*
pub struct Color { pub r: f32, pub g: f32, pub b: f32 }



pub struct Img {
    pub w: usize,
    pub h: usize,
    pub colors: Vec<Color>,
}

impl Img {
    pub fn new(w: usize, h: usize) -> Self {
        // length = w*h, filled (black). Not just capacity.
        Self { w, h, colors: vec![Color::default(); w * h] }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.w && y < self.h, "pixel out of bounds");
        y * self.w + x
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.colors[self.idx(x, y)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        let i = self.idx(x, y);
        self.colors[i] = color;
    }
}
*/