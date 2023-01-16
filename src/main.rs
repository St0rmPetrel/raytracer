pub mod image;
pub mod raytrace;

use image::RasterImage;

fn main() {
    const RESOLUTION: usize = 1024;
    let mut image = RasterImage::new("test", RESOLUTION, RESOLUTION);
    let convas = raytrace::canvas::Canvas::new(15.0, RESOLUTION);
    let sphere = raytrace::shape::new(raytrace::vector::Vector::new(2.0, 1.0, -5.0), 3.0);

    let mut scene = raytrace::scene::Scene::new();
    scene.push_object(sphere, image::color::Color::new(255, 20, 20));

    let light = raytrace::light::Light::new(raytrace::vector::Vector::new(-4.0, 0.0, 2.0));

    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let ray = convas.get_ray(i, j);
            let intersec = match scene.intersec(&ray) {
                None => continue,
                Some(intersec) => intersec,
            };

            let intensity = light.intensity(&intersec.point, &intersec.norm);

            let mut pixel = image.get_pixel(i, j).expect("pixel out of bound");
            pixel.g = (255.0 * intensity) as u8;
        }
    }

    image.save_ppm().expect("can't save ruster image");
}
