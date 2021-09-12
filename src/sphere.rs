use crate::Matrix;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    #[must_use]
    pub fn new(t: Matrix) -> Self {
        let mut s = Self::default();
        s.set_transform(t);
        s
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix::eye(4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    #[test]
    fn new_transform() {
        let mut s = Sphere::default();
        let t = Matrix::translation(Vector::new(2.0, 3.0, 4.0));
        s.set_transform(t);

        assert_eq!(s.transform, Matrix::translation(Vector::new(2.0, 3.0, 4.0)));
    }

    #[test]
    fn new_sphere() {
        let t = Matrix::translation(Vector::new(2.0, 3.0, 4.0));
        let s = Sphere::new(t);

        assert_eq!(s.transform, Matrix::translation(Vector::new(2.0, 3.0, 4.0)));
    }
}
