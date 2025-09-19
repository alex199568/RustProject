use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Plane {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub material_id: usize,
}

impl Plane {
    pub fn new(transform: &Affine3A, material: usize) -> Self {
        let bounds = Aabb::new(
            glam::vec3a(std::f32::MIN, 0.0, std::f32::MIN),
            glam::vec3a(std::f32::MAX, 0.0, std::f32::MAX),
        );
        Self {
            common: ShapeCommon::new(bounds),
            transform: ShapeTransform::new(transform),
            material_id: material,
        }
    }
}

impl LocalShape for Plane {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        if ray.direction.y.abs() < 1e-5 {
            return;
        }
        let t = -ray.origin.y / ray.direction.y;
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t,
            uv: None,
        });
    }

    fn local_normal(&self, _point: Vec3A, _intersection: Intersection) -> Vec3A {
        Vec3A::Y
    }
}

impl From<Plane> for Shape {
    fn from(p: Plane) -> Self {
        Shape::Plane(p)
    }
}
