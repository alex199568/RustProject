use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::caps::Caps;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};
use std::convert::From;

use glam::Affine3A;
use glam::Vec3A;

pub struct Cone {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub material_id: usize,
    range: (f32, f32),
    caps: bool,
}

impl Cone {
    pub fn new(tr: &Affine3A, material: usize, range: (f32, f32), caps: bool) -> Self {
        let a = range.0.abs();
        let b = range.1.abs();
        let limit = a.max(b);
        let bounds = Aabb::new(
            glam::vec3a(-limit, range.0, -limit),
            glam::vec3a(limit, range.1, limit),
        );
        Self {
            common: ShapeCommon::new(bounds),
            transform: ShapeTransform::new(tr),
            material_id: material,
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
                        t: t,
                        uv: None,
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
                    uv: None,
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
                    uv: None,
                });
            }
        }
    }

    fn local_normal(&self, p: Vec3A, _intersection: Intersection) -> Vec3A {
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

impl From<Cone> for Shape {
    fn from(c: Cone) -> Self {
        Shape::Cone(c)
    }
}
