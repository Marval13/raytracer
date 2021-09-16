#![allow(clippy::module_name_repetitions)]

use crate::transformations::Transformable;
use crate::{Color, Matrix, Object, Point};

pub trait Patterned: Transformable {
    #[must_use]
    fn color_at(&self, point: Point) -> Color;

    #[must_use]
    fn color_at_object(&self, object: &Object, point: Point) -> Color {
        let object_point = object.get_transform().inverse() * point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.color_at(pattern_point)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    None,
    Stripe(StripePattern),
}

impl Transformable for Pattern {
    #[must_use]
    fn get_transform(&self) -> Matrix {
        match self {
            Pattern::None => Matrix::default(),
            Pattern::Stripe(pattern) => pattern.get_transform(),
        }
    }

    fn set_transform(&mut self, transform: Matrix) {
        match self {
            Pattern::None => {}
            Pattern::Stripe(pattern) => pattern.set_transform(transform),
        }
    }
}

impl Patterned for Pattern {
    #[must_use]
    fn color_at(&self, point: Point) -> Color {
        match self {
            Pattern::None => panic!(),
            Pattern::Stripe(pattern) => pattern.color_at(point),
        }
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripePattern {
    pub color1: Color,
    pub color2: Color,
    pub transform: Matrix,
}

impl Default for StripePattern {
    fn default() -> Self {
        Self::new(Color::white(), Color::black())
    }
}

impl StripePattern {
    #[must_use]
    pub fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::default(),
        }
    }
}

impl Transformable for StripePattern {
    #[must_use]
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Patterned for StripePattern {
    #[must_use]
    fn color_at(&self, point: Point) -> Color {
        #[allow(clippy::cast_possible_truncation)]
        if point.x.floor() as isize % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientPattern {
    pub color1: Color,
    pub color2: Color,
    pub transform: Matrix,
}

impl Default for GradientPattern {
    fn default() -> Self {
        Self::new(Color::white(), Color::black())
    }
}

impl GradientPattern {
    #[must_use]
    pub fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::default(),
        }
    }
}

impl Transformable for GradientPattern {
    #[must_use]
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Patterned for GradientPattern {
    #[must_use]
    fn color_at(&self, point: Point) -> Color {
        self.color1 + (self.color2 - self.color1) * point.x.fract()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingPattern {
    pub color1: Color,
    pub color2: Color,
    pub transform: Matrix,
}

impl Default for RingPattern {
    fn default() -> Self {
        Self::new(Color::white(), Color::black())
    }
}

impl RingPattern {
    #[must_use]
    pub fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::default(),
        }
    }
}

impl Transformable for RingPattern {
    #[must_use]
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Patterned for RingPattern {
    #[must_use]
    fn color_at(&self, point: Point) -> Color {
        #[allow(clippy::cast_possible_truncation)]
        if (point.x * point.x + point.z * point.z).sqrt().floor() as isize % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckerPattern {
    pub color1: Color,
    pub color2: Color,
    pub transform: Matrix,
}

impl Default for CheckerPattern {
    fn default() -> Self {
        Self::new(Color::white(), Color::black())
    }
}

impl CheckerPattern {
    #[must_use]
    pub fn new(color1: Color, color2: Color) -> Self {
        Self {
            color1,
            color2,
            transform: Matrix::default(),
        }
    }
}

impl Transformable for CheckerPattern {
    #[must_use]
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Patterned for CheckerPattern {
    #[must_use]
    fn color_at(&self, point: Point) -> Color {
        #[allow(clippy::cast_possible_truncation)]
        if (point.x.floor() as isize + point.y.floor() as isize + point.z.floor() as isize) % 2 == 0
        {
            self.color1
        } else {
            self.color2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_striped_pattern() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.color1, Color::white());
        assert_eq!(pattern.color2, Color::black());
    }

    #[test]
    fn stripe_at_x() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_at_y() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_at_z() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn default_gradient_pattern() {
        let pattern = GradientPattern::default();
        assert_eq!(pattern.color1, Color::white());
        assert_eq!(pattern.color2, Color::black());
    }

    #[test]
    fn gradient_at_x() {
        let pattern = GradientPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(
            pattern.color_at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn gradient_at_y() {
        let pattern = GradientPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn gradient_at_z() {
        let pattern = GradientPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn default_ring_pattern() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.color1, Color::white());
        assert_eq!(pattern.color2, Color::black());
    }

    #[test]
    fn ring_at_x() {
        let pattern = RingPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-0.1, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-1.1, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn ring_at_y() {
        let pattern = RingPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn ring_at_z() {
        let pattern = RingPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.9)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, -0.1)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, -1.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, -1.1)), Color::black());
    }

    #[test]
    fn checker_at() {
        let pattern = CheckerPattern::default();
        assert_eq!(pattern.color_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, 0.9, 0.9)), Color::white());
        assert_eq!(pattern.color_at(Point::new(1.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.color_at(Point::new(0.0, -0.1, 0.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-1.0, -1.0, -1.0)), Color::black());
        assert_eq!(pattern.color_at(Point::new(-1.1, -1.1, 0.0)), Color::white());
    }
}
