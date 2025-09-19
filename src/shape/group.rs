use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Group {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub children: Vec<Shape>,
}

impl Group {
    pub fn new(tr: &Affine3A, mut children: Vec<Shape>) -> Self {
        let mut bounds = Aabb::default();
        for child in &children {
            bounds += &child.parent_space_bounds();
        }

        let common = ShapeCommon::new(bounds);
        for child in &mut children {
            child.common_mut().parent_id = Some(common.id)
        }
        Self {
            common: common,
            transform: ShapeTransform::new(tr),
            children: children,
        }
    }

    fn classify(&mut self) -> (Vec<Shape>, Vec<Shape>, Vec<Shape>) {
        let (leftb, rightb) = self.common.aabb.split();

        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut stay = Vec::new();

        // Move children out of self
        for child in std::mem::take(&mut self.children) {
            let parent_space_bounds = child.parent_space_bounds();

            if leftb.contains_aabb(&parent_space_bounds) {
                left.push(child);
            } else if rightb.contains_aabb(&parent_space_bounds) {
                right.push(child);
            } else {
                stay.push(child);
            }
        }

        (left, right, stay)
    }

    fn make_sub_group(&mut self, children: Vec<Shape>) {
        let mut g = Group::new(&Affine3A::IDENTITY, children);
        g.common.parent_id = Some(self.common.id);
        self.children.push(g.into());
    }

    pub fn divide(&mut self, threshold: usize) {
        // if fewer than threshold, just recurse into children and stop
        if self.children.len() < threshold {
            for child in &mut self.children {
                child.divide(threshold);
            }
            return;
        }

        let (left, right, stay) = self.classify();

        // if split is degenerate, restore children and just recurse
        if left.is_empty() || right.is_empty() {
            self.children = stay;
            self.children.extend(left);
            self.children.extend(right);
            for child in &mut self.children {
                child.divide(threshold);
            }
            return;
        }

        // commit split: keep 'stay', add two subgroups
        self.children = stay;
        self.make_sub_group(left);
        self.make_sub_group(right);

        for child in &mut self.children {
            child.divide(threshold);
        }
    }
}

impl LocalShape for Group {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        if !self.common.aabb.intersects(ray) {
            return;
        }
        for child in self.children.iter() {
            child.intersect(ray, buffer);
        }
    }

    fn local_normal(&self, _point: Vec3A, _intersection: Intersection) -> Vec3A {
        panic!("Groups don't have normals")
    }
}

impl From<Group> for Shape {
    fn from(g: Group) -> Self {
        Shape::Group(g)
    }
}
