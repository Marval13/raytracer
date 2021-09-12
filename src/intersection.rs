use crate::Sphere;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    #[must_use]
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        Self { t, object }
    }

    #[must_use]
    pub fn hit(intersections: &[Self]) -> Option<Self> {
        let i = intersections
            .iter()
            .filter(|i| i.t > 0.0)
            .min_by(|i, j| i.t.partial_cmp(&j.t).unwrap())?;

        Some(*i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn hit_positive() {
        let s = Sphere::default();
        let intersections = vec![Intersection::new(1.0, &s), Intersection::new(2.0, &s)];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_negative() {
        let s = Sphere::default();
        let intersections = vec![Intersection::new(1.0, &s), Intersection::new(-1.0, &s)];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 1.0);
    }

    #[test]
    fn hit_all_negative() {
        let s = Sphere::default();
        let intersections = vec![Intersection::new(-2.0, &s), Intersection::new(-1.0, &s)];
        let i = Intersection::hit(&intersections);

        assert!(i.is_none());
    }

    #[test]
    fn hit_big() {
        let s = Sphere::default();
        let intersections = vec![
            Intersection::new(5.0, &s),
            Intersection::new(7.0, &s),
            Intersection::new(-3.0, &s),
            Intersection::new(2.0, &s),
        ];
        let i = Intersection::hit(&intersections).unwrap();

        assert_eq!(i.t, 2.0);
    }
}
