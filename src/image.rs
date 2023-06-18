use crate::config::ImageConfig;
pub use color::Color;
use core::str;
use std::{
    fs::File,
    io::{self, Write},
};

pub mod color {
    use std::ops::{Add, Mul, MulAssign};

    #[derive(Clone, Debug)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub fn new(r: u8, g: u8, b: u8) -> Color {
            Color { r, g, b }
        }

        pub fn new_from_arr(rgb: &[u8; 3]) -> Color {
            Color {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            }
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

    fn mul_primary_color(color: u8, rhs: f32) -> u8 {
        let color = (color as f32) * rhs;
        if color > u32::MAX as f32 {
            u8::MAX
        } else {
            color as u8
        }
    }
}

pub struct RasterImage {
    name: String,
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl RasterImage {
    pub fn new(cfg: &ImageConfig) -> RasterImage {
        RasterImage {
            width: cfg.width,
            height: cfg.height,
            name: cfg.name.clone(),
            pixels: vec![Color::new(0, 0, 0); cfg.width * cfg.height],
        }
    }

    pub fn save_ppm(&self) -> Result<(), io::Error> {
        const MAGIC_NUM: &str = "P6";

        let filename = [self.name.as_str(), "ppm"].join(".");

        let mut file = File::create(filename)?;
        let header = [
            MAGIC_NUM,
            self.width.to_string().as_str(),
            self.height.to_string().as_str(),
            u8::MAX.to_string().as_str(),
        ]
        .join("\n");

        file.write_all(header.as_bytes())?;
        file.write_all(b"\n")?;
        for pixel in self.pixels.iter() {
            file.write_all(&[pixel.r, pixel.g, pixel.b])?;
        }
        Ok(())
    }
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        let index = self.width * y + x;
        self.pixels.get_mut(index)
    }
    pub fn get_resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
