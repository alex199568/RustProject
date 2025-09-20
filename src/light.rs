use crate::color::Color;
use crate::intersection::Hit;
use crate::material::Material;

use glam::Vec3A;

pub struct Light {
    pub position: Vec3A,
    pub intensity: Color,
}

impl Light {
    pub fn shade(&self, material: &Material, material_color: Color, hit: &Hit) -> Color {
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
