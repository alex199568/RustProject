use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon};
use std::convert::From;

use glam::Affine3A;
use glam::Vec2;
use glam::Vec3A;

pub struct Triangle {
    pub common: ShapeCommon,
    pub material: Material,
    p1: Vec3A,
    _p2: Vec3A,
    _p3: Vec3A,
    e1: Vec3A,
    e2: Vec3A,
    n1: Vec3A,
    n2: Option<Vec3A>,
    n3: Option<Vec3A>,
    _uv1: Option<Vec2>,
    _uv2: Option<Vec2>,
    _uv3: Option<Vec2>,
}

impl Triangle {
    pub fn new(
        material: Material,
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
            common: ShapeCommon::new(&Affine3A::IDENTITY, bounds),
            material: material,
            p1: p1,
            _p2: p2,
            _p3: p3,
            e1: e1,
            e2: e2,
            n1: n,
            n2: n2,
            n3: n3,
            _uv1: uv1,
            _uv2: uv2,
            _uv3: uv3,
        }
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
        if self.n2.is_some() {
            let n1 = self.n1;
            let n2 = self.n2.unwrap();
            let n3 = self.n3.unwrap();

            let hit_uv = intersection.uv.unwrap_or(Vec2::ZERO);

            let u = hit_uv.x;
            let v = hit_uv.y;

            n2 * u + n3 * v + n1 * (1.0 - u - v)
        } else {
            self.n1
        }
    }
}

impl From<Triangle> for Shape {
    fn from(t: Triangle) -> Self {
        Shape::Triangle(t)
    }
}
