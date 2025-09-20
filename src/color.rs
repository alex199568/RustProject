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
            None => Gray::BLACK,
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
            (x.clamp(0.0, 1.0) * 255.0) as u8
        }
        [enc(self.r), enc(self.g), enc(self.b)]
    }

    #[inline]
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
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

pub struct Red;

impl Red {
    pub const INDIAN_RED: Color = Color::hex(0xCD5C5C);
    pub const LIGHT_CORAL: Color = Color::hex(0xF08080);
    pub const SALMON: Color = Color::hex(0xFA8072);
    pub const DAKR_SALMON: Color = Color::hex(0xE9967A);
    pub const LIGHT_SALMON: Color = Color::hex(0xFFA07A);
    pub const CRIMSON: Color = Color::hex(0xDC143C);
    pub const RED: Color = Color::hex(0xFF0000);
    pub const FIRE_BRICK: Color = Color::hex(0xB22222);
    pub const DARK_RED: Color = Color::hex(0x8B0000);
}

pub struct Pink;

impl Pink {
    pub const PINK: Color = Color::hex(0xFFC0CB);
    pub const LIGHT_PINK: Color = Color::hex(0xFFB6C1);
    pub const HOT_PINK: Color = Color::hex(0xFF69B4);
    pub const DEEP_PINK: Color = Color::hex(0xFF1493);
    pub const MEDIUM_VIOLET_RED: Color = Color::hex(0xC71585);
    pub const PALE_VIOLET_RED: Color = Color::hex(0xDB7093);
}

pub struct Orange;

impl Orange {
    pub const LIGHT_SALMON: Color = Color::hex(0xFFA07A);
    pub const CORAL: Color = Color::hex(0xFF7F50);
    pub const TOMATO: Color = Color::hex(0xFF6347);
    pub const ORANGE_RED: Color = Color::hex(0xFF4500);
    pub const DARK_ORANGE: Color = Color::hex(0xFF8C00);
    pub const ORANGE: Color = Color::hex(0xFFA500);
}

pub struct Yellow;

impl Yellow {
    pub const GOLD: Color = Color::hex(0xFFD700);
    pub const YELLOW: Color = Color::hex(0xFFFF00);
    pub const LIGHT_YELLOW: Color = Color::hex(0xFFFFE0);
    pub const LEMON_CHIFFON: Color = Color::hex(0xFFFACD);
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::hex(0xFFEFD5);
    pub const PAPAYA_WHIP: Color = Color::hex(0xFFEFD5);
    pub const MOCCASIN: Color = Color::hex(0xFFE4B5);
    pub const PEACH_PUFF: Color = Color::hex(0xFFDAB9);
    pub const PALE_GOLDENROD: Color = Color::hex(0xEEE8AA);
    pub const KHAKI: Color = Color::hex(0xF0E68C);
    pub const DARK_KHAKI: Color = Color::hex(0xBDB76B);
}

pub struct Purple;

impl Purple {
    pub const LAVENDER: Color = Color::hex(0xE6E6FA);
    pub const THISTLE: Color = Color::hex(0xD8BFD8);
    pub const PLUM: Color = Color::hex(0xDDA0DD);
    pub const VIOLET: Color = Color::hex(0xEE82EE);
    pub const ORCHID: Color = Color::hex(0xDA70D6);
    pub const FUCHSIA: Color = Color::hex(0xFF00FF);
    pub const MAGENTA: Color = Color::hex(0xFF00FF);
    pub const MEDIUM_ORCHID: Color = Color::hex(0xBA55D3);
    pub const MEDIUM_PURPLE: Color = Color::hex(0x9370DB);
    pub const REBECCA_PURPLE: Color = Color::hex(0x663399);
    pub const BLUE_VIOLET: Color = Color::hex(0x8A2BE2);
    pub const DARK_VIOLET: Color = Color::hex(0x9400D3);
    pub const DARK_ORCHID: Color = Color::hex(0x9932CC);
    pub const DARK_MAGENTA: Color = Color::hex(0x8B008B);
    pub const PURPLE: Color = Color::hex(0x800080);
    pub const INDIGO: Color = Color::hex(0x4B0082);
    pub const SLATE_BLUE: Color = Color::hex(0x6A5ACD);
    pub const DARK_SLATE_BLUE: Color = Color::hex(0x483D8B);
    pub const MEDIUM_SLATE_BLUE: Color = Color::hex(0x7B68EE);
}

