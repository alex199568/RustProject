use crate::color::Color;
use crate::img::Img;
use crate::material::Material;
use crate::pattern::{PlanarTexture, UvImage};
use crate::shape::{Group, Shape, Triangle};

use glam::Affine3A;
use glam::Vec2;

use std::path::Path;

fn to_material(obj_mat: &tobj::Material, obj_dir: &Path) -> Material {
    let texture = if let Some(dt) = obj_mat.diffuse_texture.as_ref() {
        let tex_path = obj_dir.join(dt);
        let img = Img::load(tex_path).unwrap();
        let uv_image = UvImage::new(img, true);
        Some(PlanarTexture::new(uv_image.into()).into())
    } else {
        None
    };

    Material::builder()
        .color(Color::option(obj_mat.diffuse))
        // .diffuse(Color::option(obj_mat.diffuse).r)
        // .ambient(Color::option(obj_mat.ambient).r)
        .shininess(obj_mat.shininess.unwrap_or(0.0))
        .refraction(obj_mat.optical_density.unwrap_or(1.0))
        .transparency(1.0 - obj_mat.dissolve.unwrap_or(1.0))
        .specular(Color::option(obj_mat.specular).r())
        .pattern(texture)
        .build()
}

pub fn load_obj<P: AsRef<Path>>(file_path: P, tr: &Affine3A) -> (Vec<Material>, Shape) {
    let path_ref: &Path = file_path.as_ref();
    let obj_dir = path_ref.parent().unwrap_or_else(|| Path::new("."));
    let (models, materials) = tobj::load_obj(
        path_ref,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .expect("Failed to load obj");

    let mats = materials.expect("Failed to load materials");

    let mut tris: Vec<Shape> = Vec::new();
    let mut materials: Vec<Material> = Vec::new();

    for model in models.iter() {
        let mesh = &model.mesh;

        let pos = &mesh.positions; // len = 3 * num_verts
        let nrm = &mesh.normals; // len = 3 * num_verts (or 0)
        let uv = &mesh.texcoords; // len = 2 * num_verts (or 0)
        let idx = &mesh.indices; // len = 3 * num_tris

        let num_verts = pos.len() / 3;
        let has_nrm = nrm.len() == 3 * num_verts;
        let has_uv = uv.len() == 2 * num_verts;

        // // Pick material (or a default)
        let material = mesh
            .material_id
            .and_then(|i| mats.get(i))
            .map(|m| to_material(m, obj_dir))
            .unwrap_or_else(|| Material::builder().build());
        let material_id = material.id;
        materials.push(material);

        for tri in idx.chunks(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            // positions
            let p0 = glam::vec3a(pos[3 * i0], pos[3 * i0 + 1], pos[3 * i0 + 2]);
            let p1 = glam::vec3a(pos[3 * i1], pos[3 * i1 + 1], pos[3 * i1 + 2]);
            let p2 = glam::vec3a(pos[3 * i2], pos[3 * i2 + 1], pos[3 * i2 + 2]);

            // // optional per-vertex normals (object space)
            let (n1, n2, n3) = if has_nrm {
                (
                    Some(glam::vec3a(nrm[3 * i0], nrm[3 * i0 + 1], nrm[3 * i0 + 2]).normalize()),
                    Some(glam::vec3a(nrm[3 * i1], nrm[3 * i1 + 1], nrm[3 * i1 + 2]).normalize()),
                    Some(glam::vec3a(nrm[3 * i2], nrm[3 * i2 + 1], nrm[3 * i2 + 2]).normalize()),
                )
            } else {
                (None, None, None)
            };

            // optional per-vertex UVs
            let (uv1, uv2, uv3) = if has_uv {
                (
                    Some(Vec2::new(uv[2 * i0], uv[2 * i0 + 1])),
                    Some(Vec2::new(uv[2 * i1], uv[2 * i1 + 1])),
                    Some(Vec2::new(uv[2 * i2], uv[2 * i2 + 1])),
                )
            } else {
                (None, None, None)
            };

            let tri = Triangle::new(material_id, p0, p1, p2, n1, n2, n3, uv1, uv2, uv3);
            tris.push(tri.into());
        }
    }

    let mut result = Group::new(tr, tris);
    result.divide(1);

    (materials, result.into())
}
