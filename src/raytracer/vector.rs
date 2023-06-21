use rand;
use std::ops::{Add, AddAssign, Mul, Sub};

/// Vector in 3 dimension Euclidean space.
#[derive(Clone, Debug)]
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
        self.x = self.x / size;
        self.y = self.y / size;
        self.z = self.z / size;
        self
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
