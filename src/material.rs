use crate::color;
use crate::color::Color;
use crate::pattern::Pattern;

use typed_builder::TypedBuilder;

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_MATERIAL_ID: AtomicUsize = AtomicUsize::new(1);

fn next_material_id() -> usize {
    NEXT_MATERIAL_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(TypedBuilder)]
pub struct Material {
    #[builder(default = next_material_id(), setter(skip))]
    pub id: usize,
    #[builder(default = color::Gray::BLACK)]
    pub color: Color,
    #[builder(default = 0.01)]
    pub ambient: f32,
    #[builder(default = 0.9)]
    pub diffuse: f32,
    #[builder(default = 0.8)]
    pub specular: f32,
    #[builder(default = 200.0)]
    pub shininess: f32,
    #[builder(default = None)]
    pub pattern: Option<Pattern>,
    #[builder(default = None)]
    pub normal_pattern: Option<Pattern>,
    #[builder(default = 0.0)]
    pub reflection: f32,
    #[builder(default = 0.0)]
    pub transparency: f32,
    #[builder(default = 1.0)]
    pub refraction: f32,
}

impl Material {
    pub const IOR_VACUUM: f32 = 1.0;
    pub const IOR_AIR: f32 = 1.00029;
    pub const IOR_WATER: f32 = 1.333;
    pub const IOR_GLASS: f32 = 1.52;
    pub const IOR_DIAMOND: f32 = 2.417;
}