pub struct Green;

impl Green {
    pub const GREEN_YELLOW: Color = Color::hex(0xADFF2F);
    pub const CHARTRUSE: Color = Color::hex(0x7FFF00);
    pub const LAWN_GREEN: Color = Color::hex(0x7CFC00);
    pub const LIME: Color = Color::hex(0x00FF00);
    pub const LIME_GREEN: Color = Color::hex(0x32CD32);
    pub const PALE_GREEN: Color = Color::hex(0x98FB98);
    pub const LIGHT_GREEN: Color = Color::hex(0x90EE90);
    pub const MEDIUM_SPRING_GREEN: Color = Color::hex(0x00FA9A);
    pub const SPRING_GREEN: Color = Color::hex(0x00FF7F);
    pub const MEDIUM_SEA_GREEN: Color = Color::hex(0x3CB371);
    pub const SEA_GREEN: Color = Color::hex(0x2E8B57);
    pub const FOREST_GREEN: Color = Color::hex(0x228B22);
    pub const GREEN: Color = Color::hex(0x008000);
    pub const DARK_GREEN: Color = Color::hex(0x006400);
    pub const YELLOW_GREEN: Color = Color::hex(0x9ACD32);
    pub const OLIVE_DRAB: Color = Color::hex(0x6B8E23);
    pub const OLIVE: Color = Color::hex(0x808000);
    pub const DARK_OLIVE_GREEN: Color = Color::hex(0x556B2F);
    pub const MEDIUM_AQUAMARINE: Color = Color::hex(0x66CDAA);
    pub const DARK_SEA_GREEN: Color = Color::hex(0x8FBC8B);
    pub const LIGHT_SEA_GREEN: Color = Color::hex(0x20B2AA);
    pub const DARK_CYAN: Color = Color::hex(0x008B8B);
    pub const TEAL: Color = Color::hex(0x008080);
}

pub struct Blue;

impl Blue {
    pub const AQUA: Color = Color::hex(0x00FFFF);
    pub const CYAN: Color = Color::hex(0x00FFFF);
    pub const LIGHT_CYAN: Color = Color::hex(0xE0FFFF);
    pub const PALE_TURQUOISE: Color = Color::hex(0xAFEEEE);
    pub const AQUAMARINE: Color = Color::hex(0x7FFFD4);
    pub const TURQUOISE: Color = Color::hex(0x40E0D0);
    pub const MEDIUM_TURQUOISE: Color = Color::hex(0x48D1CC);
    pub const DARK_TURQUOISE: Color = Color::hex(0x00CED1);
    pub const CADET_BLUE: Color = Color::hex(0x5F9EA0);
    pub const STEEL_BLUE: Color = Color::hex(0x4682B4);
    pub const LIGHT_STEEL_BLUE: Color = Color::hex(0xB0C4DE);
    pub const POWDER_BLUE: Color = Color::hex(0xB0E0E6);
    pub const LIGHT_BLUE: Color = Color::hex(0xADD8E6);
    pub const SKY_BLUE: Color = Color::hex(0x87CEEB);
    pub const LIGHT_BLUE_SKY: Color = Color::hex(0x87CEFA);
    pub const DEEP_SKY_BLUE: Color = Color::hex(0x00BFFF);
    pub const DODGER_BLUE: Color = Color::hex(0x1E90FF);
    pub const CORNFLOWER_BLUE: Color = Color::hex(0x6495ED);
    pub const MEDIUM_SLATE_BLUE: Color = Color::hex(0x7B68EE);
    pub const ROYAL_BLUE: Color = Color::hex(0x4169E1);
    pub const BLUE: Color = Color::hex(0x0000FF);
    pub const MEDIUM_BLUE: Color = Color::hex(0x0000CD);
    pub const DARK_BLUE: Color = Color::hex(0x00008B);
    pub const NAVY: Color = Color::hex(0x000080);
    pub const MIDNIGHT_BLUE: Color = Color::hex(0x191970);
}

