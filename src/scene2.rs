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
    Checkers, Gradient, PlanarTexture, Rings, SphericalTexture, Stripes, UvCheckers,
};
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere};

pub fn scene2() -> Scene {
    let sphere_checkers = UvCheckers {
        width: 16,
        height: 8,
        a: Color::SNOW,
        b: Color::DARK_SLATE_GRAY,
    };
    let sphere_texture = SphericalTexture::new(&Affine3A::IDENTITY, sphere_checkers);
    let sphere_material = Material::builder().pattern(sphere_texture.into()).build();
    let sphere_affine = translate(0.0, 1.0, 0.0);
    let sphere = Sphere::new(&sphere_affine, sphere_material.id);

    let plane_checkers = UvCheckers {
        width: 2,
        height: 2,
        a: Color::SNOW,
        b: Color::DARK_SLATE_GRAY,
    };
    let plane_texture = PlanarTexture::new(plane_checkers);
    let plane_material = Material::builder().pattern(plane_texture.into()).build();
    let plane = Plane::new(&Affine3A::IDENTITY, plane_material.id);

    let l1 = Light {
        position: glam::vec3a(-10.0, 10.0, -10.0),
        intensity: Color::WHITE * 0.7,
    };
    let l2 = Light {
        position: glam::vec3a(10.0, 10.0, -10.0),
        intensity: Color::WHITE * 0.5,
    };

    let materials = vec![sphere_material, plane_material];
    let shapes = vec![sphere.into(), plane.into()];
    let lights = vec![l1, l2];
    let area_lights = vec![];

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 4.0, -4.0),
        glam::vec3(0.0, 1.0, 0.0),
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
