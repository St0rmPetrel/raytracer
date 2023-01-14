pub mod image;
use image::RasterImage;

fn main() {
    let mut image = RasterImage::new("test", 8, 4);

    let red = image::color::Color::new(255, 0, 0);
    let mut pixel = image.get_pixel(3, 1).expect("pixel out of bound");
    pixel.r = red.r;

    image.save_ppm().expect("can't save ruster image");
}
