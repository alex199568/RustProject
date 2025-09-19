use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::caps::Caps;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Cylinder {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub material_id: usize,
    range: (f32, f32),
    caps: bool,
}

impl Cylinder {
    pub fn new(tr: &Affine3A, material: usize, range: (f32, f32), caps: bool) -> Self {
        let bounds = Aabb::new(
            glam::vec3a(-1.0, range.0, -1.0),
            glam::vec3a(1.0, range.1, 1.0),
        );
        Self {
            common: ShapeCommon::new(bounds),
            transform: ShapeTransform::new(tr),
            material_id: material,
            range: range,
            caps: caps,
        }
    }
}

impl Caps for Cylinder {
    #[inline]
    fn caps(&self) -> bool {
        self.caps
    }

    #[inline]
    fn from(&self) -> f32 {
        self.range.0
    }

    #[inline]
    fn to(&self) -> f32 {
        self.range.1
    }
}

impl LocalShape for Cylinder {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        self.intersect_caps(ray, buffer, self.common.id);
        let o = ray.origin;
        let d = ray.direction;
        let a = d.x * d.x + d.z * d.z;
        if a.abs() < 1e-6 {
            return;
        }
        let b = 2.0 * o.x * d.x + 2.0 * o.z * d.z;
        let c = o.x * o.x + o.z * o.z - 1.0;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return;
        }
        let sd = disc.sqrt();
        let t0 = (-b - sd) / (2.0 * a);
        let mut y0 = o.y + t0 * d.y;
        if self.range.0 < y0 && y0 < self.range.1 {
            buffer.add(Intersection {
                shape_id: self.common.id,
                t: t0,
                uv: None,
            });
        }
        let t1 = (-b + sd) / (2.0 * a);
        y0 = o.y + t1 * d.y;
        if self.range.0 < y0 && y0 < self.range.1 {
            buffer.add(Intersection {
                shape_id: self.common.id,
                t: t1,
                uv: None,
            });
        }
    }

    fn local_normal(&self, point: Vec3A, _intersection: Intersection) -> Vec3A {
        let distance = point.x * point.x + point.z * point.z;
        if distance < 1.0 && point.y >= self.range.1 - 1e-5 {
            return glam::vec3a(0.0, 1.0, 0.0);
        }
        if distance < 1.0 && point.y <= self.range.0 + 1e-5 {
            return glam::vec3a(0.0, -1.0, 0.0);
        }
        glam::vec3a(point.x, 0.0, point.z)
    }
}

impl From<Cylinder> for Shape {
    fn from(c: Cylinder) -> Self {
        Shape::Cylinder(c)
    }
}
