use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};

use glam::Affine3A;
use glam::Vec3A;

use std::convert::From;

enum CsgOp {
    Union,
    Inter,
    Diff,
}

pub struct Csg {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub left: Box<Shape>,
    left_max: usize,
    pub right: Box<Shape>,
    right_max: usize,
    op: CsgOp,
}

impl Csg {
    fn new(tr: &Affine3A, mut left: Shape, mut right: Shape, op: CsgOp) -> Self {
        let mut bounds = Aabb::empty();
        bounds += &left.parent_space_bounds();
        bounds += &right.parent_space_bounds();
        let common = ShapeCommon::new(bounds);
        left.common_mut().parent_id = Some(common.id);
        right.common_mut().parent_id = Some(common.id);
        let left_max = left.max_intersections();
        let right_max = right.max_intersections();
        Self {
            common: common,
            transform: ShapeTransform::new(tr),
            left: Box::new(left),
            left_max: left_max,
            right: Box::new(right),
            right_max: right_max,
            op: op,
        }
    }

    pub fn union(tr: &Affine3A, left: Shape, right: Shape) -> Self {
        Self::new(tr, left, right, CsgOp::Union)
    }

    pub fn intersect(tr: &Affine3A, left: Shape, right: Shape) -> Self {
        Self::new(tr, left, right, CsgOp::Inter)
    }

    pub fn difference(tr: &Affine3A, left: Shape, right: Shape) -> Self {
        Self::new(tr, left, right, CsgOp::Diff)
    }

    fn intersection_allowed(&self, lhit: bool, inl: bool, inr: bool) -> bool {
        match self.op {
            CsgOp::Union => (lhit && !inr) || (!lhit && !inl),
            CsgOp::Inter => (lhit && inr) || (!lhit && inl),
            CsgOp::Diff => (lhit && !inr) || (!lhit && inl),
        }
    }

    fn filter_intersections(&self, intersections: &Vec<Intersection>) -> Vec<Intersection> {
        let mut inl = false;
        let mut inr = false;

        let mut result: Vec<Intersection> = Vec::new();

        for i in intersections {
            let lhit = self.left.find_by_id(i.shape_id).is_some();
            if self.intersection_allowed(lhit, inl, inr) {
                result.push(*i);
            }

            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }

        result
    }
}

impl LocalShape for Csg {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        if !self.common.aabb.intersects(ray) {
            return;
        }
        let mut left_buffer = IntersectionBuffer::new(self.left_max);
        let mut right_buffer = IntersectionBuffer::new(self.right_max);
        self.left.intersect(ray, &mut left_buffer);
        self.right.intersect(ray, &mut right_buffer);
        let mut combined_buffer = IntersectionBuffer::new(
            left_buffer.intersections.len() + right_buffer.intersections.len(),
        );
        combined_buffer
            .intersections
            .extend(left_buffer.intersections);
        combined_buffer
            .intersections
            .extend(right_buffer.intersections);
        combined_buffer.sort();
        let filtered = self.filter_intersections(&combined_buffer.intersections);
        buffer.intersections.extend(filtered);
    }

    fn local_normal(&self, _point: Vec3A, _intersection: Intersection) -> Vec3A {
        panic!("Trying to get normal of CSG")
    }
}

impl From<Csg> for Shape {
    fn from(value: Csg) -> Self {
        Shape::Csg(value)
    }
}
