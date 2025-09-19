use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;

use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn option(o: Option<[f32; 3]>) -> Self {
        match o {
            Some(items) => Self {
                r: items[0],
                g: items[1],
                b: items[2],
            },
            None => Self::BLACK,
        }
    }

    const fn hex(value: i32) -> Self {
        let r = ((value >> 16) & 0xFF) as f32 / 255.0;
        let g = ((value >> 8) & 0xFF) as f32 / 255.0;
        let b = (value & 0xFF) as f32 / 255.0;
        Self { r, g, b }
    }

    #[inline]
    pub fn srgb8(self) -> [u8; 3] {
        fn enc(x: f32) -> u8 {
            let y = x.clamp(0.0, 1.0);
            (y * 255.0) as u8
        }
        [enc(self.r), enc(self.g), enc(self.b)]
    }

    pub const WHITE: Color = Color::hex(0xFFFFFF);
    pub const SNOW: Color = Color::hex(0xFFFAFA);
    pub const GAINSBORO: Color = Color::hex(0xDCDCDC);
    pub const LIGHT_GRAY: Color = Color::hex(0xD3D3D3);
    pub const SILVER: Color = Color::hex(0xC0C0C0);
    pub const GRAY: Color = Color::hex(0x808080);
    pub const DARK_GRAY: Color = Color {
        r: 0.25,
        g: 0.25,
        b: 0.25,
    };
    pub const DARK_SLATE_GRAY: Color = Color::hex(0x2F4F4F);
    pub const BLACK: Color = Color::hex(0x000000);

    pub const RED: Color = Color::hex(0xFF0000);
    pub const INDIAN_RED: Color = Color::hex(0xCD5C5C);
    pub const GREEN: Color = Color::hex(0x00FF00);
    pub const FOREST_GREEN: Color = Color::hex(0x228B22);
    pub const BLUE: Color = Color::hex(0x0000FF);
    pub const STEEL_BLUE: Color = Color::hex(0x4682B4);

    pub const YELLOW: Color = Color::hex(0xFFFF00);
    pub const GOLD: Color = Color::hex(0xFFD700);
    pub const CYAN: Color = Color::hex(0x00FFFF);
    pub const TURQUOISE: Color = Color::hex(0x40E0D0);
    pub const MAGENTA: Color = Color::hex(0xFF00FF);
    pub const VIOLET: Color = Color::hex(0xEE82EE);

    pub const ALICE_BLUE: Color = Color::hex(0xF0F8FF);
    pub const LAVENDER: Color = Color::hex(0xe6e6fa);
    pub const MISTY_ROSE: Color = Color::hex(0xffe4e1);
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, n: f32) -> Color {
        Color {
            r: self.r * n,
            g: self.g * n,
            b: self.b * n,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, n: f32) -> Color {
        Color {
            r: self.r / n,
            g: self.g / n,
            b: self.b / n,
        }
    }
}

#[derive(Clone)]
pub struct AccColor {
    color: Color,
    count: usize,
}

impl AddAssign<Color> for AccColor {
    fn add_assign(&mut self, rhs: Color) {
        self.color += rhs;
        self.count += 1;
    }
}

impl AccColor {
    pub fn new() -> Self {
        Self {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            count: 0,
        }
    }

    pub fn get(&self) -> Color {
        if self.count == 0 {
            return self.color;
        }
        self.color / self.count as f32
    }
}
