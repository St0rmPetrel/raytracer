pub mod image;
pub mod raytrace;

use image::RasterImage;

use crate::raytrace::shape::Shape;

fn main() {
    const RESOLUTION: usize = 1024;
    let mut image = RasterImage::new("test", RESOLUTION, RESOLUTION);
    let convas = raytrace::canvas::Canvas::new(10.0, RESOLUTION);
    let sphere = raytrace::shape::new(raytrace::vector::Vector::new(0.0, 0.0, 0.0), 3.0);
    let litght = raytrace::vector::Vector::new(0.0, 0.0, 10.0);

    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let ray = convas.get_ray(i, j);
            let t = match sphere.intersec(&ray) {
                raytrace::shape::Intersec::None => continue,
                raytrace::shape::Intersec::OneRoot(t) => t,
                raytrace::shape::Intersec::TwoRoot(t1, t2) => {
                    if t1 < t2 {
                        t1
                    } else {
                        t2
                    }
                }
            };
            let intersec = ray.point_on_ray(t);
            let mut l = &litght - &intersec;
            l.norm();
            let norm = sphere.norm(&intersec);
            let intensity = norm.dot(&l);
            let mut pixel = image.get_pixel(i, j).expect("pixel out of bound");
            pixel.b = (255.0 * intensity) as u8;
        }
    }

    image.save_ppm().expect("can't save ruster image");
}
