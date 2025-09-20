use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon};
use std::convert::From;

use glam::Vec2;
use glam::Vec3A;

pub struct Triangle {
    pub common: ShapeCommon,
    pub material_id: usize,
    p1: Vec3A,
    e1: Vec3A,
    e2: Vec3A,
    n1: Vec3A,
    n2: Vec3A,
    n3: Vec3A,
    uv1: Vec2,
    uv2: Vec2,
    uv3: Vec2,
}

impl Triangle {
    pub fn new(
        material: usize,
        p1: Vec3A,
        p2: Vec3A,
        p3: Vec3A,
        n1: Option<Vec3A>,
        n2: Option<Vec3A>,
        n3: Option<Vec3A>,
        uv1: Option<Vec2>,
        uv2: Option<Vec2>,
        uv3: Option<Vec2>,
    ) -> Self {
        let mut bounds = Aabb::default();
        bounds += p1;
        bounds += p2;
        bounds += p3;

        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let n = n1.unwrap_or(e2.cross(e1));
        Self {
            common: ShapeCommon::new(bounds),
            material_id: material,
            p1: p1,
            e1: e1,
            e2: e2,
            n1: n,
            n2: n2.unwrap_or(n),
            n3: n3.unwrap_or(n),
            uv1: uv1.unwrap_or(Vec2::ZERO),
            uv2: uv2.unwrap_or(Vec2::ZERO),
            uv3: uv3.unwrap_or(Vec2::ZERO),
        }
    }

    pub fn local_uv(&self, point: Vec3A) -> Vec2 {
        let v0 = self.e1;
        let v1 = self.e2;
        let v2 = point - self.p1;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;
        if denom == 0.0 {
            return self.uv1;
        }

        let v = (d20 * d11 - d21 * d01) / denom;
        let w = (d21 * d00 - d20 * d01) / denom;
        let u = 1.0 - v - w;

        self.uv1 * u + self.uv2 * v + self.uv3 * w
    }
}

impl LocalShape for Triangle {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let dir_cross_e2 = ray.direction.cross(self.e2);
        let det = self.e1.dot(dir_cross_e2);
        if det.abs() < 1e-6 {
            return;
        }
        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * p1_to_origin.dot(dir_cross_e2);
        if u < 0.0 || u > 1.0 {
            return;
        }
        let origin_cross_e1 = p1_to_origin.cross(self.e1);
        let v = f * ray.direction.dot(origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return;
        }
        let t = f * self.e2.dot(origin_cross_e1);
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t,
            uv: Some(glam::vec2(u, v)),
        });
    }

    fn local_normal(&self, _point: Vec3A, intersection: Intersection) -> Vec3A {
        let hit_uv = intersection.uv.unwrap_or(Vec2::ZERO);

        let u = hit_uv.x;
        let v = hit_uv.y;

        self.n2 * u + self.n3 * v + self.n1 * (1.0 - u - v)
    }
}

impl From<Triangle> for Shape {
    fn from(t: Triangle) -> Self {
        Shape::Triangle(t)
    }
}
