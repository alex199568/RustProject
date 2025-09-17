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
mod scene;

use std::time::Instant;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::img::Img;
use crate::img::AccImg;
use crate::shape::Sphere;
use crate::intersection::IntersectionBuffer;
use crate::intersection::Hit;
use crate::material::Material;
use crate::light::Light;
use crate::camera::Camera;
use crate::scene::Scene;

// Single thread: 8365.784 ms

fn main() {
    let red_material = Material {
        color: Color::RED,
        ambient: 0.01,
        diffuse: 0.9,
        specular: 0.8,
        shininess: 200.0
    };
    let green_material = Material {
        color: Color::GREEN,
        ambient: 0.01,
        diffuse: 0.9,
        specular: 0.8,
        shininess: 150.0
    };
    let blue_material = Material {
        color: Color::BLUE,
        ambient: 0.01,
        diffuse: 0.9,
        specular: 0.8,
        shininess: 230.0
    };
    let light_gray_material = Material {
        color: Color::LIGHT_GRAY,
        ambient: 0.01,
        diffuse: 0.9,
        specular: 0.8,
        shininess: 100.0
    };

    let s1 = Sphere::new(Matrix::<4>::translate(0.0, 1.0, 0.0), red_material);
    let s2 = Sphere::new(Matrix::<4>::translate(-2.0, 1.0, 0.0), blue_material);
    let s3 = Sphere::new(Matrix::<4>::translate(2.0, 1.0, 0.0), green_material);
    let floor = Sphere::new(Matrix::<4>::scale(10.0, 0.001, 10.0), light_gray_material);
    let shapes = vec![ s1, s2, s3, floor];

    let l1 = Light {
        position: Point { x: -10.0, y: 10.0, z: -10.0 },
        intensity: Color::GRAY
    };
    let l2 = Light {
        position: Point {x: 8.0, y: 8.0, z: -8.0},
        intensity: Color::DARK_GRAY
    };
    let lights = vec![l1, l2];

    let mut scene = Scene::new(shapes, lights);

    let camera = Camera::new(
        1920, 1080, 
        std::f32::consts::PI / 3.0, 
        Matrix::<4>::view(
            Point{x: 0.0, y: 1.0, z: -8.0},
            Point{x: 0.0, y: 0.5, z: 0.0},
            Vector::Y
        )
    );

    let start = Instant::now();

    let mut aimg = AccImg::new(camera.w, camera.h);
    let aa = 8;
    let img_step = 1.0 / aa as f32;

    let mut y = 0.0f32;
    while y < aimg.h as f32 {
        let mut x = 0.0f32;
        while x < aimg.w as f32 {
            let r = camera.ray(x, y);
            let c = scene.color(&r);
            aimg.set(x, y, c);
            x += img_step;
        }
        y += img_step;
    }

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/ascene.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
