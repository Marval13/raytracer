use crate::{Intersection, Matrix, Point, Shape, Vector};

#[derive(Debug, Default, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    #[must_use]
    pub fn transform(&self, transformation: &Matrix) -> Self {
        Self {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }

    #[must_use]
    pub fn intersect<T: Shape>(&self, shape: &T) -> Vec<Intersection> {
        let ray = self.transform(&shape.get_transform().inverse());
        shape.local_intersect(&ray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::testshape::TestShape;
    use crate::vector;

    #[test]
    fn ray_translate() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translation(Vector::new(3.0, 4.0, 5.0));
        let rt = r.transform(&m);

        assert_eq!(rt.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(rt.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_scale() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scaling(Vector::new(2.0, 3.0, 4.0));
        let rt = r.transform(&m);

        assert_eq!(rt.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(rt.direction, Vector::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn intersect_right() {
        let s = TestShape::default();
        let _ = s.local_intersect(&Ray::default());
    }

    #[test]
    #[should_panic]
    fn intersect_wrong() {
        let s = TestShape::default();
        let _ = s.local_intersect(&Ray::new(Point::default(), -vector::Z));
    }
}
