use super::ray;
use super::vector;

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
}

impl Canvas {
    pub fn new(distance: f32, resolution: usize) -> Canvas {
        Canvas {
            camera: Camera::new(distance),
            step: 1.0 / resolution as f32,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> ray::Ray {
        let mut dir = self.camera.view.clone();
        let h_shift = -0.5 + (i as f32 * self.step + self.step * 0.5);
        let v_shift = 0.5 - (j as f32 * self.step + self.step * 0.5);

        dir += &self.camera.tau * h_shift;
        dir += &self.camera.up * v_shift;

        dir.norm();

        ray::Ray::new(self.camera.orig.clone(), dir)
    }
}
