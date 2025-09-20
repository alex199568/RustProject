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
mod scene2;
pub mod shape;
mod transform;

use crate::light::Light;

fn main() {
    let scene = scene2::scene2();

    render::render(&scene, "renders/uv/image1.png");
}
