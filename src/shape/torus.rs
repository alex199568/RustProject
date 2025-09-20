use crate::aabb::Aabb;
use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;
use crate::shape::shape::{LocalShape, Shape, ShapeCommon, ShapeTransform};

use std::convert::From;

use roots::{Roots, find_roots_quartic};

use glam::Affine3A;
use glam::Vec3A;

const EPS_T: f32 = 1e-5; // t threshold
const DEDUP_EPS: f32 = 1e-5; // root merging tol
const RES_TOL: f64 = 1e-6; // polynomial residual tol (scaled in f64)

// TODO: clean up and refactor

pub struct Torus {
    pub common: ShapeCommon,
    pub transform: ShapeTransform,
    pub material_id: usize,
    r: f32,
    thickness: f32,
}

impl Torus {
    pub fn new(transform: &Affine3A, material_id: usize, r: f32, thickness: f32) -> Self {
        let outter_r = r + thickness;
        let bounds = Aabb::new(
            glam::vec3a(-outter_r, -thickness, -outter_r),
            glam::vec3a(outter_r, thickness, outter_r),
        );
        Self {
            common: ShapeCommon::new(bounds),
            transform: ShapeTransform::new(transform),
            material_id,
            r,
            thickness,
        }
    }

    /// Robust quartic solve + filtering.
    /// Returns *sorted*, unique, positive roots that satisfy a residual test.
    fn solve_torus_quartic(a4: f32, a3: f32, a2: f32, a1: f32, a0: f32) -> Vec<f32> {
        // Upcast and solve: a4 t^4 + a3 t^3 + a2 t^2 + a1 t + a0 = 0
        let res = find_roots_quartic(a4 as f64, a3 as f64, a2 as f64, a1 as f64, a0 as f64);

        // Residual checker in f64
        let poly = |t: f64| {
            (((a4 as f64 * t + a3 as f64) * t + a2 as f64) * t + a1 as f64) * t + a0 as f64
        };

        // Collect -> filter -> sort -> dedup
        let mut out: Vec<f32> = match res {
            Roots::No(_) => Vec::new(),
            Roots::One(r) => r.iter().copied().map(|x| x as f32).collect(),
            Roots::Two(r) => r.iter().copied().map(|x| x as f32).collect(),
            Roots::Three(r) => r.iter().copied().map(|x| x as f32).collect(),
            Roots::Four(r) => r.iter().copied().map(|x| x as f32).collect(),
        };

        // Keep only finite, positive and with small residual
        out.retain(|&t| {
            if !t.is_finite() || t <= EPS_T {
                return false;
            }
            let r = poly(t as f64).abs();
            r < RES_TOL
        });

        out.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out.dedup_by(|a, b| (*a - *b).abs() < DEDUP_EPS);
        out
    }
}

/* -------------- LocalShape -------------- */

impl LocalShape for Torus {
    fn local_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer) {
        let o = ray.origin;
        let d = ray.direction;

        let r = self.r; // major radius
        let r0 = self.thickness; // tube radius

        // (Optionally normalize d; not required, but consistent coefficients help.)
        // let d = d.normalize();

        // Precompute dots
        let d2 = d.dot(d);
        let od = o.dot(d);
        let o2 = o.dot(o);

        // q(t) = |o + t d|^2 + R^2 - r0^2  = a t^2 + b t + c
        let a = d2;
        let b = 2.0 * od;
        let c = o2 + r * r - r0 * r0;

        let dxz2 = d.x * d.x + d.z * d.z;
        let oxz = o.x * d.x + o.z * d.z;
        let oxz2 = o.x * o.x + o.z * o.z;
        let r2 = r * r;

        // (q(t))^2 - 4 R^2 ((ox + t dx)^2 + (oz + t dz)^2) = 0
        let a4 = a * a;
        let a3 = 2.0 * a * b;
        let a2 = b * b + 2.0 * a * c - 4.0 * r2 * dxz2;
        let a1 = 2.0 * b * c - 8.0 * r2 * oxz;
        let a0 = c * c - 4.0 * r2 * oxz2;

        let roots = Self::solve_torus_quartic(a4, a3, a2, a1, a0);

        // ***** Conservative strategy: keep ONLY the nearest hit *****
        if let Some(&t) = roots.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
            buffer.add(Intersection {
                shape_id: self.common.id,
                t,
                uv: None,
            });
        }

        // If you *need* all intersections (e.g., for CSG),
        // replace the block above with:
        // for &t in &roots {
        //     buffer.add(Intersection { shape_id: self.common.id, t, uv: None });
        // }
    }

    fn local_normal(&self, p: Vec3A, _intersection: Intersection) -> Vec3A {
        // Gradient of F:
        // s = x^2 + y^2 + z^2 + R^2 - r0^2
        // ∂F/∂x = 4 x (s - 2R^2)
        // ∂F/∂y = 4 y s
        // ∂F/∂z = 4 z (s - 2R^2)
        let r = self.r;
        let r0 = self.thickness;

        let s = p.dot(p) + r * r - r0 * r0;
        let gx = 4.0 * p.x * (s - 2.0 * r * r);
        let gy = 4.0 * p.y * s;
        let gz = 4.0 * p.z * (s - 2.0 * r * r);

        let n = glam::vec3a(gx, gy, gz);
        let lsq = n.length_squared();
        if lsq > 1e-12 {
            n * lsq.sqrt()
        } else {
            glam::vec3a(0.0, 1.0, 0.0)
        }
    }
}

impl From<Torus> for Shape {
    fn from(t: Torus) -> Self {
        Shape::Torus(t)
    }
}
