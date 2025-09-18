use crate::intersection::{Intersection, IntersectionBuffer};
use crate::material::Material;
use crate::ray::Ray;

use glam::Affine3A;
use glam::Mat3A;
use glam::Vec3A;

use std::convert::From;

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_SHAPE_ID: AtomicUsize = AtomicUsize::new(1);

struct ShapeCommon {
    inv: Affine3A,
    inv_tr: Mat3A,
    id: usize
}

trait LocalShape {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer);
    fn local_normal(&self, point: Vec3A) -> Vec3A;
}

impl ShapeCommon {

    fn new(transform: &Affine3A) -> Self {
        let inv = transform.inverse();
        let inv_tr = inv.matrix3.transpose();
        let id = NEXT_SHAPE_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            inv: inv,
            inv_tr: inv_tr,
            id: id
        }
    }

    fn intersect(
        &self,
        local: &dyn LocalShape,
        ray: &Ray,
        buffer: &mut IntersectionBuffer,
    ) {
        let transformed_ray = &self.inv * ray;
        local.local_intersect(&transformed_ray, buffer);
    }

    fn normal(&self, local: &dyn LocalShape, point: Vec3A) -> Vec3A {
        let shape_point = &self.inv.transform_point3a(point);
        let shape_normal = local.local_normal(*shape_point);
        let world_normal = &self.inv_tr * shape_normal;
        world_normal.normalize()
    }
}

pub struct Sphere {
    common: ShapeCommon,
    material: Material,
}

impl Sphere {
    pub fn new(transform: &Affine3A, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material,
        }
    }
}

impl LocalShape for Sphere {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let sphere_to_ray = ray.origin;
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

        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t0,
        });
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t1,
        });
    }

    fn local_normal(&self, point: Vec3A) -> Vec3A {
        point
    }
}

pub struct Plane {
    common: ShapeCommon,
    material: Material,
}

impl Plane {
    pub fn new(transform: &Affine3A, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material,
        }
    }
}

impl LocalShape for Plane {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        if ray.direction.y.abs() < 1e-5 {
            return;
        }
        let t = -ray.origin.y / ray.direction.y;
        buffer.add(Intersection {
            shape_id: self.common.id,
            t: t,
        });
    }

    fn local_normal(&self, _point: Vec3A) -> Vec3A {
        Vec3A::Y
    }
}

pub struct Cube {
    common: ShapeCommon,
    material: Material,
}

impl Cube {
    pub fn new(transform: &Affine3A, material: Material) -> Self {
        Self {
            common: ShapeCommon::new(transform),
            material: material,
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
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let (xtmin, xtmax) = Self::check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (ytmin, ytmax) = Self::check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (ztmin, ztmax) = Self::check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);
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

pub struct Cylinder {
    common: ShapeCommon,
    material: Material,
    range: (f32, f32),
    caps: bool,
}

trait Caps {
    fn caps(&self) -> bool;
    fn from(&self) -> f32;
    fn to(&self) -> f32;

    #[inline]
    fn cap_radius2_at(&self, _y: f32) -> f32 {
        1.0
    }

    fn intersect_caps(&self, ray: &Ray, buffer: &mut IntersectionBuffer, id: usize) {
        if !self.caps() || ray.direction.y.abs() < 1e-6 {
            return; // no caps or parallel to cap planes
        }

        let inv_dy = 1.0 / ray.direction.y;

        // bottom cap at y = from()
        let y0 = self.from();
        let t0 = (y0 - ray.origin.y) * inv_dy;
        if t0 >= 0.0 {
            let x = ray.origin.x + t0 * ray.direction.x;
            let z = ray.origin.z + t0 * ray.direction.z;
            if x * x + z * z <= self.cap_radius2_at(y0) + 1e-6 {
                buffer.add(Intersection {
                    shape_id: id,
                    t: t0,
                });
            }
        }

        // top cap at y = to()
        let y1 = self.to();
        let t1 = (y1 - ray.origin.y) * inv_dy;
        if t1 >= 0.0 {
            let x = ray.origin.x + t1 * ray.direction.x;
            let z = ray.origin.z + t1 * ray.direction.z;
            if x * x + z * z <= self.cap_radius2_at(y1) + 1e-6 {
                buffer.add(Intersection {
                    shape_id: id,
                    t: t1,
                });
            }
        }
    }
}

impl Cylinder {
    pub fn new(tr: &Affine3A, material: Material, range: (f32, f32), caps: bool) -> Self {
        Self {
            common: ShapeCommon::new(tr),
            material: material,
            range: range,
            caps: caps,
        }
    }
}

impl Caps for Cylinder {
    #[inline]
    fn caps(&self) -> bool {
        self.caps
    }

    #[inline]
    fn from(&self) -> f32 {
        self.range.0
    }

    #[inline]
    fn to(&self) -> f32 {
        self.range.1
    }
}

impl LocalShape for Cylinder {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        self.intersect_caps(ray, buffer, self.common.id);
        let o = ray.origin;
        let d = ray.direction;
        let a = d.x * d.x + d.z * d.z;
        if a.abs() < 1e-6 {
            return;
        }
        let b = 2.0 * o.x * d.x + 2.0 * o.z * d.z;
        let c = o.x * o.x + o.z * o.z - 1.0;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return;
        }
        let sd = disc.sqrt();
        let t0 = (-b - sd) / (2.0 * a);
        let mut y0 = o.y + t0 * d.y;
        if self.range.0 < y0 && y0 < self.range.1 {
            buffer.add(Intersection {
                shape_id: self.common.id,
                t: t0,
            });
        }
        let t1 = (-b + sd) / (2.0 * a);
        y0 = o.y + t1 * d.y;
        if self.range.0 < y0 && y0 < self.range.1 {
            buffer.add(Intersection {
                shape_id: self.common.id,
                t: t1,
            });
        }
    }

