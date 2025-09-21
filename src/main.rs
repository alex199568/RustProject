mod aabb;
mod camera;
mod color;
mod img;
mod intersection;
mod light;
mod material;
pub mod pattern;
mod ray;
mod render;
mod robj;
mod scene;
mod scene3;
pub mod shape;
mod transform;

use crate::light::Light;

fn main() {
    let scene = scene3::scene3();

    render::render(&scene, "renders/scene3_3.png");
}
