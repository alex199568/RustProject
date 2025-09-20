use glam::{Vec2, Vec3A};

pub enum CubeFace {
    Left,
    Right,
    Back,
    Front,
    Bottom,
    Top,
}

impl CubeFace {
    #[inline]
    fn front(p: Vec3A) -> Vec2 {
        // +Z
        let u = (p.x + 1.0) * 0.5;
        let v = (p.y + 1.0) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    #[inline]
    fn back(p: Vec3A) -> Vec2 {
        // -Z
        let u = (1.0 - p.x) * 0.5;
        let v = (p.y + 1.0) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    #[inline]
    fn left(p: Vec3A) -> Vec2 {
        // -X
        let u = (p.z + 1.0) * 0.5;
        let v = (p.y + 1.0) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    #[inline]
    fn right(p: Vec3A) -> Vec2 {
        // +X
        let u = (1.0 - p.z) * 0.5;
        let v = (p.y + 1.0) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    #[inline]
    fn bottom(p: Vec3A) -> Vec2 {
        // -Y
        let u = (p.x + 1.0) * 0.5;
        let v = (p.z + 1.0) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    #[inline]
    fn top(p: Vec3A) -> Vec2 {
        // +Y
        let u = (p.x + 1.0) * 0.5;
        let v = (1.0 - p.z) * 0.5;
        Vec2::new(u.rem_euclid(1.0), v.rem_euclid(1.0))
    }

    fn uv_at(self, point: Vec3A) -> Vec2 {
        match self {
            CubeFace::Left => Self::left(point),
            CubeFace::Right => Self::right(point),
            CubeFace::Back => Self::back(point),
            CubeFace::Front => Self::front(point),
            CubeFace::Bottom => Self::bottom(point),
            CubeFace::Top => Self::top(point),
        }
    }

    pub fn from_point(point: Vec3A) -> CubeFace {
        let x = point.x.abs();
        let y = point.y.abs();
        let z = point.z.abs();
        let c = x.max(y).max(z);

        if c == point.x {
            CubeFace::Right
        } else if c == -(point.x) {
            CubeFace::Left
        } else if c == point.y {
            CubeFace::Top
        } else if c == -(point.y) {
            CubeFace::Bottom
        } else if c == point.z {
            CubeFace::Front
        } else {
            CubeFace::Back
        }
    }

    pub fn map_point(point: Vec3A) -> Vec2 {
        CubeFace::from_point(point).uv_at(point)
    }
}
