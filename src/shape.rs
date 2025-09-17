
use crate::matrix::Matrix;
use crate::point::Point;
use crate::vector::Vector;
use crate::ray::Ray;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::material::Material;

use std::convert::From;

struct ShapeCommon {
    inv: Matrix<4>,
    inv_tr: Matrix<4>
}

trait LocalShape {

    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize);
    fn local_normal(&self, point: Point) -> Vector;
}

impl ShapeCommon {

    fn new(transform: &Matrix<4>) -> Self {
        let inv = transform.inverse();
        let inv_tr = inv.transpose();
        Self {
            inv: inv,
            inv_tr: inv_tr
        }
    }

    fn intersect(&self, local: &dyn LocalShape, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        let transformed_ray = &self.inv * ray;
        local.local_intersect(&transformed_ray, buffer, index);
    }

    fn normal(&self, local: &dyn LocalShape, point: Point) -> Vector {
        let shape_point = &self.inv * point;
        let shape_normal = local.local_normal(shape_point);
        let world_normal = &self.inv_tr * shape_normal;
        world_normal.unit()
    }
}

pub struct Sphere {
    common: ShapeCommon,
    material: Material
}

impl Sphere {

    pub fn new(transform: &Matrix<4>, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material
        }
    }
}

impl LocalShape for Sphere {

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
}

pub struct Plane {
    common: ShapeCommon,
    material: Material
}

impl Plane {

    pub fn new(transform: &Matrix<4>, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material
        }
    }
}

impl LocalShape for Plane {

    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        if ray.direction.y.abs() < 1e-5 {
            return;
        }
        let t = -ray.origin.y / ray.direction.y;
        buffer.add(Intersection{shape_index: index, t: t});
    }

    fn local_normal(&self, _point: Point) -> Vector {
        Vector::Y
    }
}

pub struct Cube {
    common: ShapeCommon,
    material: Material
}

impl Cube {

    pub fn new(transform: &Matrix<4>, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material
        }
    }

    fn check_axis(o: f32, d: f32, from: f32, to: f32) -> (f32, f32) {
        let t_min_num = from - o;
        let t_max_num = to - o;
        let t_min = t_min_num / d;
        let t_max = t_max_num / d;
        if t_min < t_max {
            (t_min, t_max)
        } else {
            (t_max, t_min)
        }
    }
}

impl LocalShape for Cube {

    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        let (xtmin, xtmax) = Self::check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (ytmin, ytmax) = Self::check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (ztmin, ztmax) = Self::check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin > tmax { return; }
        buffer.add(Intersection{shape_index: index, t: tmin});
        buffer.add(Intersection{shape_index: index, t: tmax});
    }

    fn local_normal(&self, point: Point) -> Vector {
        let x = point.x.abs();
        let y = point.y.abs();
        let z = point.z.abs();
        let maxc = x.max(y).max(z);
        if (maxc - x) < 1e-6 {
            return Vector{x: point.x, y: 0.0, z: 0.0};
        }
        if (maxc - y) < 1e-6 {
            return Vector{x: 0.0, y: point.y, z: 0.0};
        }

        Vector {x: 0.0, y: 0.0, z: point.z}
    }
}

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube)
}

impl Shape {

    pub fn intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, index: usize) {
        match self {
            Shape::Sphere(s) => s.common.intersect(s, ray, buffer, index),
            Shape::Plane(p) => p.common.intersect(p, ray, buffer, index),
            Shape::Cube(c) => c.common.intersect(c, ray, buffer, index)
        }
    }

    pub fn normal(&self, point: Point) -> Vector {
        match self {
            Shape::Sphere(s) => s.common.normal(s, point),
            Shape::Plane(p) => p.common.normal(p, point),
            Shape::Cube(c) => c.common.normal(c, point)
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material,
            Shape::Plane(p) => &p.material,
            Shape::Cube(c) => &c.material
        }
    }

    pub fn inv(&self) -> &Matrix<4> {
        match self {
            Shape::Sphere(s) => &s.common.inv,
            Shape::Plane(p) => &p.common.inv,
            Shape::Cube(c) => &c.common.inv
        }
    }
}

impl From<Sphere> for Shape {

    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}

impl From<Plane> for Shape {

    fn from(p: Plane) -> Self {
        Shape::Plane(p)
    }
}

impl From<Cube> for Shape {

    fn from(c: Cube) -> Self {
        Shape::Cube(c)
    }
}
