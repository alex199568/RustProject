use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Sphere {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub material_id: usize,
}

impl Sphere {
    pub fn new(transform: &Affine3A, material: usize) -> Self {
        let bounds = Aabb::new(glam::vec3a(-1.0, -1.0, -1.0), glam::vec3a(1.0, 1.0, 1.0));
        Self {
            common: ShapeCommon::new(bounds),
            transform: ShapeTransform::new(transform),
            material_id: material,
        }
    }
}

impl LocalShape for Sphere {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let sphere_to_ray = ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return;
        }

        let sd = d.sqrt();
        let t0 = (-b - sd) / (2.0 * a);
        let t1 = (-b + sd) / (2.0 * a);

        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t0,
            uv: None,
        });
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t1,
            uv: None,
        });
    }

    fn local_normal(&self, point: Vec3A, _intersection: Intersection) -> Vec3A {
        point
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
