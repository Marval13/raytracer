use crate::{Color, Point};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    #[must_use]
    pub fn new(position: Point, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point_light() {
        let l = PointLight::default();

        assert_eq!(l.position, Point::new(0.0, 0.0, 0.0));
        assert_eq!(l.intensity, Color::new(1.0, 1.0, 1.0));
    }
}
