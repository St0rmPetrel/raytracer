use crate::raytracer::vector::Vector;

pub struct Ray {
    orig: Vector,
    dir: Vector,
}

impl Ray {
    pub fn new(orig: Vector, dir: Vector) -> Ray {
        Ray { orig, dir }
    }

    pub fn new_reflect(&self, p: &Vector, n: &Vector) -> Option<Ray> {
        let d = match self.get_dir().reflect(n) {
            None => return None,
            Some(d) => d,
        };
        Some(Ray {
            orig: p.clone(),
            dir: d,
        })
    }

    pub fn get_orig(&self) -> &Vector {
        &self.orig
    }
    pub fn get_dir(&self) -> &Vector {
        &self.dir
    }

    pub fn point_on_ray(&self, t: f32) -> Vector {
        &(&self.dir * t) + &self.orig
    }
}
