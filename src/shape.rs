use crate::{Intersection, Material, Matrix, Point, Ray, Vector};

pub trait Shape: Default {
    #[must_use]
    fn get_transform(&self) -> &Matrix;
    fn set_transform(&mut self, transform: Matrix);

    #[must_use]
    fn get_material(&self) -> &Material;
    fn set_material(&mut self, material: Material);

    fn local_normal_at(&self, point: Point) -> Vector;

    #[must_use]
    fn normal_at(&self, point: Point) -> Vector {
        let object_point = self.get_transform().inverse() * point;
        let object_normal = self.local_normal_at(object_point);
        let world_normal = self.get_transform().inverse().transpose() * object_normal;
        world_normal.normalize()
    }

    #[must_use]
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;

    #[must_use]
    fn new(transform: Matrix, material: Material) -> Self {
        let mut shape = Self::default();
        shape.set_transform(transform);
        shape.set_material(material);

        shape
    }
}

#[cfg(test)]
pub(crate) mod testshape {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TestShape {
        pub transform: Matrix,
        pub material: Material,
        pub test_ray: Ray,
    }

    impl Shape for TestShape {
        fn get_transform(&self) -> &Matrix {
            &self.transform
        }

        fn set_transform(&mut self, transform: Matrix) {
            self.transform = transform;
        }

        fn get_material(&self) -> &Material {
            &self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }

        fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
            assert_eq!(ray, &self.test_ray);
            Vec::new()
        }

        fn local_normal_at(&self, point: Point) -> Vector {
            point - Point::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal;
    use crate::Color;
    use std::f64::consts::PI;

    #[derive(Debug, Default)]
    struct TestShape {
        pub transform: Matrix,
        pub material: Material,
        pub test_ray: Ray,
    }

    impl Shape for TestShape {
        fn get_transform(&self) -> &Matrix {
            &self.transform
        }

        fn set_transform(&mut self, transform: Matrix) {
            self.transform = transform;
        }

        fn get_material(&self) -> &Material {
            &self.material
        }

        fn set_material(&mut self, material: Material) {
            self.material = material;
        }

        fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
            assert_eq!(ray, &self.test_ray);
            Vec::new()
        }

        fn local_normal_at(&self, point: Point) -> Vector {
            point - Point::default()
        }
    }

    #[test]
    fn new_test_shape() {
        let s = TestShape::default();
        assert_eq!(s.transform, Matrix::default());
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn shapes_have_transforms() {
        let mut s = TestShape::default();
        assert_eq!(s.get_transform(), &Matrix::default());

        s.set_transform(Matrix::rotation_y(2.0).inverse());
        assert_eq!(s.get_transform(), &Matrix::rotation_y(-2.0));
    }

    #[test]
    fn shapes_have_materials() {
        let mut s = TestShape::default();
        assert_eq!(s.get_material(), &Material::default());

        s.set_material(Material::new(Color::black(), 0.0, 0.5, 1.0, 50.0));
        assert_eq!(s.get_material().color, Color::black());
        assert_eq!(s.get_material().diffuse, 0.5);
    }

    #[test]
    fn translated_normals() {
        let s = TestShape::new(
            Matrix::translation(Vector::new(0.0, 1.0, 0.0)),
            Material::default(),
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 1.70711, -0.70711)),
            Vector::new(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn transformed_normals() {
        let s = TestShape::new(
            Matrix::scaling(Vector::new(1.0, 0.5, 1.0)) * Matrix::rotation_z(PI / 5.0),
            Material::default(),
        );

        assert_eq!(
            s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / -2.0)),
            Vector::new(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn normalized_normals() {
        let s = TestShape::default();

        assert!(equal(
            s.normal_at(Point::new(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            ))
            .magnitude(),
            1.0,
        ));
    }
}
