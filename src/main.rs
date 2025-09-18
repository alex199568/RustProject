mod color;
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

use glam::Affine3A;
use glam::Vec3;

use crate::color::Color;
use crate::img::AccImg;
use crate::shape::{Sphere, Plane, Cube};
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
    let cyan_material = Material::builder().color(Color::CYAN).reflection(0.1).build();

    let stripes_affine = Affine3A::from_scale(glam::vec3(0.2, 1.0, 1.0));
    let stripes = Stripes::new(&stripes_affine, Color::WHITE, Color::LIGHT_GRAY);
    let stripes_material = Material::builder().pattern(stripes.into()).build();

    let gradient_affine = Affine3A::from_scale(glam::vec3(0.2, 1.0, 1.0));
    let gradient = Gradient::new(&gradient_affine, Color::LIGHT_GRAY, Color::GRAY);
    let gradient_material = Material::builder().pattern(gradient.into()).build();

    let rings_affine = Affine3A::IDENTITY;
    let rings = Rings::new(&rings_affine, Color::YELLOW, Color::MAGENTA);
    let rings_material = Material::builder().pattern(rings.into()).build();

    let checkers_affine = Affine3A::IDENTITY;
    let checkers = Checkers::new(&checkers_affine, Color::WHITE, Color::BLACK);
    let checkers_material = Material::builder().pattern(checkers.into()).reflection(0.7).build();

    let s1_affine = Affine3A::from_translation(glam::vec3(-1.0, 1.0, 2.0));
    let s1 = Sphere::new(&s1_affine, red_material);
    let s2_affine = Affine3A::from_translation(glam::vec3(-2.0, 1.0, 0.0));
    let s2 = Sphere::new(&s2_affine, blue_material);
    let s3_affine = Affine3A::from_translation(glam::vec3(2.0, 1.0, 0.0));
    let s3 = Sphere::new(&s3_affine, green_material);
    let cube_affine =
        Affine3A::from_translation(glam::vec3(0.0, 1.0, -1.0)) *
        Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_6);
    let cube = Cube::new(&cube_affine, cyan_material);
    let floor = Plane::new(&Affine3A::IDENTITY, checkers_material);

    let wall1_tr = 
        Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0)) * 
        Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_3) *
        Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall1 = Plane::new(&wall1_tr, stripes_material);

    let wall2_tr =
        Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0)) *
        Affine3A::from_rotation_y(-std::f32::consts::FRAC_PI_3) *
        Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall2 = Plane::new(&wall2_tr, gradient_material);

    let wall3_tr =
        Affine3A::from_translation(glam::vec3(0.0, 0.0, 3.0)) *
        Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall3 = Plane::new(&wall3_tr, rings_material);
    
    let shapes = vec![ 
        s1.into(), s2.into(), s3.into(), cube.into(),
        floor.into(),
        wall1.into(), wall2.into(), wall3.into()
    ];

    let l1_position = glam::vec3a(-10.0, 10.0, -10.0);
    let l1 = Light {
        position: l1_position,
        intensity: Color::GRAY
    };
    let l2_position = glam::vec3a(8.0, 8.0, -8.0);
    let l2 = Light {
        position: l2_position,
        intensity: Color::DARK_GRAY
    };
    let lights = vec![l1, l2];

    let capacity = shapes.len() * 2;
    let scene = Scene::new(shapes, lights);

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 4.0, -12.0),
        glam::vec3(0.0, 1.0, 0.0),
        Vec3::Y
    );
    let camera = Camera::new(
        1280, 720, 
        std::f32::consts::PI / 3.0, 
        camera_view
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

    let filepath = "renders/cube.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e)
    }
}
