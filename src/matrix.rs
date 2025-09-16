
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Mul;

use crate::vector::Vector;
use crate::point::Point;

pub struct Matrix<const D: usize> {
    items: [[f32; D]; D]
}

trait Det {
    
    fn det(&self) -> f32;
}

impl<const D: usize> Matrix<D> {

    fn zero() -> Self {
        Self { items: [[0.0; D]; D] }
    }

    fn submatrix<const OUT: usize>(&self, iExclude: usize, jExclude: usize) -> Matrix<OUT> {
        debug_assert!(OUT + 1 == D);
        let mut result = Matrix::<OUT>::zero();

        let mut iOffset = 0;
        for i in 0..D {
            if i == iExclude {
                iOffset = 1;
                continue;
            }
            let mut jOffset = 0;
            for j in 0..D {
                if j == jExclude {
                    jOffset = 1;
                    continue;
                }

                result.items[i - iOffset][j - jOffset] = self.items[i][j];
            }
        }

        result
    }

    fn minor<const OUT: usize>(&self, i: usize, j: usize) -> f32 where Matrix<OUT>: Det {
        debug_assert!(OUT + 1 == D);
        let s = self.submatrix::<OUT>(i, j);
        Det::det(&s)
    }

    fn cofactor<const OUT: usize>(&self, i: usize, j: usize) -> f32 where Matrix<OUT>: Det {
        debug_assert!(OUT + 1 == D);
        let m = self.minor::<OUT>(i, j);
        if (i + j) % 2 == 0 {
            return m;
        }
        return -m;
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

impl<const D: usize> Mul for Matrix<D> {

    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..D {
            for j in 0..D {
                let mut sum: f32 = 0.0;
                for k in 0..D {
                    sum += self.items[i][k] * other.items[k][j];
                }
                result.items[i][j] = sum;
            }
        }
        result
    }
}

impl Mul<Vector> for &Matrix<4> {

    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: rhs.x * self.items[0][0] + rhs.y * self.items[0][1] + rhs.z * self.items[0][2],
            y: rhs.x * self.items[1][0] + rhs.y * self.items[1][1] + rhs.z * self.items[1][2],
            z: rhs.x * self.items[2][0] + rhs.y * self.items[2][1] + rhs.z * self.items[2][2]
        }
    }
}

impl Mul<Point> for &Matrix<4> {

    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: rhs.x * self.items[0][0] + rhs.y * self.items[0][1] + rhs.z * self.items[0][2] + self.items[0][3],
            y: rhs.x * self.items[1][0] + rhs.y * self.items[1][1] + rhs.z * self.items[1][2] + self.items[1][3],
            z: rhs.x * self.items[2][0] + rhs.y * self.items[2][1] + rhs.z * self.items[2][2] + self.items[2][3]
        }
    }
}

impl Det for Matrix<2> {

    fn det(&self) -> f32 {
        self.items[0][0] * self.items[1][1] - self.items[0][1] * self.items[1][0]
    }
}

impl Det for Matrix<3> {

    fn det(&self) -> f32 {
        self.items[0][0] * self.cofactor::<2>(0, 0) +
        self.items[0][1] * self.cofactor::<2>(0, 1) +
        self.items[0][2] * self.cofactor::<2>(0, 2)
    }
}

impl Det for Matrix<4> {

    fn det(&self) -> f32 {
        self.items[0][0] * self.cofactor::<3>(0, 0) +
        self.items[0][1] * self.cofactor::<3>(0, 1) +
        self.items[0][2] * self.cofactor::<3>(0, 2) +
        self.items[0][3] * self.cofactor::<3>(0, 3)
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

    pub fn inverse(self) -> Self {
        let mut result = Self::zero();

        let d = self.det();
        for i in 0..4 {
            for j in 0..4 {
                result.items[j][i] = self.cofactor::<3>(i, j) / d;
            }
        }

        result
    }
}
