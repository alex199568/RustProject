use glam::Affine3A;

pub fn scale(x: f32, y: f32, z: f32) -> Affine3A {
    Affine3A::from_scale(glam::vec3(x, y, z))
}

pub fn rotate_yd(degrees: f32) -> Affine3A {
    Affine3A::from_rotation_y(degrees.to_radians())
}
