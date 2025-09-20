use crate::color::Color;

use glam::{Affine3A, Vec3A};

use crate::pattern::checkers::Checkers;
use crate::pattern::cube_texture::CubeTexture;
use crate::pattern::cylindrical_texture::CylindricalTexture;
use crate::pattern::gradient::Gradient;
use crate::pattern::planar_texture::PlanarTexture;
use crate::pattern::rings::Rings;
use crate::pattern::skybox::Skybox;
use crate::pattern::spherical_texture::SphericalTexture;
use crate::pattern::stripes::Stripes;

pub struct PatternCommon {
    inv: Affine3A,
}

impl PatternCommon {
    pub fn new(transform: &Affine3A) -> Self {
        Self {
            inv: transform.inverse(),
        }
    }

    fn at(&self, local: &dyn LocalPattern, point: Vec3A) -> Color {
        let pattern_point = &self.inv.transform_point3a(point);
        local.local_at(*pattern_point)
    }
}

pub trait LocalPattern {
    fn local_at(&self, point: Vec3A) -> Color;
}

impl LocalPattern for SphericalTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        self.uv_pattern.uv_pattern_at(Self::map_point(point))
    }
}

pub enum Pattern {
    Stripes(Stripes),
    Gradient(Gradient),
    Rings(Rings),
    Checkers(Checkers),
    SphericalTexture(SphericalTexture),
    PlanarTexture(PlanarTexture),
    CylindricalTexture(CylindricalTexture),
    CubeTexture(CubeTexture),
    Skybox(Skybox),
}

impl Pattern {
    pub fn at(&self, point: Vec3A) -> Color {
        match self {
            Pattern::Stripes(s) => s.common.at(s, point),
            Pattern::Gradient(g) => g.common.at(g, point),
            Pattern::Rings(r) => r.common.at(r, point),
            Pattern::Checkers(c) => c.common.at(c, point),
            Pattern::SphericalTexture(st) => st.common.at(st, point),
            Pattern::PlanarTexture(pt) => pt.common.at(pt, point),
            Pattern::CylindricalTexture(ct) => ct.common.at(ct, point),
            Pattern::CubeTexture(ct) => ct.common.at(ct, point),
            Pattern::Skybox(s) => s.common.at(s, point),
        }
    }
}
