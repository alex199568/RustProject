mod aabb;
mod camera;
mod color;
mod img;
mod intersection;
mod light;
mod material;
mod pattern;
mod ray;
mod render;
mod robj;
mod scene;
mod scene1;
pub mod shape;

use glam::Affine3A;
use glam::Vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::light::AreaLight;
use crate::light::Light;
use crate::material::Material;
use crate::pattern::{Checkers, Gradient, Rings, Stripes};
use crate::scene::Scene;
use crate::shape::{Cone, Csg, Cube, Cylinder, Group, Plane, Shape, Sphere};

fn main() {
    let scene = scene1::scene1();

    render::render(&scene, "renders/area_lights.png");
}
