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
use crate::light::AreaLight;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{Checkers, Gradient, Rings, Stripes};
use crate::scene::Scene;
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere, Triangle};

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

fn load_obj<P: AsRef<Path>>(file_path: P, tr: &Affine3A) -> (Vec<Material>, Shape) {
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
    let mut materials: Vec<Material> = Vec::new();

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
        let material_id = material.id;
        materials.push(material);

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

            let tri = Triangle::new(material_id, p0, p1, p2, n1, n2, n3, uv1, uv2, uv3);
            tris.push(tri.into());
        }
    }

    let mut result = Group::new(tr, tris);
    result.divide(1);

    (materials, result.into())
}

fn create_csg(tr: &Affine3A, m1: usize, m2: usize, m3: usize) -> Shape {
    let c1_affine = Affine3A::IDENTITY;
    let c1 = Cube::new(&c1_affine, m1);
    let s1_affine = Affine3A::from_translation(glam::vec3(0.0, 1.0, 0.0));
    let s1 = Sphere::new(&s1_affine, m2);
    let csg1_affine = Affine3A::from_translation(glam::vec3(-0.5, 0.0, 0.0));
    let csg1 = Csg::union(&csg1_affine, c1.into(), s1.into());

    let s2_affine = Affine3A::from_translation(glam::vec3(1.0, 0.0, 0.0));
    let s2 = Sphere::new(&s2_affine, m3);
    Csg::difference(tr, csg1.into(), s2.into()).into()
}

