mod vector;
mod point;
mod color;
mod matrix;
mod ray;
mod intersection;
mod shape;
mod img;
mod material;
mod light;

use std::time::Instant;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::img::Img;
use crate::shape::Sphere;
use crate::intersection::IntersectionBuffer;
use crate::material::Material;
use crate::light::Light;

fn main() {
    let ray_origin = Point { x: 0.0, y: 0.0, z: -5.0 };
    let wall_z: f32 = 10.0;
    let wall_size: f32 = 7.0;
    let canvas_pixels = 1080;
    let half: f32 = wall_size / 2.0;
    let pixel_size: f32 = wall_size / canvas_pixels as f32;

    let mut img = Img::new(canvas_pixels, canvas_pixels);

    let red_material = Material {
        color: Color::RED,
        ambient: 0.01,
        diffuse: 0.9,
        specular: 0.8,
        shininess: 200.0
    };
    let sphere = Sphere::new(Matrix::<4>::IDENTITY, red_material);

    let light = Light {
        position: Point { x: -10.0, y: 10.0, z: -10.0 },
        intensity: Color::GRAY
    };

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
                let point = r.at(hit.t);
                let eye = -r.direction;
                let normal = sphere.normal(point);
                let color = light.shade(&sphere.material, point, eye, normal);

                img.set(x, y, color);
            }

            buffer.clear();
        }
    }

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/light.png";
    match img.save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
