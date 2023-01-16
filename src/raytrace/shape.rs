use super::ray::Ray;
use super::vector::Vector;

pub enum Intersec {
    OneRoot(f32),
    TwoRoot(f32, f32),
    None,
}

pub trait Shape {
    fn intersec(&self, ray: &Ray) -> Option<f32>;
    fn norm(&self, point: &Vector) -> Vector;
}

pub struct Sphere {
    center: Vector,
    radius2: f32,
}

pub fn new(center: Vector, radius: f32) -> impl Shape {
    Sphere {
        center,
        radius2: radius * radius,
    }
}

impl Shape for Sphere {
    fn intersec(&self, ray: &Ray) -> Option<f32> {
        let oc = &self.center - ray.get_orig();
        let oc_dir = oc.dot(ray.get_dir());

        if oc_dir <= 0.0 {
            return None;
        }

        let h2 = oc.cross(ray.get_dir()).dot2();

        let k = self.radius2 - h2;
        if k < 0.0 {
            return None;
        }

        if k == 0.0 {
            return Some(oc_dir);
        }
        let k = k.sqrt();

        let (x1, x2) = (oc_dir - k, oc_dir + k);

        if x1 < 0.0 && x2 < 0.0 {
            None
        } else if x1 < 0.0 {
            Some(x2)
        } else if x2 < 0.0 {
            Some(x1)
        } else {
            if x1 < x2 {
                Some(x1)
            } else {
                Some(x2)
            }
        }
    }

    fn norm(&self, point: &Vector) -> Vector {
        let mut norm = point - &self.center;
        norm.norm();

        norm
    }
}
