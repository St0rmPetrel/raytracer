use super::ray::Ray;
use super::vector::Vector;

pub enum Intersec {
    OneRoot(f32),
    TwoRoot(f32, f32),
    None,
}

pub trait Shape {
    fn intersec(&self, ray: &Ray) -> Intersec;
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
            return Intersec::OneRoot(oc_dir);
        }
        let k = k.sqrt();

        Intersec::TwoRoot(oc_dir - k, oc_dir + k)
    }

    fn norm(&self, point: &Vector) -> Vector {
        let mut norm = point - &self.center;
        norm.norm();

        norm
    }
}
