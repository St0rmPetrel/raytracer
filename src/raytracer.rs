mod canvas;
mod radiance;
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

    pub(crate) fn into_rows(self, width: usize, height: usize, samples: usize) -> RowRenderer {
        let resolution = width.max(height);
        RowRenderer {
            canvas: Canvas::new(self.camera, self.scene, resolution),
            width,
            x_shift: (resolution - width) / 2,
            y_shift: (resolution - height) / 2,
            samples,
        }
    }

    pub fn fill_image(self, image: &mut RasterImage) {
        let (width, height) = image.get_resolution();
        let mut renderer = self.into_rows(width, height, image.samples());
        let rgba = renderer.render_rows(0, height);
        for (index, pixel) in rgba.as_chunks::<4>().0.iter().enumerate() {
            image
                .get_pixel(index % width, index / width)
                .unwrap()
                .set(&crate::image::Color::new(pixel[0], pixel[1], pixel[2]));
        }
    }
}

pub(crate) struct RowRenderer {
    canvas: Canvas,
    width: usize,
    x_shift: usize,
    y_shift: usize,
    samples: usize,
}

impl RowRenderer {
    pub(crate) fn render_rows(&mut self, start: usize, end: usize) -> Vec<u8> {
        self.canvas.fill_canvas(
            (self.x_shift, self.x_shift + self.width),
            (self.y_shift + start, self.y_shift + end),
            self.samples,
        );
        let mut rgba = Vec::with_capacity((end - start) * self.width * 4);
        for y in start..end {
            for x in 0..self.width {
                let color = self
                    .canvas
                    .get_canvas_pixel(x + self.x_shift, y + self.y_shift)
                    .unwrap();
                rgba.extend_from_slice(&[color.r, color.g, color.b, 255]);
            }
        }
        rgba
    }
}
