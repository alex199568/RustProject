
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;

pub struct Camera {
    pub w: usize,
    pub h: usize,
    inv: Matrix<4>,
    origin: Point,
    pixel_size: f32,
    half_width: f32,
    half_height: f32
}

impl Camera {

    pub fn new(w: usize, h: usize, fov: f32, tr: Matrix<4>) -> Self {
        let half_view = (fov / 2.0).tan();
        let wf = w as f32;
        let hf = h as f32;
        let aspect = wf / hf;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / wf;
        let inv = tr.inverse();
        let origin = &inv * Point::ZERO;

        Self {
            w: w,
            h: h,
            inv: inv,
            origin: origin,
            pixel_size: pixel_size,
            half_width: half_width,
            half_height: half_height
        }
    }

    pub fn ray(&self, x: f32, y: f32) -> Ray {
        let x_offset = (x + 0.5) * self.pixel_size;
        let y_offset = (y + 0.5) * self.pixel_size;
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;
        let pixel = &self.inv * Point { x: world_x, y: world_y, z: -1.0};
        let direction = (pixel - self.origin).unit();
        Ray {
            origin: self.origin,
            direction: direction
        }
    }
}