pub struct Brown;

impl Brown {
    pub const CORNSILK: Color = Color::hex(0xFFF8DC);
    pub const BLANCHED_ALMOND: Color = Color::hex(0xFFEBCD);
    pub const BISQUE: Color = Color::hex(0xFFE4C4);
    pub const NAVAJO_WHITE: Color = Color::hex(0xFFDEAD);
    pub const WHEAT: Color = Color::hex(0xF5DEB3);
    pub const BURLY_WOOD: Color = Color::hex(0xDEB887);
    pub const TAN: Color = Color::hex(0xD2B48C);
    pub const ROSY_BROWN: Color = Color::hex(0xBC8F8F);
    pub const SANDY_BROWN: Color = Color::hex(0xF4A460);
    pub const GOLDENROD: Color = Color::hex(0xDAA520);
    pub const DARK_GOLDENROD: Color = Color::hex(0xB8860B);
    pub const PERU: Color = Color::hex(0xCD853F);
    pub const CHOCOLATE: Color = Color::hex(0xD2691E);
    pub const SADDLE_BROWN: Color = Color::hex(0x8B4513);
    pub const SIENNA: Color = Color::hex(0xA0522D);
    pub const BROWN: Color = Color::hex(0xA52A2A);
    pub const MAROON: Color = Color::hex(0x800000);
}

pub struct White;

impl White {
    pub const WHITE: Color = Color::hex(0xFFFFFF);
    pub const SNOW: Color = Color::hex(0xFFFAFA);
    pub const HONEY_DEW: Color = Color::hex(0xF0FFF0);
    pub const MINT_CREAM: Color = Color::hex(0xF5FFFA);
    pub const AZURE: Color = Color::hex(0xF0FFFF);
    pub const ALICE_BLUE: Color = Color::hex(0xF0F8FF);
    pub const GHOST_WHITE: Color = Color::hex(0xF8F8FF);
    pub const WHITE_SMOKE: Color = Color::hex(0xF5F5F5);
    pub const SEA_SHELL: Color = Color::hex(0xFFF5EE);
    pub const BEIGE: Color = Color::hex(0xF5F5DC);
    pub const OLD_LACE: Color = Color::hex(0xFDF5E6);
    pub const FLORAL_WHITE: Color = Color::hex(0xFFFAF0);
    pub const IVORY: Color = Color::hex(0xFFFFF0);
    pub const ANTIQUE_WHITE: Color = Color::hex(0xFAEBD7);
    pub const LINEN: Color = Color::hex(0xFAF0E6);
    pub const LAVENDER_BLUSH: Color = Color::hex(0xFFF0F5);
    pub const MISTY_ROSE: Color = Color::hex(0xFFE4E1);
}

pub struct Gray;

impl Gray {
    pub const GAINSBORO: Color = Color::hex(0xDCDCDC);
    pub const LIGHT_GRAY: Color = Color::hex(0xD3D3D3);
    pub const SILVER: Color = Color::hex(0xC0C0C0);
    pub const DARK_GRAY: Color = Color::hex(0xA9A9A9);
    pub const GRAY: Color = Color::hex(0x808080);
    pub const DIM_GRAY: Color = Color::hex(0x696969);
    pub const LIGHT_SLATE_GRAY: Color = Color::hex(0x778899);
    pub const SLATE_GRAY: Color = Color::hex(0x708090);
    pub const DARK_SLATE_GRAY: Color = Color::hex(0x2F4F4F);
    pub const BLACK: Color = Color::hex(0x000000);
}
