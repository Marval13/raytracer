use crate::{Color, Computations, Intersection, PointLight, Ray, Sphere};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: PointLight,
}

impl World {
    #[must_use]
    pub fn new(objects: Vec<Sphere>, light: PointLight) -> Self {
        Self { objects, light }
    }

    #[must_use]
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();

        for object in &self.objects {
            intersections.append(&mut ray.intersect(object));
        }

        intersections.sort_unstable_by(|i, j| i.t.partial_cmp(&j.t).unwrap());
        intersections
    }

    #[must_use]
    pub fn shade_hit(&self, comps: Computations) -> Color {
        comps
            .object
            .material
            .lighting(comps.point, self.light, comps.eyev, comps.normal)
    }

    #[must_use]
    pub fn color_at(&self, ray: &Ray) -> Color {
        let hit = Intersection::hit(&self.intersect(ray));
        if hit.is_none() {
            return Color::black();
        }
        let hit = hit.unwrap();
        let comps = hit.prepare_computations(ray);
        self.shade_hit(comps)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(Vec::new(), PointLight::default())
    }
}

#[cfg(test)]
pub(crate) mod test_world {
    use crate::{Material, Matrix, Point, Vector};

    use super::*;

    pub fn test_world() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let m1 = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let s1 = Sphere::new(Matrix::default(), m1);

        let s2 = Sphere::new(
            Matrix::scaling(Vector::new(0.5, 0.5, 0.5)),
            Material::default(),
        );

        World::new(vec![s1, s2], light)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;
    use crate::{Material, Point};

    use super::test_world::test_world;
    use super::*;

    #[test]
    fn new_world() {
        let world = World::default();

        assert!(world.objects.is_empty());
        assert_eq!(world.light, PointLight::default());
    }

    #[test]
    fn create_test_world() {
        let world = test_world();

        assert_eq!(world.objects.len(), 2);
        assert_eq!(world.objects[1].material, Material::default());
    }

    #[test]
    fn world_intersect() {
        let world = test_world();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), vector::Z);
        let intersections = world.intersect(&ray);

        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].t, 4.0);
        assert_eq!(intersections[1].t, 4.5);
        assert_eq!(intersections[2].t, 5.5);
        assert_eq!(intersections[3].t, 6.0);
    }

    #[test]
    fn shade_outside() {
        let world = test_world();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), vector::Z);
        let s = &world.objects[0];
        let i = Intersection::new(4.0, s);
        let comps = i.prepare_computations(&ray);

        assert_eq!(world.shade_hit(comps), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_inside() {
        let mut world = test_world();
        world.light = PointLight::new(Point::new(0.0, 0.25, 0.0), Color::white());
        let ray = Ray::new(Point::default(), vector::Z);
        let s = &world.objects[1];
        let i = Intersection::new(0.5, s);
        let comps = i.prepare_computations(&ray);

        assert_eq!(
            world.shade_hit(comps),
            Color::new(0.90498, 0.90498, 0.90498)
        );
    }

    #[test]
    fn world_shade_miss() {
        let world = test_world();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), vector::Y);

        assert_eq!(world.color_at(&ray), Color::black());
    }

    #[test]
    fn world_shade_hit() {
        let world = test_world();
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), vector::Z);

        assert_eq!(world.color_at(&ray), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn world_shade_hit_inner() {
        let mut world = test_world();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.75), -vector::Z);

        world.objects[0].material.ambient = 1.0;
        world.objects[1].material.ambient = 1.0;

        assert_eq!(world.color_at(&ray), world.objects[1].material.color);
    }
}
