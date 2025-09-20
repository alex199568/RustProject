use crate::scene::Scene;

use glam::Affine3A;
use glam::Vec3;

use crate::img::Img;
use crate::robj::load_obj;
use crate::transform::{rotate_xd, rotate_yd, rotate_zd, scale, translate};

use crate::camera::Camera;
use crate::color;
use crate::color::Color;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{
    AlignCheck, Checkers, CubeTexture, CylindricalTexture, Gradient, PlanarTexture, Rings, Skybox,
    SphericalTexture, Stripes, UvCheckers, UvImage,
};
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere, Torus};

pub fn scene2() -> Scene {
    let sphere_checkers = UvCheckers {
        width: 16,
        height: 8,
        a: color::White::SNOW,
        b: color::Gray::DARK_SLATE_GRAY,
    };
    let sphere_texture = SphericalTexture::new(sphere_checkers.into());
    let sphere_material = Material::builder()
        .pattern(Some(sphere_texture.into()))
        .build();
    let sphere_affine = translate(-5.0, 1.0, -1.0);
    let sphere = Sphere::new(&sphere_affine, sphere_material.id);

    let plane_pattern = AlignCheck {
        main: color::Yellow::GOLD,
        ul: color::Red::FIRE_BRICK,
        ur: color::Blue::AQUAMARINE,
        bl: color::Purple::BLUE_VIOLET,
        br: color::Green::CHARTRUSE,
    };
    let plane_texture = PlanarTexture::new(plane_pattern.into());
    let plane_material = Material::builder()
        .pattern(Some(plane_texture.into()))
        .reflection(0.2)
        .build();
    let plane = Plane::new(&Affine3A::IDENTITY, plane_material.id);

    let cube_pattern = AlignCheck {
        main: color::Yellow::GOLD,
        ul: color::Red::FIRE_BRICK,
        ur: color::Blue::AQUAMARINE,
        bl: color::Purple::BLUE_VIOLET,
        br: color::Green::CHARTRUSE,
    };
    let cube_texture = CubeTexture::new(cube_pattern.into());
    let cube_material = Material::builder()
        .pattern(Some(cube_texture.into()))
        .transparency(0.2)
        .refraction(Material::IOR_VACUUM)
        .build();
    let cube_affine = translate(1.0, 1.0, 5.0) * rotate_yd(15.0);
    let cube = Cube::new(&cube_affine, cube_material.id);

    let cylinder_checkers = UvCheckers {
        width: 16,
        height: 8,
        a: color::White::ALICE_BLUE,
        b: color::Gray::DARK_GRAY,
    };
    let cylinder_texture = CylindricalTexture::new(cylinder_checkers.into());
    let cylinder_material = Material::builder()
        .pattern(Some(cylinder_texture.into()))
        .build();
    let cylinder = Cylinder::new(
        &translate(4.0, 0.0, 0.0),
        cylinder_material.id,
        (0.0, 2.0),
        true,
    );

    let earth_image = Img::load("assets/images/world-map.gif").unwrap();
    let uvimage = UvImage::new(earth_image, true);
    let earth_texture = SphericalTexture::new(uvimage.into());
    let earth_material = Material::builder()
        .pattern(Some(earth_texture.into()))
        .build();
    let earth_affine = translate(-2.0, 1.0, 4.0) * rotate_yd(45.0);
    let earth = Sphere::new(&earth_affine, earth_material.id);

    let backimg = Img::load("assets/images/skybox/back.png").unwrap();
    let frontimg = Img::load("assets/images/skybox/front.png").unwrap();
    let leftimg = Img::load("assets/images/skybox/left.png").unwrap();
    let rightimg = Img::load("assets/images/skybox/right.png").unwrap();
    let topimg = Img::load("assets/images/skybox/top.png").unwrap();
    let bottomimg = Img::load("assets/images/skybox/bottom.png").unwrap();

    let backuv = UvImage::new(backimg, true);
    let frontuv = UvImage::new(frontimg, true);
    let leftuv = UvImage::new(leftimg, true);
    let rightuv = UvImage::new(rightimg, true);
    let topuv = UvImage::new(topimg, true);
    let bottomuv = UvImage::new(bottomimg, true);

    let skybox = Skybox::new(rightuv, leftuv, backuv, frontuv, bottomuv, topuv);
    let skybox_material = Material::builder()
        .pattern(Some(skybox.into()))
        .ambient(1.0)
        .diffuse(0.0)
        .specular(0.0)
        .build();
    let skybox_affine = scale(1_000.0, 1_000.0, 1_000.0);
    let skybox = Cube::new(&skybox_affine, skybox_material.id);

    let car_affine = translate(-4.0, 0.0, 3.0) * rotate_yd(130.0);
    let (car_materials, car) = load_obj("assets/models/Car.obj", &car_affine);

    let torus_affine = translate(0.0, 1.2, 0.0) * rotate_yd(30.0) * rotate_xd(90.0);
    let torus_material = Material::builder().color(color::White::AZURE).build();
    let torus = Torus::new(&torus_affine, torus_material.id, 1.0, 0.2);

    let l1 = Light {
        position: glam::vec3a(-10.0, 10.0, -10.0),
        intensity: color::White::WHITE * 0.8,
    };
    let l2 = Light {
        position: glam::vec3a(10.0, 10.0, -10.0),
        intensity: color::White::WHITE * 0.6,
    };

    let mut materials = vec![
        sphere_material,
        plane_material,
        cylinder_material,
        cube_material,
        earth_material,
        skybox_material,
        torus_material,
    ];
    materials.extend(car_materials);

    let shapes = vec![
        sphere.into(),
        plane.into(),
        cylinder.into(),
        cube.into(),
        earth.into(),
        skybox.into(),
        car,
        torus.into(),
    ];
    let lights = vec![l1, l2];

    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 3.0, -12.0),
        glam::vec3(0.0, 1.0, 0.0),
        Vec3::Y,
    );
    let camera = Camera::new(1280, 720, std::f32::consts::PI / 3.0, camera_view);

    Scene {
        materials: materials,
        shapes: shapes,
        lights: lights,
        camera: camera,
    }
}
