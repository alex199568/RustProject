mod aabb;
mod camera;
mod color;
mod img;
mod intersection;
mod light;
mod material;
mod pattern;
mod ray;
mod scene;
pub mod shape;

use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;

use glam::Affine3A;
use glam::Vec2;
use glam::Vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::img::AccImg;
use crate::intersection::IntersectionBuffer;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{Checkers, Gradient, Rings, Stripes};
use crate::scene::Scene;
use crate::shape::{Cone, Cube, Cylinder, Group, Plane, Shape, Sphere, Triangle};

fn to_material(obj_mat: &tobj::Material) -> Material {
    Material::builder()
        .color(Color::option(obj_mat.diffuse))
        // .diffuse(Color::option(obj_mat.diffuse).r)
        // .ambient(Color::option(obj_mat.ambient).r)
        .shininess(obj_mat.shininess.unwrap_or(0.0))
        .refraction(obj_mat.optical_density.unwrap_or(1.0))
        .transparency(1.0 - obj_mat.dissolve.unwrap_or(1.0))
        .specular(Color::option(obj_mat.specular).r)
        .build()
}

fn load_obj<P: AsRef<Path>>(file_path: P, tr: &Affine3A) -> Shape {
    let path_ref: &Path = file_path.as_ref();
    let (models, materials) = tobj::load_obj(
        path_ref,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    )
    .expect("Failed to load obj");

    let mats = materials.expect("Failed to load materials");

    let mut tris: Vec<Shape> = Vec::new();

    for model in models.iter() {
        let mesh = &model.mesh;

        let pos = &mesh.positions; // len = 3 * num_verts
        let nrm = &mesh.normals; // len = 3 * num_verts (or 0)
        let uv = &mesh.texcoords; // len = 2 * num_verts (or 0)
        let idx = &mesh.indices; // len = 3 * num_tris

        let num_verts = pos.len() / 3;
        let has_nrm = nrm.len() == 3 * num_verts;
        let has_uv = uv.len() == 2 * num_verts;

        // // Pick material (or a default)
        let material = mesh
            .material_id
            .and_then(|i| mats.get(i))
            .map(|m| to_material(m))
            .unwrap_or_else(|| Material::builder().build());

        for tri in idx.chunks(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            // positions
            let p0 = glam::vec3a(pos[3 * i0], pos[3 * i0 + 1], pos[3 * i0 + 2]);
            let p1 = glam::vec3a(pos[3 * i1], pos[3 * i1 + 1], pos[3 * i1 + 2]);
            let p2 = glam::vec3a(pos[3 * i2], pos[3 * i2 + 1], pos[3 * i2 + 2]);

            // // optional per-vertex normals (object space)
            let (n1, n2, n3) = if has_nrm {
                (
                    Some(glam::vec3a(nrm[3 * i0], nrm[3 * i0 + 1], nrm[3 * i0 + 2]).normalize()),
                    Some(glam::vec3a(nrm[3 * i1], nrm[3 * i1 + 1], nrm[3 * i1 + 2]).normalize()),
                    Some(glam::vec3a(nrm[3 * i2], nrm[3 * i2 + 1], nrm[3 * i2 + 2]).normalize()),
                )
            } else {
                (None, None, None)
            };

            // optional per-vertex UVs
            let (uv1, uv2, uv3) = if has_uv {
                (
                    Some(Vec2::new(uv[2 * i0], uv[2 * i0 + 1])),
                    Some(Vec2::new(uv[2 * i1], uv[2 * i1 + 1])),
                    Some(Vec2::new(uv[2 * i2], uv[2 * i2 + 1])),
                )
            } else {
                (None, None, None)
            };

            let tr_mat = Material::builder()
                .color(material.color)
                .ambient(material.ambient)
                .diffuse(material.diffuse)
                .specular(material.specular)
                .shininess(material.shininess)
                .reflection(material.reflection)
                .transparency(material.transparency)
                .refraction(material.refraction)
                .build();

            let tri = Triangle::new(tr_mat, p0, p1, p2, n1, n2, n3, uv1, uv2, uv3);
            tris.push(tri.into());
        }
    }

    let mut result = Group::new(tr, tris);
    result.divide(1);

    result.into()
}

