use crate::scene::Scene;

use glam::Affine3A;
use glam::Vec3;

use crate::robj::load_obj;
use crate::transform::{rotate_yd, scale, translate};

use crate::camera::Camera;
use crate::color::Color;
use crate::light::AreaLight;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{
    AlignCheck, Checkers, CubeTexture, CylindricalTexture, Gradient, PlanarTexture, Rings,
    SphericalTexture, Stripes, UvCheckers,
};
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere};

pub fn scene2() -> Scene {
    let sphere_checkers = UvCheckers {
        width: 16,
        height: 8,
        a: Color::SNOW,
        b: Color::DARK_SLATE_GRAY,
    };
    let sphere_texture = SphericalTexture::new(sphere_checkers.into());
    let sphere_material = Material::builder().pattern(sphere_texture.into()).build();
    let sphere_affine = translate(-3.0, 1.0, 0.0);
    let sphere = Sphere::new(&sphere_affine, sphere_material.id);

    let plane_pattern = AlignCheck {
        main: Color::GOLD,
        ul: Color::INDIAN_RED,
        ur: Color::STEEL_BLUE,
        bl: Color::VIOLET,
        br: Color::FOREST_GREEN,
    };
    let plane_texture = PlanarTexture::new(plane_pattern.into());
    let plane_material = Material::builder().pattern(plane_texture.into()).build();
    let plane = Plane::new(&Affine3A::IDENTITY, plane_material.id);

    let cube_pattern = AlignCheck {
        main: Color::GOLD,
        ul: Color::INDIAN_RED,
        ur: Color::STEEL_BLUE,
        bl: Color::VIOLET,
        br: Color::FOREST_GREEN,
    };
    let cube_texture = CubeTexture::new(cube_pattern.into());
    let cube_material = Material::builder()
        .pattern(cube_texture.into())
        .transparency(0.2)
        .refraction(Material::IOR_VACUUM)
        .build();
    let cube_affine = translate(0.0, 1.0, 0.0) * rotate_yd(15.0);
    let cube = Cube::new(&cube_affine, cube_material.id);

    let cylinder_checkers = UvCheckers {
        width: 16,
        height: 8,
        a: Color::SNOW,
        b: Color::DARK_SLATE_GRAY,
    };
    let cylinder_texture = CylindricalTexture::new(cylinder_checkers.into());
    let cylinder_material = Material::builder().pattern(cylinder_texture.into()).build();
    let cylinder = Cylinder::new(
        &translate(3.0, 0.0, 0.0),
        cylinder_material.id,
        (0.0, 2.0),
        true,
    );

    let l1 = Light {
        position: glam::vec3a(-10.0, 10.0, -10.0),
        intensity: Color::WHITE * 0.7,
    };
    let l2 = Light {
        position: glam::vec3a(10.0, 10.0, -10.0),
        intensity: Color::WHITE * 0.5,
    };

    let materials = vec![
        sphere_material,
        plane_material,
        cylinder_material,
        cube_material,
    ];
    let shapes = vec![sphere.into(), plane.into(), cylinder.into(), cube.into()];
    let lights = vec![l1, l2];
    let area_lights = vec![];

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 4.0, -6.0),
        glam::vec3(0.0, 1.5, 0.0),
        Vec3::Y,
    );
    let camera = Camera::new(1280, 720, std::f32::consts::PI / 3.0, camera_view);

    Scene {
        materials: materials,
        shapes: shapes,
        lights: lights,
        area_lights: area_lights,
        camera: camera,
    }
}
