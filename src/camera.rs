
use crate::ray::Ray;

use glam::Vec3A;
use glam::Affine3A;

pub struct Camera {
    pub w: usize,
    pub h: usize,
    inv: Affine3A,
    origin: Vec3A,
    pixel_size: f32,
    half_width: f32,
    half_height: f32
}

impl Camera {

    pub fn new(w: usize, h: usize, fov: f32, tr: Affine3A) -> Self {
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
        let origin = &inv.transform_point3a(Vec3A::ZERO);

        Self {
            w: w,
            h: h,
            inv: inv,
            origin: *origin,
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
        let pixel = &self.inv.transform_point3a(glam::vec3a(world_x, world_y, -1.0));
        let direction = (pixel - self.origin).normalize();
        Ray {
            origin: self.origin,
            direction: direction
        }
    }
}
