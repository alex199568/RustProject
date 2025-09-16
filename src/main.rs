mod vector;
mod point;
mod color;
mod matrix;
mod ray;
mod intersection;
mod shape;
mod img;

use std::time::Instant;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::img::Img;
use crate::shape::Sphere;
use crate::intersection::IntersectionBuffer;

fn main() {
    let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
    let wall_z: f32 = 10.0;
    let wall_size: f32 = 7.0;
    let canvas_pixels = 1080;
    let half: f32 = wall_size / 2.0;
    let pixel_size: f32 = wall_size / canvas_pixels as f32;

    let mut img = Img::new(canvas_pixels, canvas_pixels);

    let sphere = Sphere::new(Matrix::<4>::IDENTITY);

    let mut buffer = IntersectionBuffer::new(2);

    let start = Instant::now();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;
            let position = Point { x: world_x, y: world_y, z: wall_z};
            let r = Ray { origin: ray_origin, direction: (position - ray_origin).unit()};
            sphere.intersect(r, &mut buffer, 0);

            if let Some(hit) = buffer.hit() {
                img.set(x, y, Color::RED);
            }

            buffer.clear();
        }
    }

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    img.save("renders/sphere.png");
}
