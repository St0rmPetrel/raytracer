use crate::canvas::Canvas;
use crate::image::RasterImage;
use crate::scene::Scene;

pub fn fill_image(image: &mut RasterImage, scene: Scene) {
    let (width, height) = image.get_resolution();
    let resolution = std::cmp::max(width, height) + 1;

    let mut convas = Canvas::new(15.0, resolution);

    convas.fill_canvas(scene, (0, 1281), (280, 1001));

    for j in 0..height {
        for i in 0..width {
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
}
