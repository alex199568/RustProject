

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub shape_index: usize,
    pub t: f32
}

pub struct IntersectionBuffer {
    intersections: Vec<Intersection>
}

impl IntersectionBuffer {

    pub fn add(&mut self, i: Intersection) {
        self.intersections.push(i);
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.intersections
            .iter()
            .copied()
            .filter(|i| i.t > 1e-5)
            .min_by(|a, b| a.t.total_cmp(&b.t))
    }
}
