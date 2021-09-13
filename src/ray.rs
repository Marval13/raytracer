use crate::{Intersection, Matrix, Point, Sphere, Vector};

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
        //let transformation = &transformation.inverse().unwrap();
        Self {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }

    #[must_use]
    pub fn intersect<'a>(&self, s: &'a Sphere) -> Vec<Intersection<'a>> {
        let ray = self.transform(&s.transform.inverse());
        let sphere_to_ray = ray.origin - Point::default();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Vec::new();
        }

        vec![
            Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), s),
            Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), s),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal;
    use crate::Material;

    #[test]
    fn intersect_sphere_2_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = r.intersect(&s);

        eprintln!("{:?}", intersections);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, 4.0));
        assert!(equal(intersections[1].t, 6.0));
        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
    }

    #[test]
    fn intersect_sphere_1_point() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, 5.0));
        assert!(equal(intersections[1].t, 5.0));
        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
    }

    #[test]
    fn intersect_sphere_0_points() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_sphere_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, -1.0));
        assert!(equal(intersections[1].t, 1.0));
        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
    }

    #[test]
    fn intersect_sphere_behind() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, -6.0));
        assert!(equal(intersections[1].t, -4.0));
        assert_eq!(intersections[0].object, &s);
        assert_eq!(intersections[1].object, &s);
    }

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
    fn intersect_translated_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Matrix::translation(Vector::new(5.0, 0.0, 0.0)),
            Material::default(),
        );
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Matrix::scaling(Vector::new(2.0, 2.0, 2.0)),
            Material::default(),
        );
        let intersections = r.intersect(&s);

        assert_eq!(intersections.len(), 2);
        assert!(equal(intersections[0].t, 3.0));
        assert!(equal(intersections[1].t, 7.0));
    }
}
