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

        // Hit point in *local* space (plane is y=0 with normal +Y)
        let p = ray.origin + ray.direction * t;

        // Tile size in world units per repeat (adjust as you like)
        let tile_u = 1.0_f32;
        let tile_v = 1.0_f32;

        // Map to [0,1) and handle negatives correctly
        let u = (p.x / tile_u).rem_euclid(1.0);
        let v = (p.z / tile_v).rem_euclid(1.0);

        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t,
            uv: Some(glam::vec2(u, v)),
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
