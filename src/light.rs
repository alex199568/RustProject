use crate::color::Color;
use crate::intersection::Hit;
use crate::material::Material;
use crate::scene::Scene;
use crate::shape::Shape;

use glam::Vec3A;

pub struct Light {
    pub position: Vec3A,
    pub intensity: Color,
}

impl Light {
    pub fn shade(&self, shape: &Shape, material: &Material, hit: &Hit, scene: &Scene) -> Color {
        let material_color = match &material.pattern {
            Some(p) => {
                let mut parent_id = shape.parent_id();
                let mut point = shape.inv().transform_point3a(hit.point);
                while parent_id.is_some() {
                    let parent_shape = scene.find_shape_by_id(parent_id.unwrap());
                    point = parent_shape.inv().transform_point3a(point);
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
