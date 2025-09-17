
use crate::shape::Shape;
use crate::Light;
use crate::intersection::Intersection;
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
        let mut surface = Color{r: 0.0, g: 0.0, b: 0.0};

        for light in &self.lights {
            let s = self.shadow(light, hit.shape_index, hit.point, buffer);
            let c = light.shade(shape, hit, s);
            surface += c;
        }

        surface
    }

    pub fn find_refractions(
        &self,
        hit: Intersection,
        buffer: &IntersectionBuffer,
    ) -> (f32, f32) {
        let mut containers: Vec<usize> = Vec::new(); // holds shape indices we are currently "inside"
        let mut n1 = 1.0_f32;
        let mut n2 = 1.0_f32;

        for i in buffer.intersections.iter() {
            // Is this the intersection we're computing for?
            let is_hit =
                i.shape_index == hit.shape_index && (i.t - hit.t).abs() < 1e-6;

            // n1: IOR before toggling membership
            if is_hit {
                n1 = if containers.is_empty() {
                    1.0
                } else {
                    let s = containers.last().unwrap();
                    self.shapes[*s].material().refraction // or .refractive_index / ior
                };
            }

            // Toggle membership of this shape
            let sid = i.shape_index;
            if let Some(pos) = containers.iter().position(|&x| x == sid) {
                containers.remove(pos);
            } else {
                containers.push(sid);
            }

            // n2: IOR after toggling membership, then we're done
            if is_hit {
                n2 = if containers.is_empty() {
                    1.0
                } else {
                    let s = containers.last().unwrap();
                    self.shapes[*s].material().refraction
                };
                break;
            }
        }

        (n1, n2)
    }

    #[inline]
    fn schlick(cos_i: f32, n1: f32, n2: f32) -> f32 {
        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_i).max(0.0).powi(5)
    }
    
    pub fn color(&self, ray0: &Ray, buf: &mut IntersectionBuffer, max_depth: usize) -> Color {
        // Each item: (ray, throughput, depth)
        let mut stack: Vec<(Ray, f32, usize)> = Vec::with_capacity(max_depth * 2);
        stack.push((*ray0, 1.0, 0));

        let mut out = Color::BLACK;

        while let Some((ray, thr, depth)) = stack.pop() {
            if depth >= max_depth || thr < 1e-5 { continue; }

            self.intersect(&ray, buf);                 // must clear once per ray
            let Some(hit) = buf.hit() else {
                // environment (if any), e.g.: out += thr * env_color(ray);
                continue;
            };

            let shape = &self.shapes[hit.shape_index];
            let h = Hit::new(shape, hit, &ray);

            // --- local lighting (non-transmitted part) ---
            let local = self.shade(&h, shape, buf);

            let m = shape.material();
            let refl = m.reflection.clamp(0.0, 1.0);       // scalar 0..1
            let transp = m.transparency.clamp(0.0, 1.0);   // scalar 0..1

            // optional Fresnel splitting (better than flat refl):
            let (n1, n2) = self.find_refractions(hit, buf);    // uses current buffer contents
            let cos_i = (-ray.direction).dot(h.normal).max(0.0);
            let F = Self::schlick(cos_i, n1, n2); // reflection coefficient 0..1

            // Accumulate local (opaque) component:
            // If the material is transparent, scale local down; if opaque, transp=0.
            out += local * (thr * (1.0 - transp) * (1.0 - refl));

            // --- spawn reflection ---
            let refl_w = thr * refl; // use F to modulate; drop F if you prefer flat reflectivity, * F
            if refl_w > 1e-5 {
                let dir = h.reflect;
                let r = Ray { origin: h.over_point, direction: dir };
                stack.push((r, refl_w, depth + 1));
            }

            // --- spawn refraction (skip on TIR) ---
            let refr_w = thr * transp * (1.0 - F);
            if refr_w > 1e-5 {
                let n_ratio = n1 / n2;
                let sin2t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
                if sin2t <= 1.0 {
                    let cos_t = (1.0 - sin2t).sqrt();
                    // refract dir = n*(n_ratio*cos_i - cos_t) - v*n_ratio, where v = view = -ray.dir
                    let refr_dir = h.normal * (n_ratio * cos_i - cos_t) - (-ray.direction) * n_ratio;
                    let r = Ray { origin: h.under_point, direction: refr_dir.unit() };
                    stack.push((r, refr_w, depth + 1));
                } else {
                    // total internal reflection -> reflection already spawned above carries the energy
                }
            }
        }

        out
    }
}
