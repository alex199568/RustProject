mod vector;
mod point;
mod color;
mod matrix;
mod ray;
mod intersection;
mod shape;
mod img;
mod pattern;
mod material;
mod light;
mod camera;
mod scene;

use std::time::Instant;

use rayon::prelude::*;

use crate::vector::Vector;
use crate::point::Point;
use crate::color::Color;
use crate::matrix::Matrix;
use crate::img::AccImg;
use crate::shape::Sphere;
use crate::shape::Plane;
use crate::intersection::IntersectionBuffer;
use crate::material::Material;
use crate::pattern::{Stripes, Gradient, Rings, Checkers};
use crate::light::Light;
use crate::camera::Camera;
use crate::scene::Scene;

fn main() {
    let red_material = Material::builder().color(Color::RED).build();
    let green_material = Material::builder().color(Color::GREEN).reflection(0.3).refraction(Material::IOR_GLASS).transparency(0.7).build();
    let blue_material = Material::builder().color(Color::BLUE).refraction(Material::IOR_GLASS).transparency(0.6).build();
    let _light_gray_material = Material::builder().color(Color::LIGHT_GRAY).reflection(0.3).refraction(Material::IOR_GLASS).transparency(0.7).build();

    let stripes = Stripes::new(&Matrix::<4>::scale(0.2, 1.0, 1.0), Color::WHITE, Color::LIGHT_GRAY);
    let stripes_material = Material::builder().pattern(stripes.into()).build();

    let gradient = Gradient::new(&Matrix::<4>::scale(0.2, 1.0, 1.0), Color::LIGHT_GRAY, Color::GRAY);
    let gradient_material = Material::builder().pattern(gradient.into()).build();

    let rings = Rings::new(&Matrix::<4>::IDENTITY, Color::YELLOW, Color::MAGENTA);
    let rings_material = Material::builder().pattern(rings.into()).build();

    let checkers = Checkers::new(&Matrix::<4>::IDENTITY, Color::WHITE, Color::BLACK);
    let checkers_material = Material::builder().pattern(checkers.into()).reflection(0.7).build();

    let s1 = Sphere::new(&Matrix::<4>::translate(0.0, 1.0, 0.0), red_material);
    let s2 = Sphere::new(&Matrix::<4>::translate(-2.0, 1.0, 0.0), blue_material);
    let s3 = Sphere::new(&Matrix::<4>::translate(2.0, 1.0, 0.0), green_material);
    let floor = Plane::new(&Matrix::<4>::IDENTITY, checkers_material);

    let wall1_tr = 
        Matrix::<4>::translate(0.0, 0.0, 8.0) * 
        Matrix::<4>::rotate_y(std::f32::consts::FRAC_PI_3) *
        Matrix::<4>::rotate_x(std::f32::consts::FRAC_PI_2);
    let wall1 = Plane::new(&wall1_tr, stripes_material);

    let wall2_tr =
        Matrix::<4>::translate(0.0, 0.0, 8.0) *
        Matrix::<4>::rotate_y(-std::f32::consts::FRAC_PI_3) *
        Matrix::<4>::rotate_x(std::f32::consts::FRAC_PI_2);
    let wall2 = Plane::new(&wall2_tr, gradient_material);

    let wall3_tr =
        Matrix::<4>::translate(0.0, 0.0, 3.0) *
        Matrix::<4>::rotate_x(std::f32::consts::FRAC_PI_2);
    let wall3 = Plane::new(&wall3_tr, rings_material);
    
    let shapes = vec![ 
        s1.into(), s2.into(), s3.into(), 
        floor.into(),
        wall1.into(), wall2.into(), wall3.into()
    ];

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
    let scene = Scene::new(shapes, lights);

    let camera = Camera::new(
        1280, 720, 
        std::f32::consts::PI / 3.0, 
        Matrix::<4>::view(
            Point{x: 0.0, y: 1.0, z: -8.0},
            Point{x: 0.0, y: 1.0, z: 0.0},
            Vector::Y
        )
    );

    let mut aimg = AccImg::new(camera.w, camera.h);
    let aa = 4;
    let depth = 4;

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
                        let c = scene.color(&r, buffer, depth);
                        row[x] += c;
                    }
                }
            }
        });


    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/refractions.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
