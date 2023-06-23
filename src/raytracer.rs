mod canvas;
mod ray;
mod scene;
mod vector;

use canvas::Camera;
use canvas::Canvas;
use scene::Scene;

use crate::config::CameraConfig;
use crate::config::SceneConfig;
use crate::image::RasterImage;

pub struct Raytracer {
    camera: Camera,
    scene: Scene,
}

impl Raytracer {
    pub fn new(camera: CameraConfig, scene: SceneConfig) -> Raytracer {
        Raytracer {
            camera: Camera::new(camera),
            scene: Scene::new(scene),
        }
    }

    pub fn fill_image(self, image: &mut RasterImage) {
        let (width, height) = image.get_resolution();
        let resolution = std::cmp::max(width, height);
        let mut convas = Canvas::new(self.camera, self.scene, resolution + 1);

        let (h_shift, w_shift) = if height < width {
            ((resolution - height) / 2, 0)
        } else {
            (0, (resolution - width) / 2)
        };

        // TODO make parallel
        convas.fill_canvas(
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
}
