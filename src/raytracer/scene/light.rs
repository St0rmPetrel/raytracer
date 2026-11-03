use crate::raytracer::{radiance::Radiance, vector::Vector};

pub struct Light {
    orig: Vector,
    power: f32,
    color: Radiance,
}

impl Light {
    pub fn new(orig: Vector, power: f32, color: Radiance) -> Self {
        Self { orig, power, color }
    }
    pub fn incoming(&self, distance_squared: f32) -> Radiance {
        self.color.scale(self.power / (1.0 + distance_squared))
    }

    pub fn get_orig(&self) -> &Vector {
        &self.orig
    }
}
