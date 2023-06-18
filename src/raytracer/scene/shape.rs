use crate::raytracer::ray::Ray;
use crate::raytracer::vector::Vector;

pub trait Shape {
    fn intersec(&self, ray: &Ray) -> Intersec;
    fn norm(&self, point: &Vector) -> Vector;
}

#[derive(Debug)]
pub enum Intersec {
    One(f32),
    Two(f32, f32),
    None,
}

impl Intersec {
    pub fn get_closer(self) -> Option<f32> {
        match self {
            Self::None => None,
            Self::One(d) => Some(d),
            Self::Two(d1, d2) => {
                if d1 > 0.0 && d2 > 0.0 {
                    Some(f32::min(d1, d2))
                } else {
                    None
                }
            }
        }
    }
}

pub struct Sphere {
    center: Vector,
    radius2: f32,
}

pub fn new_sphere(center: Vector, radius: f32) -> impl Shape {
    Sphere {
        center,
        radius2: radius * radius,
    }
}

impl Shape for Sphere {
    fn intersec(&self, ray: &Ray) -> Intersec {
        let oc = &self.center - ray.get_orig();
        let oc_dir = oc.dot(ray.get_dir());

        if oc_dir <= 0.0 {
            return Intersec::None;
        }

        let h2 = oc.cross(ray.get_dir()).dot2();

        let k = self.radius2 - h2;
        if k < 0.0 {
            return Intersec::None;
        }

        if k == 0.0 {
            return Intersec::One(oc_dir);
        }
        let k = k.sqrt();

        Intersec::Two(oc_dir - k, oc_dir + k)
    }

    fn norm(&self, point: &Vector) -> Vector {
        (point - &self.center).norm()
    }
}
