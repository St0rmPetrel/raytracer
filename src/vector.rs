use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn new_from_arr(xyz: &[f32; 3]) -> Vector {
        Vector {
            x: xyz[0],
            y: xyz[1],
            z: xyz[2],
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn dot2(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn size(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn norm(&mut self) {
        let size = self.size();
        self.x = self.x / size;
        self.y = self.y / size;
        self.z = self.z / size;
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

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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

impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x = {}, y = {}, z = {})", self.x, self.y, self.z)
    }
}
