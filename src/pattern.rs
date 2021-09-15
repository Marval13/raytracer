#![allow(clippy::module_name_repetitions)]

use crate::{Color, Matrix, Object, Point, Shape};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    None,
    Stripe(StripePattern),
}

impl Pattern {
    #[must_use]
    pub fn color_at(&self, point: Point) -> Color {
        match self {
            Pattern::None => panic!(),
            Pattern::Stripe(pattern) => pattern.stripe_at(point),
        }
    }

    #[must_use]
    pub fn color_at_object(&self, object: &Object, point: Point) -> Color {
        match self {
            Pattern::None => panic!(),
            Pattern::Stripe(pattern) => pattern.stripe_at_object(object, point),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Pattern::None => {},
            Pattern::Stripe(pattern) => pattern.set_transform(transform),
        }
    }

    #[must_use]
    pub fn get_transform(&mut self) -> Matrix {
        match self {
            Pattern::None => Matrix::default(),
            Pattern::Stripe(pattern) => pattern.get_transform(),
        }
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

    #[must_use]
    pub fn stripe_at(&self, point: Point) -> Color {
        #[allow(clippy::cast_possible_truncation)]
        if point.x.floor() as isize % 2 == 0 {
            self.color1
        } else {
            self.color2
        }
    }

    #[must_use]
    pub fn stripe_at_object(&self, object: &Object, point: Point) -> Color {
        let object_point = object.get_transform().inverse() * point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.stripe_at(pattern_point)
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    #[must_use]
    pub fn get_transform(&self) -> Matrix {
        self.transform
    }
}

#[cfg(test)]
mod tests {
    use crate::{Shape, Sphere, Vector};

    use super::*;

    #[test]
    fn new_striped_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());
        assert_eq!(pattern.color1, Color::white());
        assert_eq!(pattern.color2, Color::black());
    }

    #[test]
    fn stripe_at_x() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(
            pattern.stripe_at(Point::new(-0.1, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.stripe_at(Point::new(-1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.stripe_at(Point::new(-1.1, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_at_y() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_at_z() {
        let pattern = StripePattern::default();
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.stripe_at(Point::new(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_with_object_transform() {
        let mut s = Object::Sphere(Sphere::default());
        let pattern = StripePattern::default();
        s.set_transform(Matrix::scaling(Vector::new(2.0, 2.0, 2.0)));

        assert_eq!(
            pattern.stripe_at_object(&s, Point::new(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_with_pattern_transform() {
        let s = Object::Sphere(Sphere::default());
        let mut pattern = StripePattern::default();
        pattern.set_transform(Matrix::scaling(Vector::new(2.0, 2.0, 2.0)));

        assert_eq!(
            pattern.stripe_at_object(&s, Point::new(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_with_pattern_and_object_transform() {
        let mut s = Object::Sphere(Sphere::default());
        let mut pattern = StripePattern::default();
        s.set_transform(Matrix::scaling(Vector::new(2.0, 2.0, 2.0)));
        pattern.set_transform(Matrix::translation(Vector::new(0.5, 0.0, 0.0)));

        assert_eq!(
            pattern.stripe_at_object(&s, Point::new(2.5, 0.0, 0.0)),
            Color::white()
        );
    }
}
