use crate::canvas::Canvas;
use crate::config::CameraConfig;
use crate::image::RasterImage;
use crate::scene::Scene;

pub fn fill_image(image: &mut RasterImage, camera: CameraConfig, scene: Scene) {
    let (width, height) = image.get_resolution();
    let resolution = std::cmp::max(width, height);

    let mut convas = Canvas::new(&camera, resolution + 1);

    let (h_shift, w_shift) = if height < width {
        ((resolution - height) / 2, 0)
    } else {
        (0, (resolution - width) / 2)
    };

    convas.fill_canvas(
        scene,
        (w_shift, resolution - w_shift + 1),
        (h_shift, resolution - h_shift + 1),
    );

    for j in 0..height {
        for i in 0..width {
            let pixel = image.get_pixel(i, j).expect("pixel in image not found");
            let color_ul = convas
                .get_canvas_pixel(i + w_shift, j + h_shift)
                .expect("pixel in convas not found");
            let color_ur = convas
                .get_canvas_pixel(i + w_shift + 1, j + h_shift)
                .expect("pixel in convas not found");
            let color_bl = convas
                .get_canvas_pixel(i + w_shift, j + h_shift + 1)
                .expect("pixel in convas not found");
            let color_br = convas
                .get_canvas_pixel(i + w_shift + 1, j + h_shift + 1)
                .expect("pixel in convas not found");
            let color_u = color_ul + color_ur;
            let color_b = color_bl + color_br;
            let color = &color_b + &color_u;
            pixel.set(&color);
        }
    }
}
