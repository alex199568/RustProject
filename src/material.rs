
use crate::color::Color;
use crate::pattern::Pattern;

pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub pattern: Option<Pattern>
}

impl Material {

    pub fn color(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color: color,
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
            shininess: shininess,
            pattern: None
        }
    }

    pub fn pattern(pattern: Pattern, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color: Color::BLACK,
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
            shininess: shininess,
            pattern: Some(pattern)
        }
    }
}
