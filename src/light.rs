
use crate::point::Point;
use crate::color::Color;
use crate::material::Material;
use crate::vector::Vector;

pub struct Light {
    pub position: Point,
    pub intensity: Color
}

impl Light {

    pub fn shade(&self, material: &Material, point: Point, eye: Vector, normal: Vector) -> Color {
        // 1.0 - shadow
        let effective_color = material.color * self.intensity * (1.0);
        let light_v = (self.position - point).unit();
        let ambient= effective_color * material.ambient;
        let light_dot_normal = light_v.dot(normal);
        if light_dot_normal < 0.0 {
            return ambient;
        }

        let diffuse = effective_color * (material.diffuse * light_dot_normal);
        let reflect = (-light_v).reflect(normal);
        let reflect_dot_eye = reflect.dot(eye);
        if reflect_dot_eye <= 0.0 {
            return ambient + diffuse;
        }

        let shininess = material.shininess;
        let factor = reflect_dot_eye.powf(shininess);
        let specular = self.intensity * (material.specular * factor);

        ambient + diffuse + specular
    }
}
