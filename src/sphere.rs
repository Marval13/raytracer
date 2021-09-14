use crate::{Intersection, Material, Matrix, Object, Point, Ray, Shape, Vector};

#[derive(Debug, PartialEq, Clone, Copy)]
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
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            transform: Matrix::eye(4),
            material: Material::default(),
        }
    }
}

impl Shape for Sphere {
    fn get_transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Vec::new()
        } else {
            vec![
                Intersection::new(
                    (-b - discriminant.sqrt()) / (2.0 * a),
                    &Object::Sphere(*self),
                ),
                Intersection::new(
                    (-b + discriminant.sqrt()) / (2.0 * a),
                    &Object::Sphere(*self),
                ),
            ]
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        (point - Point::default()).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal;

    #[test]
    fn normals() {
        let s = Sphere::default();

        assert_eq!(
            s.local_normal_at(Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            s.local_normal_at(Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            s.local_normal_at(Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            s.local_normal_at(Point::new(
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0,
                3_f64.sqrt() / 3.0
            )),
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
        );
    }

    #[test]
    fn intersect_2_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = s.local_intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, 4.0));
        assert!(equal(intersections[1].t, 6.0));
        assert_eq!(intersections[0].object, Object::Sphere(s));
        assert_eq!(intersections[1].object, Object::Sphere(s));
    }

    #[test]
    fn intersect_sphere_1_point() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = s.local_intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, 5.0));
        assert!(equal(intersections[1].t, 5.0));
        assert_eq!(intersections[0].object, Object::Sphere(s));
        assert_eq!(intersections[1].object, Object::Sphere(s));
    }

    #[test]
    fn intersect_0_points() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = s.local_intersect(&r);

        assert!(intersections.is_empty());
    }

    #[test]
    fn intersect_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = s.local_intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, -1.0));
        assert!(equal(intersections[1].t, 1.0));
        assert_eq!(intersections[0].object, Object::Sphere(s));
        assert_eq!(intersections[1].object, Object::Sphere(s));
    }

    #[test]
    fn intersect_behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = s.local_intersect(&r);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, -6.0));
        assert!(equal(intersections[1].t, -4.0));
        assert_eq!(intersections[0].object, Object::Sphere(s));
        assert_eq!(intersections[1].object, Object::Sphere(s));
    }
}
