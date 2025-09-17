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
mod camera;

use std::time::Instant;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::img::Img;
use crate::shape::Sphere;
use crate::intersection::IntersectionBuffer;
use crate::intersection::Hit;
use crate::material::Material;
use crate::light::Light;
use crate::camera::Camera;

fn main() {
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

    let camera = Camera::new(
        1920, 1080, 
        std::f32::consts::PI / 3.0, 
        Matrix::<4>::view(
            Point{x: 0.0, y: 0.0, z: -8.0},
            Point{x: 0.0, y: 0.0, z: 0.0},
            Vector::Y
        )
    );
    let mut img = Img::new(camera.w, camera.h);

    let mut buffer = IntersectionBuffer::new(2);

    let start = Instant::now();

    for y in 0..camera.h {
        for x in 0..camera.w {
            let r = camera.ray(x as f32, y as f32);
            sphere.intersect(r, &mut buffer, 0);
            if let Some(hit) = buffer.hit() {
                let h = Hit::new(&sphere, hit, r);
                let color = light.shade(&sphere.material, &h);
                img.set(x, y, color);
            }
            buffer.clear();
        } 
    }

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/scene.png";
    match img.save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