    fn local_normal(&self, point: Vec3A) -> Vec3A {
        let distance = point.x * point.x + point.z * point.z;
        if distance < 1.0 && point.y >= self.range.1 - 1e-5 {
            return glam::vec3a(0.0, 1.0, 0.0);
        }
        if distance < 1.0 && point.y <= self.range.0 + 1e-5 {
            return glam::vec3a(0.0, -1.0, 0.0);
        }
        glam::vec3a(point.x, 0.0, point.z)
    }
}

pub struct Cone {
    common: ShapeCommon,
    material: Material,
    range: (f32, f32),
    caps: bool,
}

impl Cone {
    pub fn new(tr: &Affine3A, material: Material, range: (f32, f32), caps: bool) -> Self {
        Self {
            common: ShapeCommon::new(tr),
            material: material,
            range: range,
            caps: caps,
        }
    }
}

impl Caps for Cone {
    #[inline]
    fn caps(&self) -> bool {
        self.caps
    }

    #[inline]
    fn from(&self) -> f32 {
        self.range.0
    }

    #[inline]
    fn to(&self) -> f32 {
        self.range.1
    }

    #[inline]
    fn cap_radius2_at(&self, _y: f32) -> f32 {
        (_y * _y).abs()
    }
}

impl LocalShape for Cone {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        // caps first (fine either order)
        self.intersect_caps(ray, buffer, self.common.id);

        let o = ray.origin;
        let d = ray.direction;

        // Cone (double cone, 45°): x^2 + z^2 - y^2 = 0
        let a = d.x * d.x + d.z * d.z - d.y * d.y;
        let b = 2.0 * (o.x * d.x + o.z * d.z - o.y * d.y);
        let c = o.x * o.x + o.z * o.z - o.y * o.y;

        const EPS: f32 = 1e-6;

        // Linear case: a ≈ 0 -> bt + c = 0
        if a.abs() < EPS {
            if b.abs() < EPS {
                // No intersection (ray parallel to the cone asymptote and offset)
                return;
            }
            let t = -c / b;
            if t >= 0.0 {
                let y = o.y + t * d.y;
                if self.range.0 < y && y < self.range.1 {
                    buffer.add(Intersection {
                        shape_id: self.common.id,
                        t,
                    });
                }
            }
            return;
        }

