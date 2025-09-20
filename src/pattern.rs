use crate::color::Color;

use glam::Affine3A;
use glam::Vec2;
use glam::Vec3A;

use std::convert::From;

struct PatternCommon {
    inv: Affine3A,
}

impl PatternCommon {
    fn new(transform: &Affine3A) -> Self {
        Self {
            inv: transform.inverse(),
        }
    }

    fn at(&self, local: &dyn LocalPattern, point: Vec3A) -> Color {
        let pattern_point = &self.inv.transform_point3a(point);
        local.local_at(*pattern_point)
    }
}

trait LocalPattern {
    fn local_at(&self, point: Vec3A) -> Color;
}

pub struct Stripes {
    common: PatternCommon,
    a: Color,
    b: Color,
}

impl Stripes {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
        }
    }
}

impl LocalPattern for Stripes {
    fn local_at(&self, point: Vec3A) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct Gradient {
    common: PatternCommon,
    a: Color,
    b: Color,
}

impl Gradient {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
        }
    }
}

impl LocalPattern for Gradient {
    fn local_at(&self, point: Vec3A) -> Color {
        self.a + (self.b - self.a) * (point.x - point.x.floor())
    }
}

pub struct Rings {
    common: PatternCommon,
    a: Color,
    b: Color,
}

impl Rings {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
        }
    }
}

impl LocalPattern for Rings {
    fn local_at(&self, point: Vec3A) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct Checkers {
    common: PatternCommon,
    a: Color,
    b: Color,
}

impl Checkers {
    pub fn new(transform: &Affine3A, a: Color, b: Color) -> Self {
        Self {
            common: PatternCommon::new(transform),
            a: a,
            b: b,
        }
    }
}

impl LocalPattern for Checkers {
    fn local_at(&self, point: Vec3A) -> Color {
        if (point.x.round() + point.y.round() + point.z.round()) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct SphericalTexture {
    common: PatternCommon,
    uv_pattern: UvCheckers,
}

impl SphericalTexture {
    fn map_point(p: Vec3A) -> Vec2 {
        let theta = libm::atan2f(p.x, p.z);
        let r = p.length();
        let phi = libm::acosf(p.y / r);
        let raw_u = theta / (std::f32::consts::PI * 2.0);
        let u = 1.0 - (raw_u + 0.5);
        let v = 1.0 - phi / std::f32::consts::PI;
        glam::vec2(u, v)
    }

    pub fn new(transform: &Affine3A, uv_pattern: UvCheckers) -> Self {
        Self {
            common: PatternCommon::new(transform),
            uv_pattern: uv_pattern,
        }
    }
}

impl LocalPattern for SphericalTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        let uv = Self::map_point(point);
        self.uv_pattern.uv_pattern_at(uv.x, uv.y)
    }
}

pub struct PlanarTexture {
    common: PatternCommon,
    uv_pattern: UvCheckers,
}

impl PlanarTexture {
    pub fn new(uv_pattern: UvCheckers) -> Self {
        Self {
            common: PatternCommon::new(&Affine3A::IDENTITY),
            uv_pattern: uv_pattern,
        }
    }

    fn map_point(point: Vec3A) -> Vec2 {
        glam::vec2(point.x.fract(), point.z.fract())
    }
}

impl LocalPattern for PlanarTexture {
    fn local_at(&self, point: Vec3A) -> Color {
        let uv = Self::map_point(point);
        self.uv_pattern.uv_pattern_at(uv.x, uv.y)
    }
}

pub enum Pattern {
    Stripes(Stripes),
    Gradient(Gradient),
    Rings(Rings),
    Checkers(Checkers),
    SphericalTexture(SphericalTexture),
    PlanarTexture(PlanarTexture),
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
        }
    }
}

impl From<Stripes> for Pattern {
    fn from(s: Stripes) -> Self {
        Pattern::Stripes(s)
    }
}

impl From<Gradient> for Pattern {
    fn from(g: Gradient) -> Self {
        Pattern::Gradient(g)
    }
}

impl From<Rings> for Pattern {
    fn from(r: Rings) -> Self {
        Pattern::Rings(r)
    }
}

impl From<Checkers> for Pattern {
    fn from(c: Checkers) -> Self {
        Pattern::Checkers(c)
    }
}

impl From<SphericalTexture> for Pattern {
    fn from(t: SphericalTexture) -> Self {
        Pattern::SphericalTexture(t)
    }
}

impl From<PlanarTexture> for Pattern {
    fn from(t: PlanarTexture) -> Self {
        Pattern::PlanarTexture(t)
    }
}

pub struct UvCheckers {
    pub width: usize,
    pub height: usize,
    pub a: Color,
    pub b: Color,
}

impl UvCheckers {
    pub fn uv_pattern_at(&self, u: f32, v: f32) -> Color {
        let u2 = (u * self.width as f32).floor();
        let v2 = (v * self.height as f32).floor();
        if (u2 + v2) as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
