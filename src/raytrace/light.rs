use super::vector::Vector;

pub struct Light {
    orig: Vector,
}

impl Light {
    pub fn new(orig: Vector) -> Light {
        Light { orig }
    }

    pub fn intensity(&self, intersec_point: &Vector, norm: &Vector) -> f32 {
        let mut l = &self.orig - intersec_point;
        l.norm();

        let intensity = l.dot(norm);

        if intensity < 0.0 {
            return 0.0;
        }
        intensity
    }
}
