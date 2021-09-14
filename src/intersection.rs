use crate::utils::EPSILON;
use crate::{Object, Point, Ray, Shape, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Computations {
    pub t: f64,
    pub object: Object,
    pub point: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
}

impl Intersection {
    #[must_use]
    pub fn new(t: f64, object: &Object) -> Self {
        Self { t, object: *object }
    }

    #[must_use]
    pub fn hit(intersections: &[Self]) -> Option<Self> {
        let i = intersections
            .iter()
            .filter(|i| i.t > 0.0)
            .min_by(|i, j| i.t.partial_cmp(&j.t).unwrap())?;

        Some(*i)
    }

    #[must_use]
    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let normal = self.object.normal_at(point);
        let inside = normal.dot(&eyev) < 0.0;
        let normal = if inside { -normal } else { normal };

        Computations {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normal,
            inside,
            over_point: point + normal * EPSILON,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vector, Material, Matrix, Sphere};

    #[test]
    fn new_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &Object::Sphere(s));

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, Object::Sphere(s));
    }

    #[test]
    fn hit_positive() {
        let s = Sphere::default();
        let intersections = vec![
            Intersection::new(1.0, &Object::Sphere(s)),
            Intersection::new(2.0, &Object::Sphere(s)),
        ];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_negative() {
        let s = Sphere::default();
        let intersections = vec![
            Intersection::new(1.0, &Object::Sphere(s)),
            Intersection::new(-1.0, &Object::Sphere(s)),
        ];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::default();
        let intersections = vec![
            Intersection::new(-2.0, &Object::Sphere(s)),
            Intersection::new(-1.0, &Object::Sphere(s)),
        ];
        let i = Intersection::hit(&intersections);

        assert!(i.is_none());
    }

    #[test]
    fn hit_big() {
        let s = Sphere::default();
        let intersections = vec![
            Intersection::new(5.0, &Object::Sphere(s)),
            Intersection::new(7.0, &Object::Sphere(s)),
            Intersection::new(-3.0, &Object::Sphere(s)),
            Intersection::new(2.0, &Object::Sphere(s)),
        ];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 2.0);
    }

    #[test]
    fn precomputations() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = ray.intersect(&s)[0];
        let comps = i.prepare_computations(&ray);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputations_inside() {
        let ray = Ray::new(Point::default(), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let i = Intersection::new(1.0, &Object::Sphere(s));
        let comps = i.prepare_computations(&ray);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn precomputations_over_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), vector::Z);
        let s = Sphere::new(Matrix::translation(vector::Z), Material::default());
        let i = Intersection::new(5.0, &Object::Sphere(s));
        let comps = i.prepare_computations(&ray);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.over_point.z < comps.point.z);
    }
}
