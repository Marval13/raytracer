use crate::{Color, Computations, Intersection, Object, Point, PointLight, Ray, Shape};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
}

impl World {
    #[must_use]
    pub fn new(objects: Vec<Object>, light: PointLight) -> Self {
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
        comps.object.get_material().lighting(
            comps.point,
            self.light,
            comps.eyev,
            comps.normal,
            self.is_shadowed(comps.over_point),
        )
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

    #[must_use]
    pub fn is_shadowed(&self, point: Point) -> bool {
        let direction = self.light.position - point;
        let distance = direction.magnitude();
        let ray = Ray::new(point, direction.normalize());
        let hit = Intersection::hit(&self.intersect(&ray));

        hit.map_or(false, |hit| hit.t <= distance)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new(Vec::new(), PointLight::default())
    }
}

#[cfg(test)]
pub(crate) mod test_world {
    use crate::{Material, Matrix, Point, Sphere, Vector};

    use super::*;

    pub fn test_world() -> World {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

        let m1 = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };
        let s1 = Object::Sphere(Sphere::new(Matrix::default(), m1));

        let s2 = Object::Sphere(Sphere::new(
            Matrix::scaling(Vector::new(0.5, 0.5, 0.5)),
            Material::default(),
        ));

        World::new(vec![s1, s2], light)
    }
}

#[cfg(test)]
mod tests {
    use super::test_world::test_world;
    use super::*;
    use crate::{vector, Material, Matrix, Sphere};

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
        assert_eq!(world.objects[1].get_material(), Material::default());
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
        let s = world.objects[0];
        let i = Intersection::new(4.0, &s);
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

        world.objects[0].set_material(Material {
            ambient: 1.0,
            ..Default::default()
        });
        world.objects[1].set_material(Material {
            ambient: 1.0,
            ..Default::default()
        });

        assert_eq!(world.color_at(&ray), world.objects[1].get_material().color);
    }

    #[test]
    fn shadow_point_away() {
        let world = test_world();
        assert!(!world.is_shadowed(Point::new(0.0, 10.0, 0.0)));
    }

    #[test]
    fn shadow_light_object_point() {
        let world = test_world();
        assert!(world.is_shadowed(Point::new(10.0, -10.0, 10.0)));
    }

    #[test]
    fn shadow_point_light_object() {
        let world = test_world();
        assert!(!world.is_shadowed(Point::new(-20.0, 20.0, -20.0)));
    }

    #[test]
    fn shadow_light_point_object() {
        let world = test_world();
        assert!(!world.is_shadowed(Point::new(-2.0, 2.0, -2.0)));
    }
    #[test]
    fn shade_hit_and_shadows() {
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
        let s1 = Object::Sphere(Sphere::default());
        let s2 = Object::Sphere(Sphere::new(
            Matrix::translation(vector::Z * 10.0),
            Material::default(),
        ));
        let world = World::new(vec![s1, s2], light);
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), vector::Z);
        let i = Intersection::new(4.0, &world.objects[1]);
        let comps = i.prepare_computations(&ray);

        assert_eq!(world.shade_hit(comps), Color::new(0.1, 0.1, 0.1));
    }
}
