use glam::Affine3A;

#[inline]
pub fn translate(x: f32, y: f32, z: f32) -> Affine3A {
    Affine3A::from_translation(glam::vec3(x, y, z))
}

#[inline]
pub fn scale(x: f32, y: f32, z: f32) -> Affine3A {
    Affine3A::from_scale(glam::vec3(x, y, z))
}

#[inline]
pub fn rotate_xd(degrees: f32) -> Affine3A {
    Affine3A::from_rotation_x(degrees.to_radians())
}

#[inline]
pub fn rotate_yd(degrees: f32) -> Affine3A {
    Affine3A::from_rotation_y(degrees.to_radians())
}

#[inline]
pub fn rotate_zd(degrees: f32) -> Affine3A {
    Affine3A::from_rotation_z(degrees.to_radians())
}
