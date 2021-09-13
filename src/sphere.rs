use crate::{Material, Matrix, Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    #[must_use]
    pub fn new(transform: Matrix, material: Material) -> Self {
        let mut s = Self::default();
        s.set_transform(transform);
        s.set_material(material);
        s
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    #[must_use]
    pub fn normal_at(&self, point: Point) -> Vector {
        let object_point = self.transform.inverse() * point;
        let object_normal = (object_point - Point::default()).normalize();
        let world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix::eye(4),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Color;

    use super::*;
    use std::f64::consts::PI;

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
        let s = Sphere::new(t, Material::default());

        assert_eq!(s.transform, Matrix::translation(Vector::new(2.0, 3.0, 4.0)));
    }

    #[test]
    fn sphere_normals() {
        let s = Sphere::default();

        assert_eq!(
            s.normal_at(Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            s.normal_at(Point::new(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            )),
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn normalized_normals() {
        let s = Sphere::default();

        assert_eq!(
            s.normal_at(Point::new(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            )),
            s.normal_at(Point::new(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            ))
            .normalize(),
        );
    }

    #[test]
    fn translated_sphere_normals() {
        let s = Sphere::new(
            Matrix::translation(Vector::new(0.0, 1.0, 0.0)),
            Material::default(),
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 1.70711, -0.70711)),
            Vector::new(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn transformed_sphere_normals() {
        let s = Sphere::new(
            Matrix::scaling(Vector::new(1.0, 0.5, 1.0)) * Matrix::rotation_z(PI / 5.0),
            Material::default(),
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / -2.0)),
            Vector::new(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn sphere_material() {
        let s1 = Sphere::default();
        assert_eq!(s1.material, Material::default());

        let m = Material::new(Color::new(0.3, 0.6, 0.8), 0.2, 0.5, 0.6, 20.0);
        let s2 = Sphere::new(Matrix::default(), m);

        assert_eq!(
            s2.material,
            Material::new(Color::new(0.3, 0.6, 0.8), 0.2, 0.5, 0.6, 20.0)
        );
    }
}
