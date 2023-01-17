pub mod image;
pub mod raytrace;

use image::RasterImage;

fn main() {
    const RESOLUTION: usize = 1280 + 1;

    const HD_W: usize = 1280;
    const HD_H: usize = 720;

    let mut image = RasterImage::new("test", HD_W, HD_H);
    let mut convas = raytrace::canvas::Canvas::new(15.0, RESOLUTION);

    let sphere_r = raytrace::shape::new(raytrace::vector::Vector::new(0.0, 3.0, 0.0), 4.0);
    let sphere_g = raytrace::shape::new(raytrace::vector::Vector::new(-3.0, -2.0, 0.0), 4.0);
    let sphere_b = raytrace::shape::new(raytrace::vector::Vector::new(3.0, -2.0, 0.0), 4.0);

    let mut scene = raytrace::scene::Scene::new();
    scene.push_object(sphere_r, image::color::Color::new(255, 0, 0));
    scene.push_object(sphere_g, image::color::Color::new(0, 255, 0));
    scene.push_object(sphere_b, image::color::Color::new(0, 0, 255));

    scene.push_light(raytrace::vector::Vector::new(0.0, 0.0, 10.0));

    convas.fill_canvas(scene, (0, 1281), (280, 1001));

    for j in 0..HD_H {
        for i in 0..HD_W {
            let pixel = image.get_pixel(i, j).expect("pixel in image not found");
            let color_ul = convas
                .get_canvas_pixel(i, j + 280)
                .expect("pixel in convas not found");
            let color_ur = convas
                .get_canvas_pixel(i + 1, j + 280)
                .expect("pixel in convas not found");
            let color_bl = convas
                .get_canvas_pixel(i, j + 281)
                .expect("pixel in convas not found");
            let color_br = convas
                .get_canvas_pixel(i + 1, j + 281)
                .expect("pixel in convas not found");
            let color_u = color_ul + color_ur;
            let color_b = color_bl + color_br;
            let color = &color_b + &color_u;
            pixel.set(&color);
        }
    }

    image.save_ppm().expect("can't save ruster image");
}
