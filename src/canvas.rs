use crate::config::CameraConfig;
use crate::image::Color;
use crate::ray;
use crate::scene;
use crate::vector::Vector;

struct Camera {
    orig: Vector,
    view: Vector,
    tau: Vector,
    up: Vector,
}

impl Camera {
    fn new(conf: &CameraConfig) -> Camera {
        let mut view = Vector::new_from_arr(&conf.view);
        view.norm();
        let mut up = Vector::new_from_arr(&conf.up);
        up.norm();
        let mut tau = view.cross(&up);
        tau.norm();
        let mut up = tau.cross(&view);
        up.norm();
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
    step: f32,
    resolution: usize,
    canvas: Vec<Color>,
}

impl Canvas {
    pub fn new(conf: &CameraConfig, resolution: usize) -> Canvas {
        Canvas {
            camera: Camera::new(conf),
            resolution,
            step: 1.0 / resolution as f32,
            canvas: vec![Color::new(0, 0, 0); resolution * resolution],
        }
    }

    pub fn get_canvas_pixel(&self, i: usize, j: usize) -> Option<&Color> {
        let index = self.resolution * j + i;
        self.canvas.get(index)
    }

    pub fn fill_canvas(
        &mut self,
        scene: scene::Scene,
        i_bound: (usize, usize),
        j_bound: (usize, usize),
    ) {
        for j in j_bound.0..j_bound.1 {
            for i in i_bound.0..i_bound.1 {
                let ray = self.get_ray(i, j);
                let ray_color = scene.get_ray_color(&ray);
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

    fn get_ray(&self, i: usize, j: usize) -> ray::Ray {
        let mut dir = self.camera.view.clone();
        let h_shift = -0.5 + (i as f32 * self.step + self.step * 0.5);
        let v_shift = 0.5 - (j as f32 * self.step + self.step * 0.5);

        dir += &self.camera.tau * h_shift;
        dir += &self.camera.up * v_shift;

        dir.norm();

        ray::Ray::new(self.camera.orig.clone(), dir)
    }
}
