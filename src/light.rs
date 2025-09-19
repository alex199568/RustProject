use crate::color::Color;
use crate::intersection::Hit;
use crate::material::Material;
use crate::scene::Scene;
use crate::shape::Shape;

use glam::Vec3A;

use rand::Rng;

pub struct Light {
    pub position: Vec3A,
    pub intensity: Color,
}

impl Light {
    pub fn shade(&self, shape: &Shape, material: &Material, hit: &Hit, scene: &Scene) -> Color {
        let material_color = match &material.pattern {
            Some(p) => {
                let mut parent_id = shape.parent_id();
                let mut point = shape.transform_point(hit.point);
                while parent_id.is_some() {
                    let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
                    point = parent_shape.transform_point(point);
                    parent_id = parent_shape.parent_id();
                }

                p.at(point)
            }
            None => material.color,
        };

        let effective_color = material_color * self.intensity;

        let light_v = (self.position - hit.point).normalize();
        let ambient = effective_color * material.ambient;
        let light_dot_normal = light_v.dot(hit.normal);
        if light_dot_normal < 0.0 {
            return ambient;
        }

        let diffuse = effective_color * (material.diffuse * light_dot_normal);
        let reflect = (-light_v).reflect(hit.normal);
        let reflect_dot_eye = reflect.dot(hit.eye);
        if reflect_dot_eye <= 0.0 {
            return ambient + diffuse;
        }

        let shininess = material.shininess;
        let factor = reflect_dot_eye.powf(shininess);
        let specular = self.intensity * (material.specular * factor);

        ambient + diffuse + specular
    }
}

pub struct AreaLight {
    corner: Vec3A,
    uvec: Vec3A,
    pub usteps: usize,
    vvec: Vec3A,
    pub vsteps: usize,
    intensity: Color,
    pub samples: usize,
}

impl AreaLight {
    pub fn new(
        corner: Vec3A,
        uvec: Vec3A,
        usteps: usize,
        vvec: Vec3A,
        vsteps: usize,
        intensity: Color,
    ) -> Self {
        Self {
            corner: corner,
            uvec: uvec,
            usteps: usteps,
            vvec: vvec,
            vsteps: vsteps,
            intensity: intensity,
            samples: usteps * vsteps,
        }
    }

    pub fn point_on(&self, u: f32, v: f32) -> Vec3A {
        let mut r = rand::rng();
        let u_offset: f32 = r.random();
        let v_offset: f32 = r.random();
        self.corner + self.uvec * (u + u_offset) + self.vvec * (v + v_offset)
    }

    pub fn shade(&self, shape: &Shape, material: &Material, hit: &Hit, scene: &Scene) -> Color {
        let material_color = match &material.pattern {
            Some(p) => {
                let mut parent_id = shape.parent_id();
                let mut point = shape.transform_point(hit.point);
                while parent_id.is_some() {
                    let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
                    point = parent_shape.transform_point(point);
                    parent_id = parent_shape.parent_id();
                }

                p.at(point)
            }
            None => material.color,
        };

        let mut total = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };

        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                let effective_color = material_color * self.intensity;

                let position = self.point_on(u as f32, v as f32);

                let light_v = (position - hit.point).normalize();
                let ambient = effective_color * material.ambient;
                let light_dot_normal = light_v.dot(hit.normal);
                if light_dot_normal < 0.0 {
                    total += ambient;
                    continue;
                }

                let diffuse = effective_color * (material.diffuse * light_dot_normal);
                let reflect = (-light_v).reflect(hit.normal);
                let reflect_dot_eye = reflect.dot(hit.eye);
                if reflect_dot_eye <= 0.0 {
                    total += ambient + diffuse;
                    continue;
                }

                let shininess = material.shininess;
                let factor = reflect_dot_eye.powf(shininess);
                let specular = self.intensity * (material.specular * factor);

                total += ambient + diffuse + specular
            }
        }

        total / self.samples as f32
    }
}
