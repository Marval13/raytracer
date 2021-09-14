use crate::utils::equal;

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub static X: Vector = Vector {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub static Y: Vector = Vector {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub static Z: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Vector {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    #[must_use]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[must_use]
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[must_use]
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * 2.0 * self.dot(normal)
    }
}

impl Default for Vector {
    fn default() -> Self {
        X
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector() {
        let p = Vector::new(4.0, -4.0, 3.5);
        assert!(equal(p.x, 4.0));
        assert!(equal(p.y, -4.0));
        assert!(equal(p.z, 3.5));
    }

    #[test]
    fn vector_sub() {
        let p1 = Vector::new(3.0, 2.0, 1.0);
        let p2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn vector_neg() {
        assert_eq!(-Vector::new(1.0, -2.0, 3.0), Vector::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn vector_mul() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0) * 3.5,
            Vector::new(3.5, -7.0, 10.5),
        );
    }

    #[test]
    fn vector_div() {
        assert_eq!(
            Vector::new(1.0, -2.0, 3.0) / 2.0,
            Vector::new(0.5, -1.0, 1.5),
        );
    }

    #[test]
    fn vector_magnitude() {
        assert!(equal(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0));

        assert!(equal(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0));

        assert!(equal(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0));

        assert!(equal(Vector::new(1.0, 2.0, 3.0).magnitude(), 14_f64.sqrt()));

        assert!(equal(
            Vector::new(1.0, -2.0, 3.0).magnitude(),
            14_f64.sqrt(),
        ));
    }

    #[test]
    fn vector_normalize() {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).normalize(),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            Vector::new(1.0, 2.0, 3.0).normalize(),
            Vector::new(0.26726, 0.53452, 0.80178),
        );

        assert!(equal(
            Vector::new(1.0, 2.0, 3.0).normalize().magnitude(),
            1.0,
        ));
    }

    #[test]
    fn vector_dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert!(equal(v1.dot(&v2), 20.0));
    }

    #[test]
    fn vector_cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn vector_reflect() {
        let normal1 = Vector::new(0.0, 1.0, 0.0);
        let normal2 = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);

        assert_eq!(
            Vector::new(1.0, -1.0, 0.0).reflect(&normal1),
            Vector::new(1.0, 1.0, 0.0),
        );

        assert_eq!(
            Vector::new(0.0, -1.0, 0.0).reflect(&normal2),
            Vector::new(1.0, 0.0, 0.0),
        );
    }
}
