use super::vector::Vector;

pub struct Ray {
    orig: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(orig: Vector, dir: Vector) -> Ray {
        Ray { orig, dir }
    }

    pub fn is_sphere_intersec(&self, center: Vector, radius: f32) -> bool {
        let oc = &center - &self.orig;
        let oc_dir = oc.dot(&self.dir);

        if oc_dir <= 0.0 {
            return false;
        }

        let h2 = oc.cross(&self.dir).dot2();

        if h2 > radius * radius {
            return false;
        }

        true
    }
}
