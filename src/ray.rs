
use std::ops::Mul;

use glam::Vec3A;
use glam::Affine3A;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A
}

impl Ray {

    #[inline]
    pub fn at(self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}

impl Mul<&Ray> for &Affine3A {

    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Ray {
        Ray {
            origin: self.transform_point3a(rhs.origin),
            direction: self.transform_vector3a(rhs.direction)
        }
    }
}
