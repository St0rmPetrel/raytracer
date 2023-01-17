use std::ops::{Add, Mul, MulAssign};

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn set(&mut self, color: &Color) {
        self.r = color.r;
        self.g = color.g;
        self.b = color.b;
    }
}

impl Add for &Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r / 2 + rhs.r / 2,
            g: self.g / 2 + rhs.g / 2,
            b: self.b / 2 + rhs.b / 2,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: mul_primary_color(self.r, rhs),
            g: mul_primary_color(self.g, rhs),
            b: mul_primary_color(self.b, rhs),
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = mul_primary_color(self.r, rhs);
        self.g = mul_primary_color(self.g, rhs);
        self.b = mul_primary_color(self.b, rhs);
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        fn norm_u8(byte: u8) -> f32 {
            byte as f32 / u8::MAX as f32
        }
        let (r, g, b) = (norm_u8(rhs.r), norm_u8(rhs.g), norm_u8(rhs.b));
        Self {
            r: mul_primary_color(self.r, r),
            g: mul_primary_color(self.g, g),
            b: mul_primary_color(self.b, b),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r {}, g {}, b {})", self.r, self.g, self.b)
    }
}

fn mul_primary_color(color: u8, rhs: f32) -> u8 {
    let color = (color as f32) * rhs;
    if color > u32::MAX as f32 {
        u8::MAX
    } else {
        color as u8
    }
}
