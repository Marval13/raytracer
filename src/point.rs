use crate::utils::equal;
use crate::Vector;

use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub static UX: Point = Point {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub static UY: Point = Point {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub static UZ: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl Point {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        equal(self.x, other.x) && equal(self.y, other.y) && equal(self.z, other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point() {
        let p = Point::new(4.0, -4.0, 3.5);
        assert!(equal(p.x, 4.0));
        assert!(equal(p.y, -4.0));
        assert!(equal(p.z, 3.5));
    }

    #[test]
    fn point_add() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn point_sub_point() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn point_sub_vector() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }
}
