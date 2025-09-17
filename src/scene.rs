
use crate::shape::Sphere;
use crate::Light;
use crate::intersection::Intersection;
use crate::intersection::IntersectionBuffer;
use crate::intersection::Hit;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::Material;

pub struct Scene {
    shapes: Vec<Sphere>,
    lights: Vec<Light>,
    intersections: IntersectionBuffer
}

impl Scene {

    pub fn new(shapes: Vec<Sphere>, lights: Vec<Light>) -> Self {
        let capacity = shapes.len() * 2;
        Self {
            shapes: shapes,
            lights: lights,
            intersections: IntersectionBuffer::new(capacity)
        }
    }

    fn intersect(&mut self, ray: &Ray) {
        self.intersections.clear();
        for (i, shape) in self.shapes.iter().enumerate() {
            shape.intersect(ray, &mut self.intersections, i);
        }
    }

    fn shade(&self, hit: &Hit, material: &Material) -> Color {
        let mut result = Color{r: 0.0, g: 0.0, b: 0.0};

        for light in &self.lights {
            let c = light.shade(material, hit);
            result += c;
        } 

        result
    }

    pub fn color(&mut self, ray: &Ray) -> Color {
        self.intersect(ray);
        if let Some(hit) = self.intersections.hit() {
            let shape = &self.shapes[hit.shape_index];
            let h = Hit::new(shape, hit, ray);
            return self.shade(&h, &shape.material);
        }

        Color::BLACK
    }
}
