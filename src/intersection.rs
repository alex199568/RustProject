

use crate::vector::Vector;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Sphere;

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub shape_index: usize,
    pub t: f32
}

pub struct IntersectionBuffer {
    intersections: Vec<Intersection>
}

impl IntersectionBuffer {

    pub fn new(capacity: usize) -> Self {
        Self {
            intersections: Vec::with_capacity(capacity)
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

    pub fn clear(&mut self) {
        self.intersections.clear();
    }
}

pub struct Hit {
    intersection: Intersection,
    pub point: Point,
    pub eye: Vector,
    pub normal: Vector,
    over_point: Point,
    under_point: Point,
    reflect: Vector
}

impl Hit {

    pub fn new(shape: &Sphere, intersection: Intersection, ray: &Ray) -> Self {
        let point = ray.at(intersection.t);
        let eye = -ray.direction;
        let mut normal = shape.normal(point);
        if normal.dot(eye) < 0.0 {
            normal = -normal;
        }
        let tiny_normal = normal * 1e-5;
        let over_point = point + tiny_normal;
        let under_point = point - tiny_normal;
        let reflect = ray.direction.reflect(normal);

        Self {
            intersection: intersection,
            point: point,
            eye: eye,
            normal: normal,
            over_point: over_point,
            under_point: under_point,
            reflect: reflect
        }
    }
}
