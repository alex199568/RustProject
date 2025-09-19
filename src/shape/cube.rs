use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Cube {
    pub common: ShapeCommon,
    pub material: Material,
}

impl Cube {
    pub fn new(transform: &Affine3A, material: Material) -> Self {
        let bounds = Aabb::new(glam::vec3a(-1.0, -1.0, -1.0), glam::vec3a(1.0, 1.0, 1.0));
        Self {
            common: ShapeCommon::new(transform, bounds),
            material: material,
        }
    }
}

impl LocalShape for Cube {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let (xtmin, xtmax) = Aabb::check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (ytmin, ytmax) = Aabb::check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (ztmin, ztmax) = Aabb::check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin > tmax {
            return;
        }
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: tmin,
        });
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: tmax,
        });
    }

    fn local_normal(&self, point: Vec3A) -> Vec3A {
        let x = point.x.abs();
        let y = point.y.abs();
        let z = point.z.abs();
        let maxc = x.max(y).max(z);
        if (maxc - x) < 1e-6 {
            return glam::vec3a(point.x, 0.0, 0.0);
        }
        if (maxc - y) < 1e-6 {
            return glam::vec3a(0.0, point.y, 0.0);
        }

        glam::vec3a(0.0, 0.0, point.z)
    }
}

impl From<Cube> for Shape {
    fn from(c: Cube) -> Self {
        Shape::Cube(c)
    }
}
