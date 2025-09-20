use crate::scene::Scene;

use glam::Affine3A;
use glam::Vec3;

use crate::robj::load_obj;
use crate::transform::{rotate_yd, scale};

use crate::camera::Camera;
use crate::color::Color;
use crate::light::AreaLight;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{Checkers, Gradient, Rings, SphericalMapper, Stripes, Texture, UvCheckers};
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere};

pub fn scene2() -> Scene {
    let uv_checkers = UvCheckers {
        width: 8,
        height: 4,
        a: Color::SNOW,
        b: Color::DARK_SLATE_GRAY,
    };
    let spherical_mapper = SphericalMapper {};
    let texture = Texture::new(&Affine3A::IDENTITY, uv_checkers, spherical_mapper);
    let checkers_material = Material::builder().pattern(texture.into()).build();

    let sphere_affine = rotate_yd(45.0);
    let sphere = Sphere::new(&sphere_affine, checkers_material.id);

    let l1 = Light {
        position: glam::vec3a(-10.0, 10.0, -10.0),
        intensity: Color::WHITE * 0.7,
    };
    let l2 = Light {
        position: glam::vec3a(10.0, 1.0, -1.0),
        intensity: Color::WHITE * 0.5,
    };

    let materials = vec![checkers_material];
    let shapes = vec![sphere.into()];
    let lights = vec![l1, l2];
    let area_lights = vec![];

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 0.0, -4.0),
        glam::vec3(0.0, 0.0, 0.0),
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
