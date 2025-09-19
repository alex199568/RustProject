use crate::Light;
use crate::color::Color;
use crate::intersection::Hit;
use crate::intersection::Intersection;
use crate::intersection::IntersectionBuffer;
use crate::light::AreaLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;

use glam::Vec3A;

pub struct Scene {
    materials: Vec<Material>,
    shapes: Vec<Shape>,
    lights: Vec<Light>,
    area_lights: Vec<AreaLight>,
}

impl Scene {
    pub fn new(
        materials: Vec<Material>,
        shapes: Vec<Shape>,
        lights: Vec<Light>,
        area_lights: Vec<AreaLight>,
    ) -> Self {
        Self {
            materials: materials,
            shapes: shapes,
            lights: lights,
            area_lights: area_lights,
        }
    }

    fn intersect(&self, ray: &Ray, intersections: &mut IntersectionBuffer) {
        intersections.clear();
        for shape in self.shapes.iter() {
            shape.intersect(ray, intersections);
        }
    }

    fn shadow(
        &self,
        light_position: Vec3A,
        shape_id: usize,
        point: Vec3A,
        buffer: &mut IntersectionBuffer,
    ) -> f32 {
        let v = light_position - point;
        let distance = v.length();
        let direction = v / distance;
        let r = Ray {
            origin: point,
            direction: direction,
        };
        self.intersect(&r, buffer);

        let mut transittance = 1.0f32; // transmittance (1=fully visible)
        for i in buffer
            .intersections
            .iter()
            .filter(|i| i.t > 1e-4 && i.t < distance && i.shape_id != shape_id)
        {
            let s = self.find_shape_by_id(i.shape_id);
            let m = self.find_material_by_id(s.material_id());
            let tau = m.transparency.clamp(0.0, 1.0); // per-surface transmittance
            transittance *= tau; // multiply through each layer
            // might break early if transittance is nearly 0, this way intersections need to be sorted by t
        }

        transittance
    }

    fn light_intensity(
        &self,
        shape_id: usize,
        light: &Light,
        point: Vec3A,
        buffer: &mut IntersectionBuffer,
    ) -> f32 {
        self.shadow(light.position, shape_id, point, buffer)
    }

    fn area_light_intensity(
        &self,
        shape_id: usize,
        light: &AreaLight,
        point: Vec3A,
        buffer: &mut IntersectionBuffer,
    ) -> f32 {
        let mut total = 0.0f32;

        for v in 0..light.vsteps {
            for u in 0..light.usteps {
                let position = light.point_on(u as f32, v as f32);
                let s = self.shadow(position, shape_id, point, buffer);
                total += s;
            }
        }

        total / light.samples as f32
    }

    fn shade(
        &self,
        hit: &Hit,
        shape: &Shape,
        material: &Material,
        buffer: &mut IntersectionBuffer,
    ) -> Color {
        let mut surface = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };

        for light in &self.lights {
            // let s = self.shadow(light.position, hit.shape_id, hit.over_point, buffer);
            let intensity = self.light_intensity(hit.shape_id, light, hit.over_point, buffer);
            let c = light.shade(shape, material, hit, self) * intensity;
            surface += c;
        }

        for light in &self.area_lights {
            let intensity = self.area_light_intensity(hit.shape_id, light, hit.over_point, buffer);
            let c = light.shade(shape, material, hit, self) * intensity;
            surface += c;
        }