fn main() {
    let red_material = Material::builder().color(Color::RED).build();
    let green_material = Material::builder()
        .color(Color::GREEN)
        .reflection(0.3)
        .refraction(Material::IOR_GLASS)
        .transparency(0.7)
        .build();
    let blue_material = Material::builder()
        .color(Color::BLUE)
        .refraction(Material::IOR_GLASS)
        .transparency(0.6)
        .build();
    let cyan_material = Material::builder()
        .color(Color::CYAN)
        .reflection(0.1)
        .build();
    let alice_blue_material = Material::builder().color(Color::ALICE_BLUE).build();
    let lavender_material = Material::builder().color(Color::LAVENDER).build();

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
    let checkers_material = Material::builder()
        .pattern(checkers.into())
        .reflection(0.7)
        .build();

    let s1_affine = Affine3A::from_translation(glam::vec3(-1.0, 1.0, 2.0));
    let s1 = Sphere::new(&s1_affine, red_material);
    let s2_affine = Affine3A::from_translation(glam::vec3(-2.0, 1.0, 0.0));
    let s2 = Sphere::new(&s2_affine, blue_material);
    let s3_affine = Affine3A::from_translation(glam::vec3(2.0, 1.0, 0.0));
    let s3 = Sphere::new(&s3_affine, green_material);
    let cube_affine = Affine3A::from_translation(glam::vec3(0.0, 1.0, -1.0))
        * Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_6);
    let cube = Cube::new(&cube_affine, cyan_material);

    let cylinder_affine = Affine3A::from_translation(glam::vec3(-3.5, 1.0, -1.5));
    let cylinder = Cylinder::new(&cylinder_affine, alice_blue_material, (-1.0, 1.0), true);

    let cone_affine = Affine3A::from_translation(glam::vec3(3.5, 1.0, -1.5));
    let cone = Cone::new(&cone_affine, lavender_material, (-1.0, 0.5), true);

    let floor = Plane::new(&Affine3A::IDENTITY, checkers_material);

    let wall1_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0))
        * Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_3)
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall1 = Plane::new(&wall1_tr, stripes_material);

    let wall2_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0))
        * Affine3A::from_rotation_y(-std::f32::consts::FRAC_PI_3)
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall2 = Plane::new(&wall2_tr, gradient_material);

    let wall3_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 3.0))
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall3 = Plane::new(&wall3_tr, rings_material);

    let room_shapes = vec![wall1.into(), wall2.into(), wall3.into(), floor.into()];
    let room_affine = Affine3A::from_translation(glam::vec3(0.0, 0.0, 2.0));
    let room_group = Group::new(&room_affine, room_shapes);

    let prim_shapes = vec![
        s1.into(),
        s2.into(),
        s3.into(),
        cube.into(),
        cylinder.into(),
        cone.into(),
    ];
    let shapes_affine = Affine3A::from_translation(glam::vec3(-3.0, 0.0, 2.0))
        * Affine3A::from_scale(glam::vec3(0.5, 0.5, 0.5));
    let shapes_group = Group::new(&shapes_affine, prim_shapes);

    let monkey_affine = Affine3A::from_translation(glam::vec3(3.0, 1.0, 0.0))
        * Affine3A::from_rotation_y(20.0f32.to_radians());
    let monkey = load_obj("assets/models/monkey/smooth_monkey.obj", &monkey_affine);

    let normal_car_affine = Affine3A::from_rotation_y(140.0f32.to_radians());
    let normal_car = load_obj("assets/models/NormalCar1.obj", &normal_car_affine);

    let shapes = vec![
        shapes_group.into(),
        room_group.into(),
        monkey,
        normal_car.into(),
    ];

    let l1_position = glam::vec3a(-10.0, 3.0, -10.0);
    let l1 = Light {
        position: l1_position,
        intensity: Color::GRAY,
    };
    let l2_position = glam::vec3a(8.0, 8.0, -8.0);
    let l2 = Light {
        position: l2_position,
        intensity: Color::GRAY,
    };
    let lights = vec![l1, l2];

    let capacity = shapes.iter().map(|s: &Shape| s.max_intersections()).sum();
    let scene = Scene::new(shapes, lights);

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 4.0, -12.0),
        glam::vec3(0.0, 1.0, 0.0),
        Vec3::Y,
    );
    let camera = Camera::new(1280, 720, std::f32::consts::PI / 3.0, camera_view);

    let mut aimg = AccImg::new(camera.w, camera.h);
    let aa = 4;
    let depth = 4;

    println!("Rendering threads: {}", rayon::current_num_threads());

    let start = Instant::now();

    let w = aimg.w;
    aimg.colors.par_chunks_mut(w).enumerate().for_each_init(
        || IntersectionBuffer::new(capacity),
        |buffer, (y, row)| {
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
        },
    );

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath = "renders/normal_car.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
