
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Mul;

pub struct Matrix<const D: usize> {
    items: [[f32; D]; D]
}

impl<const D: usize> Matrix<D> {

    fn zero() -> Self {
        Self { items: [[0.0; D]; D] }
    }
}


impl<const D: usize> Display for Matrix<D> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..D {
            for j in 0..D {
                write!(f, "\t{}", self.items[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Matrix<4> {

    pub const IDENTITY: Self = Self {
        items: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    };

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self {
            items: [
                [ 1.0, 0.0, 0.0, x ],
                [ 0.0, 1.0, 0.0, y ],
                [ 0.0, 0.0, 1.0, z ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            items: [
                [ x, 0.0, 0.0, 0.0 ],
                [ 0.0, y, 0.0, 0.0 ],
                [ 0.0, 0.0, z, 0.0 ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }
}
