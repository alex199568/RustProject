use std::sync::atomic::{AtomicUsize, Ordering};

use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::scene::Scene;

use crate::shape::cone::Cone;
use crate::shape::cube::Cube;
use crate::shape::cylinder::Cylinder;
use crate::shape::group::Group;
use crate::shape::plane::Plane;
use crate::shape::sphere::Sphere;
use crate::shape::triangle::Triangle;

use glam::Affine3A;
use glam::Mat3A;
use glam::Vec3A;

static NEXT_SHAPE_ID: AtomicUsize = AtomicUsize::new(1);

pub struct ShapeCommon {
    pub tr: Affine3A,
    inv: Affine3A,
    inv_tr: Mat3A,
    pub id: usize,
    pub parent_id: Option<usize>,
    pub aabb: Aabb,
}

pub trait LocalShape {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer);
    fn local_normal(&self, point: Vec3A, intersection: Intersection) -> Vec3A;
}

impl ShapeCommon {
    pub fn new(transform: &Affine3A, aabb: Aabb) -> Self {
        let inv = transform.inverse();
        let inv_tr = inv.matrix3.transpose();
        let id = NEXT_SHAPE_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            tr: *transform,
            inv: inv,
            inv_tr: inv_tr,
            id: id,
            parent_id: None,
            aabb: aabb,
        }
    }

    fn intersect(&self, local: &dyn LocalShape, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let transformed_ray = &self.inv * ray;
        local.local_intersect(&transformed_ray, buffer);
    }

    fn normal(&self, local: &dyn LocalShape, point: Vec3A, intersection: Intersection) -> Vec3A {
        let shape_point = self.inv.transform_point3a(point);
        let shape_normal = local.local_normal(shape_point, intersection);
        let world_normal = self.inv_tr * shape_normal;
        world_normal.normalize()
    }
}

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Triangle(Triangle),
    Group(Group),
}

impl Shape {
    pub fn common(&self) -> &ShapeCommon {
        match self {
            Shape::Sphere(s) => &s.common,
            Shape::Plane(p) => &p.common,
            Shape::Cube(c) => &c.common,
            Shape::Cylinder(c) => &c.common,
            Shape::Cone(c) => &c.common,
            Shape::Triangle(t) => &t.common,
            Shape::Group(g) => &g.common,
        }
    }

    pub fn common_mut(&mut self) -> &mut ShapeCommon {
        match self {
            Shape::Sphere(s) => &mut s.common,
            Shape::Plane(p) => &mut p.common,
            Shape::Cube(c) => &mut c.common,
            Shape::Cylinder(c) => &mut c.common,
            Shape::Cone(c) => &mut c.common,
            Shape::Triangle(t) => &mut t.common,
            Shape::Group(g) => &mut g.common,
        }
    }

    pub fn intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        match self {
            Shape::Sphere(s) => s.common.intersect(s, ray, buffer),
            Shape::Plane(p) => p.common.intersect(p, ray, buffer),
            Shape::Cube(c) => c.common.intersect(c, ray, buffer),
            Shape::Cylinder(c) => c.common.intersect(c, ray, buffer),
            Shape::Cone(c) => c.common.intersect(c, ray, buffer),
            Shape::Triangle(t) => t.local_intersect(ray, buffer),
            Shape::Group(g) => g.common.intersect(g, ray, buffer),
        }
    }

    pub fn normal(&self, point: Vec3A, scene: &Scene, intersection: Intersection) -> Vec3A {
        let mut p = point;
        let mut parent_id: Option<usize> = self.common().parent_id;
        while parent_id.is_some() {
            let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
            p = parent_shape.inv().transform_point3a(p);
            parent_id = parent_shape.common().parent_id;
        }

        let mut shape_normal = match self {
            Shape::Sphere(s) => s.common.normal(s, p, intersection),
            Shape::Plane(pl) => pl.common.normal(pl, p, intersection),
            Shape::Cube(c) => c.common.normal(c, p, intersection),
            Shape::Cylinder(c) => c.common.normal(c, p, intersection),
            Shape::Cone(c) => c.common.normal(c, p, intersection),
            Shape::Triangle(t) => t.common.normal(t, p, intersection),
            Shape::Group(g) => g.common.normal(g, p, intersection),
        };

        parent_id = self.common().parent_id;
        while parent_id.is_some() {
            let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
            shape_normal = parent_shape.inv_tr() * shape_normal;
            shape_normal = shape_normal.normalize();
            parent_id = parent_shape.common().parent_id;
        }

        shape_normal
    }

    pub fn material_id(&self) -> usize {
        match self {
            Shape::Sphere(s) => s.material_id,
            Shape::Plane(p) => p.material_id,
            Shape::Cube(c) => c.material_id,
            Shape::Cylinder(c) => c.material_id,
            Shape::Cone(c) => c.material_id,
            Shape::Triangle(t) => t.material_id,
            Shape::Group(g) => g.children[0].material_id(),
        }
    }

    pub fn inv(&self) -> &Affine3A {
        &self.common().inv
    }

    pub fn inv_tr(&self) -> &Mat3A {
        &self.common().inv_tr
    }

    pub fn max_intersections(&self) -> usize {
        match self {
            Shape::Sphere(_) => 2,
            Shape::Plane(_) => 1,
            Shape::Cube(_) => 2,
            Shape::Cylinder(_) => 2,
            Shape::Cone(_) => 4,
            Shape::Triangle(_) => 1,
            Shape::Group(g) => g.children.iter().map(|c| c.max_intersections()).sum(),
        }
    }

    pub fn parent_id(&self) -> Option<usize> {
        self.common().parent_id
    }

    pub fn find_by_id(&self, id: usize) -> Option<&Shape> {
        if self.common().id == id {
            return Some(self);
        }
        if let Shape::Group(g) = self {
            // try children recursively and return the first match
            return g.children.iter().find_map(|c| c.find_by_id(id));
        }
        None
    }

    pub fn divide(&mut self, threshold: usize) {
        if let Shape::Group(g) = self {
            g.divide(threshold);
        }
    }
}
