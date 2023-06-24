use crate::config::CameraConfig;
use crate::image::Color;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::vector::Vector;

pub struct Camera {
    orig: Vector,
    view: Vector,
    tau: Vector,
    up: Vector,
}

impl Camera {
    pub fn new(conf: CameraConfig) -> Camera {
        let view = Vector::new_from_arr(&conf.view).norm();
        let up = Vector::new_from_arr(&conf.up).norm();
        let tau = view.cross(&up).norm();
        let up = tau.cross(&view).norm();
        Camera {
            orig: Vector::new_from_arr(&conf.origin),
            view,
            tau,
            up,
        }
    }
}

pub struct Canvas {
    camera: Camera,
    scene: Scene,
    step: f32,
    resolution: usize,
    canvas: Vec<Color>,
}

impl Canvas {
    pub fn new(camera: Camera, scene: Scene, resolution: usize) -> Canvas {
        Canvas {
            camera,
            scene,
            resolution,
            step: 1.0 / resolution as f32,
            canvas: vec![Color::new(0, 0, 0); resolution * resolution],
        }
    }

    pub fn get_canvas_pixel(&self, i: usize, j: usize) -> Option<&Color> {
        let index = self.resolution * j + i;
        self.canvas.get(index)
    }

    pub fn fill_canvas(&mut self, i_bound: (usize, usize), j_bound: (usize, usize)) {
        for j in j_bound.0..j_bound.1 {
            for i in i_bound.0..i_bound.1 {
                let ray = self.get_ray(i, j);
                let ray_color = self.scene.get_ray_color(&ray, 0);
                let pixel = match self.get_canvas_pixel_mut(i, j) {
                    Some(pixel) => pixel,
                    None => continue,
                };
                pixel.set(&ray_color);
            }
        }
    }

    fn get_canvas_pixel_mut(&mut self, i: usize, j: usize) -> Option<&mut Color> {
        let index = self.resolution * j + i;
        self.canvas.get_mut(index)
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let mut dir = self.camera.view.clone();
        let h_shift = -0.5 + (i as f32 * self.step + self.step * 0.5);
        let v_shift = 0.5 - (j as f32 * self.step + self.step * 0.5);

        dir += &(&self.camera.tau * h_shift);
        dir += &(&self.camera.up * v_shift);

        Ray::new(self.camera.orig.clone(), dir.norm())
    }
}
