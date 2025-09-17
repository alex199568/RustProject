
use crate::shape::Shape;
use crate::Light;
use crate::intersection::IntersectionBuffer;
use crate::intersection::Hit;
use crate::point::Point;
use crate::ray::Ray;
use crate::color::Color;

pub struct Scene {
    shapes: Vec<Shape>,
    lights: Vec<Light>,
}

impl Scene {

    pub fn new(shapes: Vec<Shape>, lights: Vec<Light>) -> Self {
        Self {
            shapes: shapes,
            lights: lights,
        }
    }

    fn intersect(&self, ray: &Ray, intersections: &mut IntersectionBuffer) {
        intersections.clear();
        for (i, shape) in self.shapes.iter().enumerate() {
            shape.intersect(ray, intersections, i);
        }
    }

    fn shadow_intersect(&self, ray: &Ray, buffer: &mut IntersectionBuffer, ignore: usize) {
        buffer.clear();
        for (i, shape) in self.shapes.iter().enumerate() {
            if i != ignore {
                shape.intersect(ray, buffer, i);
            }
        }
    }

    fn shadow(&self, light: &Light, shape_index: usize, point: Point, buffer: &mut IntersectionBuffer) -> f32 {
        let v = light.position - point;
        let distance = v.length();
        let direction = v / distance;
        let r = Ray { origin: point, direction: direction };
        self.shadow_intersect(&r, buffer, shape_index);
        if let Some(hit) = buffer.hit() {
            return if hit.t < distance {
                1.0
            } else {
                0.0
            }
        }
        0.0
    }

    fn shade(&self, hit: &Hit, shape: &Shape, buffer: &mut IntersectionBuffer) -> Color {
        let mut result = Color{r: 0.0, g: 0.0, b: 0.0};

        for light in &self.lights {
            let s = self.shadow(light, hit.shape_index, hit.point, buffer);
            let c = light.shade(shape, hit, s);
            result += c;
        } 

        result
    }

    pub fn color(&self, ray: &Ray, buffer: &mut IntersectionBuffer) -> Color {
        self.intersect(ray, buffer);
        if let Some(hit) = buffer.hit() {
            let shape = &self.shapes[hit.shape_index];
            let h = Hit::new(shape, hit, ray);
            return self.shade(&h, shape, buffer);
        }

        Color::BLACK
    }
}
