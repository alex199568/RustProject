
use crate::color::Color;
use crate::pattern::Pattern;

use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Material {
    #[builder(default = Color::BLACK)]
    pub color: Color,
    #[builder(default = 0.01)]
    pub ambient: f32,
    #[builder(default = 0.9)]
    pub diffuse: f32,
    #[builder(default = 0.8)]
    pub specular: f32,
    #[builder(default = 200.0)]
    pub shininess: f32,
    #[builder(default = None, setter(strip_option))]
    pub pattern: Option<Pattern>,
    #[builder(default = 0.0)]
    pub reflective: f32
}
