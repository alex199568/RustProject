use crate::color::AccColor;
use crate::color::Color;

use image::{ColorType, ImageResult};
use std::path::Path;

pub struct Img {
    pub w: usize,
    pub h: usize,
    pub colors: Vec<Color>,
}

impl Img {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            colors: vec![Color::BLACK; w * h],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.colors[y * self.w + x] = color;
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Color {
        let yi = y.clamp(0, self.h - 1);
        let xi = x.clamp(0, self.w - 1);
        self.colors[yi * self.w + xi]
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
            image::ImageFormat::Png,
        )
    }

    pub fn load<P: AsRef<Path>>(file_path: P) -> ImageResult<Img> {
        let dyn_img = image::open(file_path)?;

        let rgb_img = dyn_img.to_rgb8();
        let (w, h) = rgb_img.dimensions();

        let mut colors = Vec::with_capacity((w * h) as usize);
        for p in rgb_img.pixels() {
            let [r, g, b] = p.0;
            colors.push(Color::from_srgb8(r, g, b))
        }

        Ok(Img {
            w: w as usize,
            h: h as usize,
            colors: colors,
        })
    }
}

pub struct AccImg {
    pub w: usize,
    pub h: usize,
    pub colors: Vec<AccColor>,
}

impl AccImg {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            colors: vec![AccColor::new(); w * h],
        }
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
