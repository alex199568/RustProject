
use std::fmt::Display;
use std::fmt::Formatter;

pub struct MatrixFmt<'a>(pub &'a [f32; 16]);

impl Display for MatrixFmt<'_> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m = self.0;
        writeln!(f, "{}, {}, {}, {}", m[0], m[1], m[2], m[3])?;
        writeln!(f, "{}, {}, {}, {}", m[4], m[5], m[6], m[7])?;
        writeln!(f, "{}, {}, {}, {}", m[8], m[9], m[10], m[11])?;
        write!(f, "{}, {}, {}, {}", m[12], m[13], m[14], m[15])
    }
}

pub fn translate(x: f32, y: f32, z: f32) -> [f32; 16] {
    return [
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, z,
        0.0, 0.0, 0.0, 1.0
    ];
}
