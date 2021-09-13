use crate::utils::equal;

use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    #[must_use]
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    #[must_use]
    pub fn white() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.r, other.r) && equal(self.g, other.g) && equal(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_color() {
        let c = Color::new(0.3, 0.4, 0.5);
        assert!(equal(c.r, 0.3));
        assert!(equal(c.g, 0.4));
        assert!(equal(c.b, 0.5));
    }

    #[test]
    fn color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_mul_scalar() {
        assert_eq!(Color::new(0.2, 0.3, 0.4) * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn color_mul() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
