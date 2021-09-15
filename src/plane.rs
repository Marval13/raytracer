use crate::utils::EPSILON;
use crate::{vector, Intersection, Material, Matrix, Object, Point, Ray, Shape, Vector};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Plane {
    transform: Matrix,
    material: Material,
}

impl Shape for Plane {
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
        if ray.direction.y.abs() < EPSILON {
            Vec::new()
        } else {
            vec![Intersection::new(
                -ray.origin.y / ray.direction.y,
                &Object::Plane(*self),
            )]
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        vector::Y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Object;

    #[test]
    fn normals() {
        let p = Plane::default();
        assert_eq!(p.local_normal_at(Point::default()), vector::Y);
        assert_eq!(p.local_normal_at(Point::new(10.0, 0.0, -10.0)), vector::Y);
        assert_eq!(p.local_normal_at(Point::new(-5.0, 0.0, 150.0)), vector::Y);
    }

    #[test]
    fn intersect_parallel() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), vector::Z);
        assert!(p.local_intersect(&r).is_empty());
    }

    #[test]
    fn intersect_coplanar() {
        let p = Plane::default();
        let r = Ray::new(Point::default(), vector::Z);
        assert!(p.local_intersect(&r).is_empty());
    }

    #[test]
    fn intersect_from_above() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), -vector::Y);
        let intersections = p.local_intersect(&r);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, Object::Plane(p));
    }

    #[test]
    fn intersect_from_below() {
        let p = Plane::default();
        let r = Ray::new(Point::new(0.0, -1.0, 0.0), vector::Y);
        let intersections = p.local_intersect(&r);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, Object::Plane(p));
    }
}
