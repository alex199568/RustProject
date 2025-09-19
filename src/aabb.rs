use glam::Affine3A;
use glam::Vec3A;

use crate::ray::Ray;

use std::ops::AddAssign;

pub struct Aabb {
    from: Vec3A,
    to: Vec3A,
}

impl Aabb {
    pub fn default() -> Self {
        Self {
            from: Vec3A::MAX,
            to: Vec3A::MIN,
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            from: Vec3A::MAX,
            to: Vec3A::MIN,
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.from.x > self.to.x || self.from.y > self.to.y || self.from.z > self.to.z
    }

    pub fn new(from: Vec3A, to: Vec3A) -> Self {
        Self { from: from, to: to }
    }

    fn add_point(&mut self, rhs: Vec3A) {
        self.from.x = self.from.x.min(rhs.x);
        self.from.y = self.from.y.min(rhs.y);
        self.from.z = self.from.z.min(rhs.z);

        self.to.x = self.to.x.max(rhs.x);
        self.to.y = self.to.y.max(rhs.y);
        self.to.z = self.to.z.max(rhs.z);
    }

    fn add_aabb(&mut self, rhs: &Aabb) {
        self.add_point(rhs.from);
        self.add_point(rhs.to);
    }

    fn contains_point(&self, p: Vec3A) -> bool {
        self.from.x < p.x
            && p.x <= self.to.x
            && self.from.y < p.y
            && p.y <= self.to.y
            && self.from.z < p.z
            && p.z <= self.to.z
    }

    pub fn contains_aabb(&self, other: &Aabb) -> bool {
        self.contains_point(other.from) && self.contains_point(other.to)
    }

    fn x(&self) -> f32 {
        (self.to.x - self.from.x).abs()
    }

    fn y(&self) -> f32 {
        (self.to.y - self.from.y).abs()
    }

    fn z(&self) -> f32 {
        (self.to.z - self.from.z).abs()
    }

    pub fn split(&self) -> (Aabb, Aabb) {
        let dx = self.x();
        let dy = self.y();
        let dz = self.z();

        let greatest = dx.max(dy).max(dz);
        let mut x0 = self.from.x;
        let mut y0 = self.from.y;
        let mut z0 = self.from.z;
        let mut x1 = self.to.x;
        let mut y1 = self.to.y;
        let mut z1 = self.to.z;

        if (greatest - dx).abs() < 1e-6 {
            let newx = x0 + dx / 2.0;
            x0 = newx;
            x1 = newx;
        } else if (greatest - dy) < 1e-6 {
            let newy = y0 + dy / 2.0;
            y0 = newy;
            y1 = newy;
        } else {
            let newz = z0 + dz / 2.0;
            z0 = newz;
            z1 = newz;
        }

        let mid_min = glam::vec3a(x0, y0, z0);
        let mid_max = glam::vec3a(x1, y1, z1);

        let left = Self::new(self.from, mid_max);
        let right = Self::new(mid_min, self.to);

        (left, right)
    }

    pub fn check_axis(o: f32, d: f32, from: f32, to: f32) -> (f32, f32) {
        let t1 = (from - o) / d;
        let t2 = (to - o) / d;
        (t1.min(t2), t1.max(t2))
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) =
            Self::check_axis(ray.origin.x, ray.direction.x, self.from.x, self.to.x);
        let (ytmin, ytmax) =
            Self::check_axis(ray.origin.y, ray.direction.y, self.from.y, self.to.y);
        let (ztmin, ztmax) =
            Self::check_axis(ray.origin.z, ray.direction.z, self.from.z, self.to.z);

        let tmin = xtmin.max(ytmin).max(ztmin); // enter at the latest min
        let tmax = xtmax.min(ytmax).min(ztmax); // exit at the earliest max
        tmax >= tmin && tmax >= 0.0
    }
}

impl AddAssign<Vec3A> for Aabb {
    fn add_assign(&mut self, rhs: Vec3A) {
        self.add_point(rhs);
    }
}

impl AddAssign<&Aabb> for Aabb {
    fn add_assign(&mut self, rhs: &Aabb) {
        self.add_aabb(rhs);
    }
}

impl std::ops::Mul<&Affine3A> for &Aabb {
    type Output = Aabb;
    fn mul(self, m: &Affine3A) -> Aabb {
        let c = (self.from + self.to) * 0.5;
        let e = (self.to - self.from) * 0.5;

        let mc = m.transform_point3a(c);
        let a = m.matrix3.abs();

        let me = Vec3A::new(a.x_axis.dot(e), a.y_axis.dot(e), a.z_axis.dot(e));
        Aabb::new(mc - me, mc + me)
    }
}
