use rand;
use std::ops::{Add, AddAssign, Mul, Sub};

/// Vector in 3 dimension Euclidean space.
#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    /// Return new Vector (Point in Euclidean space) with given coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    /// Return new Vector with random direction and random size in [0, 1].
    ///
    /// # Example
    /// ```rust
    /// // create Vector r with r.size() <= 1.0 and random direction
    /// let r = Vector::new_rand();
    /// ```
    ///
    /// # Use case
    /// In this project random Vectors used for diffusion by change direction of ideal shape's
    /// normal Vectors:
    /// ```rust
    /// let normal_with_diffusion = (normal_ideal + Vector::new_rand() * diffusion_coefficient).norm();
    /// ```
    pub fn new_rand() -> Vector {
        (Vector {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
        .norm())
            * rand::random()
    }

    /// Return new Vector with given coordinates from array.
    ///
    /// In this project it's used for easy creation Vector from configuration structure where
    /// coordinates stored in array
    ///
    /// #Example
    /// ```rust
    /// let conf_origin: [f32; 3] = [1.0, -1.0, 0.0];
    /// let origin = Vector::new_from_arr(&conf_origin)
    /// ```
    pub fn new_from_arr(xyz: &[f32; 3]) -> Vector {
        Vector {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }

    /// Return scalar or dot product of two Vectors (scalar)
    ///
    /// #Example
    /// ```rust
    /// let a = Vector::new(0, 1, 2);
    /// let b = Vector::new(2, 1, 0);
    ///
    /// // p = a dot b
    /// // p == 0 * 2 + 1 * 1 + 2 * 0 == 1
    /// // p == a.size() * b.size() * cos(a, b)
    /// let p = a.dot(&b);
    /// assert!(p, 1);
    /// ```
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Return dot product of Vector by itself (scalar)
    ///
    /// #Example
    /// ```rust
    /// let a = Vector::new(0, 1, 2);
    ///
    /// // p = a dot a
    /// // p == 0 * 0 + 1 * 1 + 2 * 2 == 5
    /// // p == (a.size()).pow(2)
    /// let p = a.dot2();
    /// assert!(p, 5);
    /// ```
    pub fn dot2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Return cross product of two Vectors (Vector)
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Return size (length) of the Vector
    pub fn size(&self) -> f32 {
        self.dot(self).sqrt()
    }

    /// Normalize Vector (change size of Vector to make it's size = 1.0)
    pub fn norm(mut self) -> Vector {
        let size = self.size();
        self.x /= size;
        self.y /= size;
        self.z /= size;
        self
    }

    /// Assumed that n.size() == 1.0
    pub fn reflect(&self, n: &Vector) -> Option<Vector> {
        let dot = self.dot(n);
        if dot >= 0.0 {
            return None;
        }
        let n = (-2.0 * dot) * n;
        let r = self.clone() + &n;

        Some(r)
    }

    /// Step away from point (Vector) along the normal need for
    /// case in which shadow ray or reflection ray may intersect self object (the object from which
    /// it was created). So the origin (point) of new ray (reflection, shadow or other) created
    /// a little step away from real object intersection origin.
    ///
    /// # Example
    /// ```rust
    /// // ray = Ray::new(orig, dir)
    /// let ray = Ray::new(Vector::new(0, 0, 0), Vector::new(0, 0, 1));
    /// let sphere_intersec_point = Vector::new(0, 0, 1);
    /// let sphere_intersec_norm = Vector::new(0, 1, 0);
    ///
    /// // origin of refl_ray is step away from sphere to don't intersect that sphere
    /// let refl_ray = Ray::new(
    ///     sphere_intersec_point.step_away(&sphere_intersec_norm),
    ///     ray.dir,
    /// );
    /// ```
    pub fn step_away(&self, n: &Vector) -> Vector {
        const STEP_FROM_ORIGIN: f32 = 100.0 * f32::EPSILON;
        self + &(STEP_FROM_ORIGIN * n)
    }
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(mut self, rhs: &Vector) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul<f32> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        let _ = self.x * rhs;
        let _ = self.y * rhs;
        let _ = self.z * rhs;
        self
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        let _ = rhs.x * self;
        let _ = rhs.y * self;
        let _ = rhs.z * self;
        rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::vector::Vector;

    #[test]
    fn reflect_positive_1() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        let r = v.reflect(&n).unwrap();
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }
}
