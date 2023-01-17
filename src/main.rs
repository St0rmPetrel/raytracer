pub mod image;
pub mod raytrace;

use image::RasterImage;

fn main() {
    const RESOLUTION: usize = 1024;
    let mut image = RasterImage::new("test", RESOLUTION, RESOLUTION);
    let convas = raytrace::canvas::Canvas::new(15.0, RESOLUTION);
    let sphere_r = raytrace::shape::new(raytrace::vector::Vector::new(0.0, 3.0, 0.0), 4.0);
    let sphere_g = raytrace::shape::new(raytrace::vector::Vector::new(-3.0, -2.0, 0.0), 4.0);
    let sphere_b = raytrace::shape::new(raytrace::vector::Vector::new(3.0, -2.0, 0.0), 4.0);

    let mut scene = raytrace::scene::Scene::new();
    scene.push_object(sphere_r, image::color::Color::new(255, 0, 0));
    scene.push_object(sphere_g, image::color::Color::new(0, 255, 0));
    scene.push_object(sphere_b, image::color::Color::new(0, 0, 255));

    scene.push_light(raytrace::vector::Vector::new(0.0, 0.0, 10.0));

    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let ray = convas.get_ray(i, j);
            let ray_color = scene.get_ray_color(&ray);
            let pixel = image.get_pixel(i, j).expect("pixel out of bound");
            pixel.set(&ray_color);
        }
    }

    image.save_ppm().expect("can't save ruster image");
}
