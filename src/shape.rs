
use crate::matrix::Matrix;
use crate::point::Point;
use crate::vector::Vector;
use crate::ray::Ray;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::material::Material;

use std::convert::From;

pub struct Sphere {
    inv: Matrix<4>,
    inv_tr: Matrix<4>,
    pub material: Material
}

impl Sphere {

    pub fn new(transform: Matrix<4>, material: Material) -> Self {
        let inv = transform.inverse();
        let inv_tr = inv.transpose();
        Self {
            inv: inv,
            inv_tr: inv_tr,
            material: material
        }
    }

    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
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

    fn local_normal(&self, point: Point) -> Vector {
        point - Point::ZERO
    }

    fn intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        let transformed_ray = &self.inv * ray;
        self.local_intersect(&transformed_ray, buffer, index);
    }

    fn normal(&self, point: Point) -> Vector {
        let shape_point = &self.inv * point;
        let shape_normal = self.local_normal(shape_point);
        let world_normal = &self.inv_tr * shape_normal;
        world_normal.unit()
    }
}

pub enum Shape {
    Sphere(Sphere)
}

impl Shape {

    pub fn intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        match self {
            Shape::Sphere(s) => s.intersect(ray, buffer, index)
        }
    }

    pub fn normal(&self, point: Point) -> Vector {
        match self {
            Shape::Sphere(s) => s.normal(point)
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material
        }
    }
}

impl From<Sphere> for Shape {

    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
