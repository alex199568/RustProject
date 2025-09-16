
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::intersection::{Intersection, IntersectionBuffer};

pub struct Sphere {
    inv: Matrix<4>
}

impl Sphere {

    pub fn new(transform: Matrix<4>) -> Self {
        Self {
            inv: transform.inverse()
        }
    }

    pub fn intersect(&self, ray: Ray, buffer: &mut IntersectionBuffer, index: usize) {
        self.local_intersect(&self.inv * ray, buffer, index);
    }

    fn local_intersect(&self, ray: Ray, buffer: &mut IntersectionBuffer, index: usize) {
        let sphere_to_ray = ray.origin - Point::ZERO;
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

        buffer.add(Intersection{ shape_index: index, t: t0});
        buffer.add(Intersection{ shape_index: index, t: t1});
    }
}
