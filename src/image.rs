use color::Color;
use core::str;
use std::{
    fs::File,
    io::{self, Write},
};

pub mod color;

pub struct RasterImage {
    name: String,
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl RasterImage {
    pub fn new(name: &str, width: usize, height: usize) -> RasterImage {
        RasterImage {
            width,
            height,
            name: String::from(name),
            pixels: vec![Color::new(0, 0, 0); width * height],
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
}