fn main() {
    let indian_red_material = Material::builder().color(Color::INDIAN_RED).build();
    let indian_red_id = indian_red_material.id;
    let forest_green_material = Material::builder()
        .color(Color::FOREST_GREEN)
        .reflection(0.3)
        .refraction(Material::IOR_GLASS)
        .transparency(0.7)
        .build();
    let forest_green_id = forest_green_material.id;
    let steel_blue_material = Material::builder()
        .color(Color::STEEL_BLUE)
        .refraction(Material::IOR_GLASS)
        .transparency(0.6)
        .build();
    let steel_blue_id = steel_blue_material.id;
    let cyan_material = Material::builder()
        .color(Color::TURQUOISE)
        .reflection(0.1)
        .build();
    let cyan_id = cyan_material.id;
    let alice_blue_material = Material::builder().color(Color::ALICE_BLUE).build();
    let alice_blue_id = alice_blue_material.id;
    let lavender_material = Material::builder().color(Color::LAVENDER).build();
    let lavender_id = lavender_material.id;

    let stripes_affine = Affine3A::from_scale(glam::vec3(0.2, 1.0, 1.0));
    let stripes = Stripes::new(&stripes_affine, Color::SNOW, Color::SILVER);
    let stripes_material = Material::builder().pattern(stripes.into()).build();
    let stripes_id = stripes_material.id;

    let gradient_affine = Affine3A::from_scale(glam::vec3(0.2, 1.0, 1.0));
    let gradient = Gradient::new(&gradient_affine, Color::GAINSBORO, Color::SILVER);
    let gradient_material = Material::builder().pattern(gradient.into()).build();
    let gradient_id = gradient_material.id;

    let rings_affine = Affine3A::IDENTITY;
    let rings = Rings::new(&rings_affine, Color::GOLD, Color::VIOLET);
    let rings_material = Material::builder().pattern(rings.into()).build();
    let rings_id = rings_material.id;

    let checkers_affine = Affine3A::IDENTITY;
    let checkers = Checkers::new(&checkers_affine, Color::SNOW, Color::DARK_SLATE_GRAY);
    let checkers_material = Material::builder()
        .pattern(checkers.into())
        .reflection(0.3)
        .build();
    let checkers_id = checkers_material.id;

    let red_material = Material::builder()
        .color(Color::RED)
        .transparency(0.5)
        .refraction(Material::IOR_AIR)
        .build();
    let red_id = red_material.id;
    let green_material = Material::builder()
        .color(Color::GREEN)
        .transparency(0.5)
        .refraction(Material::IOR_AIR)
        .build();
    let green_id = green_material.id;
    let blue_material = Material::builder()
        .color(Color::BLUE)
        .transparency(0.3)
        .reflection(0.2)
        .refraction(Material::IOR_GLASS)
        .build();
    let blue_id = blue_material.id;

    let mut materials = vec![
        indian_red_material,
        forest_green_material,
        steel_blue_material,
        cyan_material,
        alice_blue_material,
        lavender_material,
        stripes_material,
        gradient_material,
        rings_material,
        checkers_material,
        red_material,
        green_material,
        blue_material,
    ];

    let s1_affine = Affine3A::from_translation(glam::vec3(-1.0, 1.0, 2.0));
    let s1 = Sphere::new(&s1_affine, indian_red_id);
    let s2_affine = Affine3A::from_translation(glam::vec3(-2.0, 1.0, 0.0));
    let s2 = Sphere::new(&s2_affine, steel_blue_id);
    let s3_affine = Affine3A::from_translation(glam::vec3(2.0, 1.0, 0.0));
    let s3 = Sphere::new(&s3_affine, forest_green_id);
    let cube_affine = Affine3A::from_translation(glam::vec3(0.0, 1.0, -1.0))
        * Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_6);
    let cube = Cube::new(&cube_affine, cyan_id);

    let cylinder_affine = Affine3A::from_translation(glam::vec3(-3.5, 1.0, -1.5));
    let cylinder = Cylinder::new(&cylinder_affine, alice_blue_id, (-1.0, 1.0), true);

    let cone_affine = Affine3A::from_translation(glam::vec3(3.5, 1.0, -1.5));
    let cone = Cone::new(&cone_affine, lavender_id, (-1.0, 0.5), true);

    let floor = Plane::new(&Affine3A::IDENTITY, checkers_id);

    let wall1_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0))
        * Affine3A::from_rotation_y(std::f32::consts::FRAC_PI_3)
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall1 = Plane::new(&wall1_tr, stripes_id);

    let wall2_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 8.0))
        * Affine3A::from_rotation_y(-std::f32::consts::FRAC_PI_3)
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall2 = Plane::new(&wall2_tr, gradient_id);

    let wall3_tr = Affine3A::from_translation(glam::vec3(0.0, 0.0, 3.0))
        * Affine3A::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let wall3 = Plane::new(&wall3_tr, rings_id);

    let room_shapes = vec![wall1.into(), wall2.into(), wall3.into(), floor.into()];
    let room_affine = Affine3A::from_translation(glam::vec3(0.0, 0.0, 4.0));
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

    let monkey_affine = Affine3A::from_translation(glam::vec3(4.0, 1.0, 0.0))
        * Affine3A::from_rotation_y(20.0f32.to_radians());
    let (monkey_materials, monkey) =
        load_obj("assets/models/monkey/smooth_monkey.obj", &monkey_affine);

    let normal_car_affine = Affine3A::from_translation(glam::vec3(-4.0, 0.0, -2.0))
        * Affine3A::from_rotation_y(140.0f32.to_radians());
    let (normal_car_materials, normal_car) =
        load_obj("assets/models/NormalCar1.obj", &normal_car_affine);

    materials.extend(monkey_materials);
    materials.extend(normal_car_materials);

    let csg1_affine = Affine3A::from_translation(glam::vec3(-0.5, 0.0, 0.0))
        * Affine3A::from_rotation_y(180.0f32.to_radians());
    let csg1 = create_csg(&csg1_affine, red_id, green_id, blue_id);
    let csg2 = create_csg(
        &Affine3A::from_translation(glam::vec3(0.5, 0.0, 0.0)),
        red_id,
        green_id,
        blue_id,
    );
    let csg3_affine = Affine3A::from_translation(glam::vec3(0.0, 0.6, 0.0))
        * Affine3A::from_scale(glam::vec3(0.6, 0.6, 0.6))
        * Affine3A::from_rotation_y(30.0f32.to_radians());
    let csg3 = Csg::intersect(&csg3_affine, csg1, csg2);

    let shapes = vec![
        shapes_group.into(),
        room_group.into(),
        monkey,
        normal_car,
        csg3.into(),
    ];

    let l1_position = glam::vec3a(-3.0, 2.0, -16.0);
    let l2_position = glam::vec3a(10.0, 10.0, -16.0);
    let lights: Vec<Light> = vec![];

    let al1 = AreaLight::new(
        l1_position,
        glam::vec3a(0.2, 0.0, 0.0),
        2,
        glam::vec3a(0.0, 0.2, 0.0),
        2,
        Color::WHITE * 0.4,
    );
    let al2 = AreaLight::new(
        l2_position,
        glam::vec3a(0.2, 0.0, 0.0),
        2,
        glam::vec3a(0.0, 0.2, 0.0),
        2,
        Color::WHITE * 0.7,
    );
    let area_lights: Vec<AreaLight> = vec![al1, al2];

    let capacity = shapes.iter().map(|s: &Shape| s.max_intersections()).sum();
    let scene = Scene::new(materials, shapes, lights, area_lights);

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

    let filepath = "renders/area_light.png";
    match aimg.img().save(filepath) {
        Ok(_) => println!("Render saved to: {}", filepath),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
