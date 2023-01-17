use super::ray;
use super::scene;
use super::vector;
use crate::image::Color;

struct Camera {
    orig: vector::Vector,
    view: vector::Vector,
    tau: vector::Vector,
    up: vector::Vector,
}

impl Camera {
    fn new(distance: f32) -> Camera {
        Camera {
            orig: vector::Vector::new(0.0, 0.0, distance),
            view: vector::Vector::new(0.0, 0.0, -1.0),
            tau: vector::Vector::new(1.0, 0.0, 0.0),
            up: vector::Vector::new(0.0, 1.0, 0.0),
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
    pub fn new(distance: f32, resolution: usize) -> Canvas {
        Canvas {
            camera: Camera::new(distance),
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
