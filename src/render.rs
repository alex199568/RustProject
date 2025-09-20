use crate::scene::Scene;
use crate::shape::Shape;

use crate::img::AccImg;
use crate::intersection::IntersectionBuffer;

use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;

pub fn render<P: AsRef<Path>>(scene: &Scene, filepath: P) {
    let mut aimg = AccImg::new(scene.camera.w, scene.camera.h);
    let aa = 4;
    let depth = 4;

    println!("Rendering threads: {}", rayon::current_num_threads());

    let capacity = scene
        .shapes
        .iter()
        .map(|s: &Shape| s.max_intersections())
        .sum();

    let start = Instant::now();

    let w = aimg.w;
    aimg.colors.par_chunks_mut(w).enumerate().for_each_init(
        || IntersectionBuffer::new(capacity),
        |buffer, (y, row)| {
            let fy = y as f32;
            for x in 0..w {
                let fx = x as f32;
                for sy in 0..aa {
                    for sx in 0..aa {
                        let u = fx + (sx as f32 + 0.5) / aa as f32;
                        let v = fy + (sy as f32 + 0.5) / aa as f32;
                        let r = scene.camera.ray(u, v);
                        let c = scene.color(&r, buffer, depth);
                        row[x] += c;
                    }
                }
            }
        },
    );

    let elapsed = start.elapsed();
    println!("Rendering time: {:.3} ms", elapsed.as_secs_f64() * 1e3);

    let filepath_ref = filepath.as_ref();
    match aimg.img().save(filepath_ref) {
        Ok(_) => println!("Render saved to: {}", filepath_ref.display()),
        Err(e) => eprintln!("Failed to save image: {}", e),
    }
}
