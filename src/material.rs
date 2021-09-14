use crate::utils::equal;
use crate::{Color, Point, PointLight, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && equal(self.ambient, other.ambient)
            && equal(self.diffuse, other.diffuse)
            && equal(self.specular, other.specular)
            && equal(self.shininess, other.shininess)
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::default(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    #[must_use]
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    #[must_use]
    pub fn lighting(
        &self,
        point: Point,
        light: PointLight,
        eyev: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(&normal);

        if in_shadow {
            ambient
        } else {
            let diffuse;
            let specular;

            if light_dot_normal < 0.0 {
                diffuse = Color::black();
                specular = Color::black();
            } else {
                diffuse = effective_color * self.diffuse * light_dot_normal;
                let reflectv = (-lightv).reflect(&normal);
                let reflect_dot_eye = reflectv.dot(&eyev);
                if reflect_dot_eye <= 0.0 {
                    specular = Color::black();
                } else {
                    let factor = reflect_dot_eye.powf(self.shininess);
                    specular = light.intensity * self.specular * factor;
                }
            }

            ambient + diffuse + specular
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(equal(m.ambient, 0.1));
        assert!(equal(m.diffuse, 0.9));
        assert!(equal(m.specular, 0.9));
        assert!(equal(m.shininess, 200.0));
    }

    #[test]
    fn lighting_straight() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, false),
            Color::new(1.9, 1.9, 1.9),
        );
    }

    #[test]
    fn lighting_eye_45_degrees() {
        let eye = Vector::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, false),
            Color::new(1.0, 1.0, 1.0),
        );
    }

    #[test]
    fn lighting_light_45_degrees() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, false),
            Color::new(0.7364, 0.7364, 0.7364),
        );
    }

    #[test]
    fn lighting_eye_light_45_degrees() {
        let eye = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, false),
            Color::new(1.6364, 1.6364, 1.6364),
        );
    }

    #[test]
    fn lighting_behind() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, false),
            Color::new(0.1, 0.1, 0.1),
        );
    }

    #[test]
    fn lighting_in_shadow() {
        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::default());

        assert_eq!(
            Material::default().lighting(Point::default(), light, eye, normal, true),
            Color::new(0.1, 0.1, 0.1),
        );
    }
}
