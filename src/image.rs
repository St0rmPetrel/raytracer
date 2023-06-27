use crate::config::ImageConfig;
pub use color::Color;
use core::str;
use std::{
    fs::File,
    io::{self, Write},
};

pub mod color {
    use std::ops::{Add, Mul};

    /// Color in RGB model
    #[derive(Clone, Debug)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        /// Return new Color with given primary colors
        pub fn new(r: u8, g: u8, b: u8) -> Color {
            Color { r, g, b }
        }

        /// Return new Color with given primary colors from array
        pub fn new_from_arr(rgb: &[u8; 3]) -> Color {
            Color {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            }
        }

        /// Set primary color of the light
        pub fn set(&mut self, color: &Color) {
            self.r = color.r;
            self.g = color.g;
            self.b = color.b;
        }

        /// Add to Color reflection color
        /// rfl is reflection coefficient
        pub fn add_refl(self, rfl: f32, rht: Color) -> Color {
            &((1.0 - rfl) * self) + &(rfl * rht)
        }
    }

    impl Add for &Color {
        type Output = Color;

        fn add(self, rhs: Self) -> Self::Output {
            Color {
                r: hypot_u8(self.r, rhs.r),
                g: hypot_u8(self.g, rhs.g),
                b: hypot_u8(self.b, rhs.b),
            }
        }
    }

    impl Mul<f32> for Color {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self {
                r: mul_u8_f32(self.r, rhs),
                g: mul_u8_f32(self.g, rhs),
                b: mul_u8_f32(self.b, rhs),
            }
        }
    }
    impl Mul<Color> for f32 {
        type Output = Color;

        fn mul(self, rhs: Color) -> Self::Output {
            Color {
                r: mul_u8_f32(rhs.r, self),
                g: mul_u8_f32(rhs.g, self),
                b: mul_u8_f32(rhs.b, self),
            }
        }
    }

    fn mul_u8_f32(color: u8, rhs: f32) -> u8 {
        let color = (color as f32) * rhs;
        if color > u32::MAX as f32 {
            u8::MAX
        } else {
            color as u8
        }
    }

    fn hypot_u8(a: u8, b: u8) -> u8 {
        let h = ((a as f32).powi(2) + (b as f32).powi(2)).sqrt();
        if h > u8::MAX as f32 {
            return u8::MAX;
        }
        h as u8
    }
}

pub struct RasterImage {
    name: String,
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl RasterImage {
    /// Create new empty raster image with given name and resolution
    pub fn new(cfg: ImageConfig) -> RasterImage {
        RasterImage {
            width: cfg.width,
            height: cfg.height,
            name: cfg.name.clone(),
            pixels: vec![Color::new(0, 0, 0); cfg.width * cfg.height],
        }
    }

    /// Save image in ppm format in current directory in file called by name of the image
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

    /// Return mutable pixel, you can change color of this pixel
    /// # Example
    /// ```rust
    /// let p = image.get_pixel(0, 0).expect("pixel not found");
    /// p.set(&Color::new(255, 0, 0));
    /// ```
    pub fn get_pixel(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        let index = self.width * y + x;
        self.pixels.get_mut(index)
    }
    /// Return resolution width x height of the image
    pub fn get_resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
