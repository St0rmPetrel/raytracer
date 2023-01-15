pub mod image;
pub mod raytrace;

use image::RasterImage;

fn main() {
    const RESOLUTION: usize = 1024;
    let mut image = RasterImage::new("test", RESOLUTION, RESOLUTION);
    let convas = raytrace::canvas::Canvas::new(10.0, RESOLUTION);

    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let ray = convas.get_ray(i, j);
            if ray.is_sphere_intersec(raytrace::vector::Vector::new(0.0, 0.0, 0.0), 3.0) {
                let mut pixel = image.get_pixel(i, j).expect("pixel out of bound");
                pixel.r = 255;
            }
        }
    }

    image.save_ppm().expect("can't save ruster image");
}
