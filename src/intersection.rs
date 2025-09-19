use crate::ray::Ray;
use crate::scene::Scene;
use crate::shape::Shape;

use std::cmp::Ordering;

use glam::Vec2;
use glam::Vec3A;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub shape_id: usize,
    pub t: f32,
    pub uv: Option<Vec2>,
}

pub struct IntersectionBuffer {
    pub intersections: Vec<Intersection>,
}

impl IntersectionBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            intersections: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, i: Intersection) {
        self.intersections.push(i);
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.intersections
            .iter()
            .copied()
            .filter(|i| i.t > 1e-5)
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }

    pub fn hit_ignoring(&self, ignore_id: usize) -> Option<Intersection> {
        self.intersections
            .iter()
            .copied()
            .filter(|i| i.t > 1e-5 && i.shape_id != ignore_id)
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }

    pub fn clear(&mut self) {
        self.intersections.clear();
    }

    pub fn sort(&mut self) {
        self.intersections
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal))
    }
}

pub struct Hit {
    pub shape_id: usize,
    pub point: Vec3A,
    pub eye: Vec3A,
    pub normal: Vec3A,
    pub over_point: Vec3A,
    pub under_point: Vec3A,
    pub reflect: Vec3A,
}

impl Hit {
    pub fn new(shape: &Shape, intersection: Intersection, ray: &Ray, scene: &Scene) -> Self {
        let point = ray.at(intersection.t);
        let eye = -ray.direction;

        let mut normal = shape.normal(point, scene, intersection);
        if normal.dot(eye) < 0.0 {
            normal = -normal;
        }
        let tiny_normal = normal * 1e-5;
        let over_point = point + tiny_normal;
        let under_point = point - tiny_normal;
        let reflect = ray.direction.reflect(normal);

        Self {
            shape_id: intersection.shape_id,
            point: point,
            eye: eye,
            normal: normal,
            over_point: over_point,
            under_point: under_point,
            reflect: reflect,
        }
    }
}
