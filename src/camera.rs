use crate::{Canvas, Matrix, Point, Ray, World};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub h_size: usize,
    pub v_size: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (h_size as f64) / (v_size as f64);
        let (half_width, half_height) = if aspect > 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / h_size as f64;

        Self {
            h_size,
            v_size,
            field_of_view,
            transform: Matrix::default(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let transform_inv = self.transform.inverse();
        let pixel = &transform_inv * Point::new(world_x, world_y, -1.0);
        let origin = &transform_inv * Point::default();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    #[must_use]
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);
        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::equal;
    use crate::world::test_world::test_world;
    use crate::{vector, Color, Vector};
    use std::f64::consts::PI;

    #[test]
    fn new_camera() {
        let c = Camera::new(160, 120, PI / 2.0);

        assert_eq!(c.h_size, 160);
        assert_eq!(c.v_size, 120);
        assert!(equal(c.field_of_view, PI / 2.0));
        assert_eq!(c.transform, Matrix::default());
    }

    #[test]
    fn pixel_size() {
        assert!(equal(Camera::new(200, 125, PI / 2.0).pixel_size, 0.01));
        assert!(equal(Camera::new(125, 200, PI / 2.0).pixel_size, 0.01));
    }

    #[test]
    fn ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::default());
        assert_eq!(r.direction, -vector::Z);
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Point::default());
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_through_transformed_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform =
            Matrix::rotation_y(PI / 4.0) * Matrix::translation(Vector::new(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / -2.0)
        );
    }

    #[test]
    fn render_world() {
        let world = test_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        c.transform =
            Matrix::view_transform(Point::new(0.0, 0.0, -5.0), Point::default(), vector::Y);
        let image = c.render(&world);
        assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}