        surface
    }

    pub fn find_refractions(&self, hit: Intersection, buffer: &IntersectionBuffer) -> (f32, f32) {
        let mut containers: Vec<usize> = Vec::new(); // holds shape indices we are currently "inside"
        let mut n1 = 1.0_f32;
        let mut n2 = 1.0_f32;

        for i in buffer.intersections.iter() {
            // Is this the intersection we're computing for?
            let is_hit = i.shape_id == hit.shape_id && (i.t - hit.t).abs() < 1e-6;

            // n1: IOR before toggling membership
            if is_hit {
                n1 = if containers.is_empty() {
                    1.0
                } else {
                    let s = containers.last().unwrap();
                    let shape = self.find_shape_by_id(*s);
                    let material = self.find_material_by_id(shape.material_id());
                    material.refraction
                };
            }

            // Toggle membership of this shape
            let sid = i.shape_id;
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
                    let shape = self.find_shape_by_id(*s);
                    let material = self.find_material_by_id(shape.material_id());
                    material.refraction
                };
                break;
            }
        }

        (n1, n2)
    }

    fn schlick_ior(cos_i: f32, n1: f32, n2: f32) -> f32 {
        let f0 = ((n1 - n2) / (n1 + n2)).powi(2);
        f0 + (1.0 - f0) * (1.0 - cos_i).powi(5)
    }

    pub fn find_shape_by_id(&self, id: usize) -> &Shape {
        self.shapes.iter().find_map(|s| s.find_by_id(id)).unwrap()
    }

    pub fn find_material_by_id(&self, id: usize) -> &Material {
        self.materials.iter().find(|m| m.id == id).unwrap()
    }

    pub fn color(&self, ray0: &Ray, buf: &mut IntersectionBuffer, max_depth: usize) -> Color {
        // Each item: (ray, throughput, depth)
        let mut stack: Vec<(Ray, f32, usize)> = Vec::with_capacity(max_depth * 2);
        stack.push((*ray0, 1.0, 0));

        let mut out = Color::BLACK;

        while let Some((ray, thr, depth)) = stack.pop() {
            if depth >= max_depth || thr < 1e-5 {
                continue;
            }

            self.intersect(&ray, buf); // must clear once per ray
            let Some(hit) = buf.hit() else {
                // environment (if any), e.g.: out += thr * env_color(ray);
                continue;
            };

            let shape = self.find_shape_by_id(hit.shape_id);
            let m = self.find_material_by_id(shape.material_id());

            let h = Hit::new(shape, hit, &ray, self);

            // --- local lighting (non-transmitted part) ---
            let local = self.shade(&h, shape, m, buf);

            let refl = m.reflection.clamp(0.0, 1.0); // scalar 0..1
            let transp = m.transparency.clamp(0.0, 1.0); // scalar 0..1

            // optional Fresnel splitting (better than flat refl):
            let (n1, n2) = self.find_refractions(hit, buf); // uses current buffer contents
            let cos_i = (-ray.direction).dot(h.normal).max(0.0);
            // let F = Self::schlick(cos_i, n1, n2); // reflection coefficient 0..1
            let f_phys = Self::schlick_ior(cos_i, n1, n2);
            let fresnel = 1.0 - (1.0 - m.reflection.clamp(0.0, 1.0)) * (1.0 - f_phys); // >= reflection

            // Accumulate local (opaque) component:
            // If the material is transparent, scale local down; if opaque, transp=0.
            out += local * (thr * (1.0 - transp) * (1.0 - refl));

            // --- spawn reflection ---
            let refl_w = thr * refl; // use F to modulate; drop F if you prefer flat reflectivity, * F
            if refl_w > 1e-5 {
                let dir = h.reflect;
                let r = Ray {
                    origin: h.over_point + dir * 1e-4,
                    direction: dir,
                };
                stack.push((r, refl_w, depth + 1));
            }

            // --- spawn refraction (skip on TIR) ---
            let refr_w = thr * transp * (1.0 - fresnel);
            if refr_w > 1e-5 {
                let n_ratio = n1 / n2;
                let cos_i = h.eye.dot(h.normal);
                let sin2t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
                if sin2t <= 1.0 {
                    let cos_t = (1.0 - sin2t).sqrt();
                    // refract dir = n*(n_ratio*cos_i - cos_t) - v*n_ratio, where v = view = -ray.dir
                    let refr_dir = h.normal * (n_ratio * cos_i - cos_t) - h.eye * n_ratio;
                    let r = Ray {
                        origin: h.under_point + refr_dir * 1e-4,
                        direction: refr_dir.normalize(),
                    };
                    stack.push((r, refr_w, depth + 1));
                } else {
                    // total internal reflection -> reflection already spawned above carries the energy
                }
            }
        }

        out
    }
}
