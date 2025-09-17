
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
}

impl Scene {

    pub fn new(shapes: Vec<Sphere>, lights: Vec<Light>) -> Self {
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

    fn shade(&self, hit: &Hit, material: &Material) -> Color {
        let mut result = Color{r: 0.0, g: 0.0, b: 0.0};

        for light in &self.lights {
            let c = light.shade(material, hit);
            result += c;
        } 

        result
    }

    pub fn color(&self, ray: &Ray, buffer: &mut IntersectionBuffer) -> Color {
        self.intersect(ray, buffer);
        if let Some(hit) = buffer.hit() {
            let shape = &self.shapes[hit.shape_index];
            let h = Hit::new(shape, hit, ray);
            return self.shade(&h, &shape.material);
        }

        Color::BLACK
    }
}
