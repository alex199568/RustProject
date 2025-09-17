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

use rayon::prelude::*;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::img::Img;
use crate::img::AccImg;
use crate::shape::Sphere;
use crate::shape::Plane;
use crate::intersection::IntersectionBuffer;
use crate::intersection::Hit;
use crate::material::Material;
use crate::light::Light;
use crate::camera::Camera;
use crate::scene::Scene;

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

    let s1 = Sphere::new(&Matrix::<4>::translate(0.0, 1.0, 0.0), red_material);
    let s2 = Sphere::new(&Matrix::<4>::translate(-2.0, 1.0, 0.0), blue_material);
    let s3 = Sphere::new(&Matrix::<4>::translate(2.0, 1.0, 0.0), green_material);
    let floor = Plane::new(&Matrix::<4>::IDENTITY, light_gray_material);
    let shapes = vec![ s1.into(), s2.into(), s3.into(), floor.into()];

    let l1 = Light {
        position: Point { x: -10.0, y: 10.0, z: -10.0 },
        intensity: Color::GRAY
    };
    let l2 = Light {
        position: Point {x: 8.0, y: 8.0, z: -8.0},
        intensity: Color::DARK_GRAY
    };
    let lights = vec![l1, l2];

    let capacity = shapes.len() * 2;
    let mut scene = Scene::new(shapes, lights);

    let camera = Camera::new(
        1280, 720, 
        std::f32::consts::PI / 3.0, 
        Matrix::<4>::view(
            Point{x: 0.0, y: 1.0, z: -8.0},
            Point{x: 0.0, y: 0.5, z: 0.0},
            Vector::Y
        )
    );

    let mut aimg = AccImg::new(camera.w, camera.h);
    let aa = 4;
    let img_step = 1.0 / aa as f32;

    println!("Rendeing threads: {}", rayon::current_num_threads());

    let start = Instant::now();

    let w = aimg.w;
    aimg.colors
        .par_chunks_mut(w)
        .enumerate()
        .for_each_init( || IntersectionBuffer::new(capacity), |buffer, (y, row)| {
            let fy = y as f32;
            for x in 0..w {
                let fx = x as f32;
                for sy in 0..aa {
                    for sx in 0..aa {
                        let u = fx + (sx as f32 + 0.5) / aa as f32;
                        let v = fy + (sy as f32 + 0.5) / aa as f32;
                        let r = camera.ray(u, v);
                        let c = scene.color(&r, buffer);
                        row[x] += c;
                    }
                }
            }
        });


    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/plane.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
