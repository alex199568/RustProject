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

pub struct ShapeTransform {
    pub tr: Affine3A,
    inv: Affine3A,
    inv_tr: Mat3A,
}

impl ShapeTransform {
    pub fn new(transform: &Affine3A) -> Self {
        let inv = transform.inverse();
        let inv_tr = inv.matrix3.transpose();
        Self {
            tr: *transform,
            inv: inv,
            inv_tr: inv_tr,
        }
    }
}

pub struct ShapeCommon {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub aabb: Aabb,
}

pub trait LocalShape {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer);
    fn local_normal(&self, point: Vec3A, intersection: Intersection) -> Vec3A;
}

impl ShapeCommon {
    pub fn new(aabb: Aabb) -> Self {
        let id = NEXT_SHAPE_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id: id,
            parent_id: None,
            aabb: aabb,
        }
    }

    fn intersect(
        &self,
        transform: &ShapeTransform,
        local: &dyn LocalShape,
        ray: &Ray,
        buffer: &mut IntersectionBuffer,
    ) {
        let transformed_ray = &transform.inv * ray;
        local.local_intersect(&transformed_ray, buffer);
    }

    fn normal(
        &self,
        transform: &ShapeTransform,
        local: &dyn LocalShape,
        point: Vec3A,
        intersection: Intersection,
    ) -> Vec3A {
        let shape_point = transform.inv.transform_point3a(point);
        let shape_normal = local.local_normal(shape_point, intersection);
        let world_normal = transform.inv_tr * shape_normal;
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

    pub fn parent_space_bounds(&self) -> Aabb {
        match self {
            Shape::Sphere(s) => &s.common.aabb * &s.transform.tr,
            Shape::Plane(p) => &p.common.aabb * &p.transform.tr,
            Shape::Cube(c) => &c.common.aabb * &c.transform.tr,
            Shape::Cylinder(c) => &c.common.aabb * &c.transform.tr,
            Shape::Cone(c) => &c.common.aabb * &c.transform.tr,
            Shape::Triangle(t) => t.common.aabb.clone(),
            Shape::Group(g) => &g.common.aabb * &g.transform.tr,
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
            Shape::Sphere(s) => s.common.intersect(&s.transform, s, ray, buffer),
            Shape::Plane(p) => p.common.intersect(&p.transform, p, ray, buffer),
            Shape::Cube(c) => c.common.intersect(&c.transform, c, ray, buffer),
            Shape::Cylinder(c) => c.common.intersect(&c.transform, c, ray, buffer),
            Shape::Cone(c) => c.common.intersect(&c.transform, c, ray, buffer),
            Shape::Triangle(t) => t.local_intersect(ray, buffer),
            Shape::Group(g) => g.common.intersect(&g.transform, g, ray, buffer),
        }
    }

    pub fn normal(&self, point: Vec3A, scene: &Scene, intersection: Intersection) -> Vec3A {
        let mut p = point;
        let mut parent_id: Option<usize> = self.common().parent_id;
        while parent_id.is_some() {
            let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
            p = parent_shape.transform_point(p);
            parent_id = parent_shape.common().parent_id;
        }

        let mut shape_normal = match self {
            Shape::Sphere(s) => s.common.normal(&s.transform, s, p, intersection),
            Shape::Plane(pl) => pl.common.normal(&pl.transform, pl, p, intersection),
            Shape::Cube(c) => c.common.normal(&c.transform, c, p, intersection),
            Shape::Cylinder(c) => c.common.normal(&c.transform, c, p, intersection),
            Shape::Cone(c) => c.common.normal(&c.transform, c, p, intersection),
            Shape::Triangle(t) => t.local_normal(p, intersection),
            Shape::Group(g) => g.common.normal(&g.transform, g, p, intersection),
        };

        parent_id = self.common().parent_id;
        while parent_id.is_some() {
            let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
            shape_normal = parent_shape.transform_normal(shape_normal);
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

    pub fn transform_point(&self, point: Vec3A) -> Vec3A {
        match self {
            Shape::Sphere(s) => s.transform.inv.transform_point3a(point),
            Shape::Plane(p) => p.transform.inv.transform_point3a(point),
            Shape::Cube(c) => c.transform.inv.transform_point3a(point),
            Shape::Cylinder(c) => c.transform.inv.transform_point3a(point),
            Shape::Cone(c) => c.transform.inv.transform_point3a(point),
            Shape::Triangle(_) => point,
            Shape::Group(g) => g.transform.inv.transform_point3a(point),
        }
    }

    pub fn transform_normal(&self, normal: Vec3A) -> Vec3A {
        match self {
            Shape::Sphere(s) => s.transform.inv_tr * normal,
            Shape::Plane(p) => p.transform.inv_tr * normal,
            Shape::Cube(c) => c.transform.inv_tr * normal,
            Shape::Cone(c) => c.transform.inv_tr * normal,
            Shape::Cylinder(c) => c.transform.inv_tr * normal,
            Shape::Triangle(_) => normal,
            Shape::Group(g) => g.transform.inv_tr * normal,
        }
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
