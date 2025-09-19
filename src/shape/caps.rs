use crate::intersection::{Intersection, IntersectionBuffer};
use crate::ray::Ray;

pub trait Caps {
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