        // Quadratic
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return;
        }

        let sd = disc.sqrt();
        let inv2a = 0.5 / a;

        // Ensure t0 <= t1 for stable y checks (optional but nice)
        let (t0, t1) = {
            let t0 = (-b - sd) * inv2a;
            let t1 = (-b + sd) * inv2a;
            if t0 <= t1 { (t0, t1) } else { (t1, t0) }
        };

        // First root
        if t0 >= 0.0 {
            let y0 = o.y + t0 * d.y;
            if self.range.0 < y0 && y0 < self.range.1 {
                buffer.add(Intersection {
                    shape_id: self.common.id,
                    t: t0,
                });
            }
        }

        // Second root
        if t1 >= 0.0 {
            let y1 = o.y + t1 * d.y;
            if self.range.0 < y1 && y1 < self.range.1 {
                buffer.add(Intersection {
                    shape_id: self.common.id,
                    t: t1,
                });
            }
        }
    }

    fn local_normal(&self, p: Vec3A) -> Vec3A {
        // For a 45° double cone x^2 + z^2 - y^2 = 0:
        // Side normal: (x, ±sqrt(x^2+z^2), z) with sign depending on y
        // Cap normals handled first if you have finite caps with radius |y|.
        let distance = p.x * p.x + p.z * p.z;

        // If you have finite caps, their radii are |y| at the cap’s y.
        // So a proper cap test is: distance < (|y_cap|)^2.
        // If your cone’s top/bottom y are (y_min, y_max),
        // then radius at y_min is |y_min|, at y_max is |y_max|.
        // Example with small tolerance:
        const EPS: f32 = 1e-6;
        let r_top2 = self.range.1.abs().powi(2);
        let r_bot2 = self.range.0.abs().powi(2);

        if self.caps && (p.y >= self.range.1 - EPS) && (distance <= r_top2 + 1e-6) {
            return glam::vec3a(0.0, 1.0, 0.0);
        }
        if self.caps && (p.y <= self.range.0 + EPS) && (distance <= r_bot2 + 1e-6) {
            return glam::vec3a(0.0, -1.0, 0.0);
        }

        // Side normal
        let mut y = (distance).sqrt();
        if p.y > 0.0 {
            y = -y;
        } // matches RTC convention
        glam::vec3a(p.x, y, p.z).normalize()
    }
}

pub struct Group {
    common: ShapeCommon,
    children: Vec<Shape>
}

impl Group {

    pub fn new(tr: &Affine3A) -> Self {
        Self {
            common: ShapeCommon::new(tr),
            children: vec![]
        }
    }

    pub fn add(&mut self, shape: Shape) {
        self.children.push(shape);
    }
}

impl LocalShape for Group {

    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        for child in self.children.iter() {
            child.intersect(ray, buffer);
        }
    }

    fn local_normal(&self, point: Vec3A) -> Vec3A {
        Vec3A::ZERO
    }
}

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
    Group(Group)
}

impl Shape {

    fn common(&self) -> &ShapeCommon {
        match self {
            Shape::Sphere(s) => &s.common,
            Shape::Plane(p) => &p.common,
            Shape::Cube(c) => &c.common,
            Shape::Cylinder(c) => &c.common,
            Shape::Cone(c) => &c.common,
            Shape::Group(g) => &g.common
        }
    }

    pub fn intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        match self {
            Shape::Sphere(s) => s.common.intersect(s, ray, buffer),
            Shape::Plane(p) => p.common.intersect(p, ray, buffer),
            Shape::Cube(c) => c.common.intersect(c, ray, buffer),
            Shape::Cylinder(c) => c.common.intersect(c, ray, buffer),
            Shape::Cone(c) => c.common.intersect(c, ray, buffer),
            Shape::Group(g) => g.common.intersect(g, ray, buffer)
        }
    }

    pub fn normal(&self, point: Vec3A) -> Vec3A {
        match self {
            Shape::Sphere(s) => s.common.normal(s, point),
            Shape::Plane(p) => p.common.normal(p, point),
            Shape::Cube(c) => c.common.normal(c, point),
            Shape::Cylinder(c) => c.common.normal(c, point),
            Shape::Cone(c) => c.common.normal(c, point),
            Shape::Group(g) => g.common.normal(g, point)
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => &s.material,
            Shape::Plane(p) => &p.material,
            Shape::Cube(c) => &c.material,
            Shape::Cylinder(c) => &c.material,
            Shape::Cone(c) => &c.material,
            Shape::Group(g) => g.children[0].material()
        }
    }

    pub fn inv(&self) -> &Affine3A {
        &self.common().inv
    }

    pub fn max_intersections(&self) -> usize {
        match self {
            Shape::Sphere(_) => 2,
            Shape::Plane(_) => 1,
            Shape::Cube(_) => 2,
            Shape::Cylinder(_) => 2,
            Shape::Cone(_) => 4,
            Shape::Group(g) => g.children.iter().map(|c| c.max_intersections()).sum()
        }
    }

    pub fn check_id(&self, id: usize) -> bool {
        if self.common().id == id {
            return true;
        }
        if let Shape::Group(g) = self {
            return g.children.iter().any(|c| c.check_id(id));
        }
        return false;
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

impl From<Cylinder> for Shape {
    fn from(c: Cylinder) -> Self {
        Shape::Cylinder(c)
    }
}

impl From<Cone> for Shape {
    fn from(c: Cone) -> Self {
        Shape::Cone(c)
    }
}

impl From<Group> for Shape {

    fn from(g: Group) -> Self {
        Shape::Group(g)
    }
}
