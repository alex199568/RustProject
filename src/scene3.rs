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

pub fn scene3() -> Scene {
    let plane_albedo = UvImage::scaled(
        Img::load("assets/StoneWall/albedo.tif").unwrap(),
        glam::vec2(4.0, 4.0),
    );
    let plane_texture = CubeTexture::new(plane_albedo.into());
    let plane_normal = UvImage::scaled(
        Img::load("assets/StoneWall/normal.tif").unwrap(),
        glam::vec2(4.0, 4.0),
    );
    let plane_normal_texture = CubeTexture::new(plane_normal.into());
    let plane_material = Material::builder()
        .pattern(Some(plane_texture.into()))
        .normal_pattern(Some(plane_normal_texture.into()))
        .build();

    let floor_affine = translate(0.0, -1.0, 0.0) * scale(8.0, 1.0, 8.0);
    let floor = Cube::new(&floor_affine, plane_material.id);

    let tristar_affine = translate(2.0, 0.0, 3.0) * rotate_yd(-45.0);
    let (tristar_materials, tristar) =
        load_obj("assets/models/Tristar/tristarRacer.obj", &tristar_affine);

    let woman_affine = scale(0.1, 0.1, 0.1);
    let (woman_materials, woman) = load_obj("assets/models/woman/woman.obj", &woman_affine);

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

    let l1 = Light {
        position: glam::vec3a(0.3, 1.0, -0.3).normalize() * 10.0,
        intensity: color::White::WHITE * 0.8,
    };
    let l2 = Light {
        position: glam::vec3a(-0.3, 1.0, -0.3).normalize() * 10.0,
        intensity: color::White::WHITE * 0.6,
    };

    let mut materials = vec![plane_material, skybox_material];
    materials.extend(tristar_materials);
    materials.extend(woman_materials);

    let shapes = vec![floor.into(), skybox.into(), tristar, woman];
    let lights = vec![l1, l2];
    let camera_view = Affine3A::look_at_rh(
        glam::vec3(0.0, 2.0, -6.0),
        glam::vec3(1.0, 1.0, 0.0),
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
