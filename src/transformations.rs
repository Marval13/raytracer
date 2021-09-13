use crate::{Matrix, Point, Vector};

use grid::Grid;

impl Matrix {
    #[must_use]
    pub fn translation(v: Vector) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            1.0, 0.0, 0.0, v.x,
            0.0, 1.0, 0.0, v.y,
            0.0, 0.0, 1.0, v.z,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn scaling(v: Vector) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            v.x, 0.0, 0.0, 0.0,
            0.0, v.y, 0.0, 0.0,
            0.0, 0.0, v.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn rotation_x(angle: f64) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, angle.cos(), -angle.sin(), 0.0,
            0.0, angle.sin(), angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn rotation_y(angle: f64) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            angle.cos(), 0.0, angle.sin(), 0.0,
            0.0, 1.0, 0.0, 0.0,
            -angle.sin(), 0.0, angle.cos(), 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn rotation_z(angle: f64) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            angle.cos(), -angle.sin(), 0.0, 0.0,
            angle.sin(), angle.cos(), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        #[rustfmt::skip]
        let v_grid = vec![
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        }
    }

    #[must_use]
    pub fn view_transform(from: Point, to: Point, up: Vector) -> Self {
        let f = (to - from).normalize();
        let l = f.cross(&up.normalize());
        let u = l.cross(&f);

        #[rustfmt::skip]
        let v_grid = vec![
            l.x,  l.y,  l.z,  0.0,
            u.x,  u.y,  u.z,  0.0,
            -f.x, -f.y, -f.z, 0.0,
            0.0,  0.0,  0.0,  1.0,
        ];

        let orientation = Self {
            dimension: 4,
            grid: Grid::from_vec(v_grid, 4),
        };

        orientation * Matrix::translation(Vector::new(-from.x, -from.y, -from.z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector;

    use std::f64::consts::PI;

    #[test]
    fn translation_point() {
        let t = Matrix::translation(Vector::new(5.0, -3.0, 2.0));
        assert_eq!(t * Point::new(-3.0, 4.0, 5.0), Point::new(2.0, 1.0, 7.0),);
    }

    #[test]
    fn translation_inverse() {
        let t = Matrix::translation(Vector::new(5.0, -3.0, 2.0)).inverse();
        assert_eq!(t * Point::new(2.0, 1.0, 7.0), Point::new(-3.0, 4.0, 5.0),);
    }

    #[test]
    fn translation_vector() {
        let t = Matrix::translation(Vector::new(5.0, -3.0, 2.0)).inverse();
        assert_eq!(t * Vector::new(2.0, 1.0, 7.0), Vector::new(2.0, 1.0, 7.0),);
    }

    #[test]
    fn scaling_point() {
        let t = Matrix::scaling(Vector::new(2.0, 3.0, 4.0));
        assert_eq!(t * Point::new(-4.0, 6.0, 8.0), Point::new(-8.0, 18.0, 32.0),);
    }

    #[test]
    fn scaling_vector() {
        let t = Matrix::scaling(Vector::new(2.0, 3.0, 4.0));
        assert_eq!(
            t * Vector::new(-4.0, 6.0, 8.0),
            Vector::new(-8.0, 18.0, 32.0),
        );
    }

    #[test]
    fn scaling_inverse() {
        let t = Matrix::scaling(Vector::new(2.0, 3.0, 4.0)).inverse();
        assert_eq!(t * Point::new(-8.0, 18.0, 32.0), Point::new(-4.0, 6.0, 8.0),);
    }

    #[test]
    fn scaling_reflection() {
        let t = Matrix::scaling(Vector::new(-1.0, -1.0, -1.0));
        assert_eq!(t * Point::new(4.0, -6.0, -8.0), Point::new(-4.0, 6.0, 8.0),);
    }

    #[test]
    fn rotation_x() {
        let t1 = Matrix::rotation_x(PI / 4.0);
        let t2 = Matrix::rotation_x(PI / 2.0);
        let t3 = Matrix::rotation_x(PI / 4.0).inverse();

        assert_eq!(
            t1 * Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );

        assert_eq!(t2 * Point::new(0.0, 1.0, 0.0), Point::new(0.0, 0.0, 1.0),);

        assert_eq!(
            t3 * Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / -2.0),
        );
    }

    #[test]
    fn rotation_y() {
        let t1 = Matrix::rotation_y(PI / 4.0);
        let t2 = Matrix::rotation_y(PI / 2.0);

        assert_eq!(
            t1 * Point::new(0.0, 0.0, 1.0),
            Point::new(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0),
        );

        assert_eq!(t2 * Point::new(0.0, 0.0, 1.0), Point::new(1.0, 0.0, 0.0),);
    }

    #[test]
    fn rotation_z() {
        let t1 = Matrix::rotation_z(PI / 4.0);
        let t2 = Matrix::rotation_z(PI / 2.0);

        assert_eq!(
            t1 * Point::new(0.0, 1.0, 0.0),
            Point::new(2_f64.sqrt() / -2.0, 2_f64.sqrt() / 2.0, 0.0),
        );

        assert_eq!(t2 * Point::new(0.0, 1.0, 0.0), Point::new(-1.0, 0.0, 0.0),);
    }

    #[test]
    fn shearing() {
        let t1 = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let t2 = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let t3 = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let t4 = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let t5 = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let t6 = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(t1 * Point::new(2.0, 3.0, 4.0), Point::new(5.0, 3.0, 4.0));

        assert_eq!(t2 * Point::new(2.0, 3.0, 4.0), Point::new(6.0, 3.0, 4.0));

        assert_eq!(t3 * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 5.0, 4.0));

        assert_eq!(t4 * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 7.0, 4.0));

        assert_eq!(t5 * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 3.0, 6.0));

        assert_eq!(t6 * Point::new(2.0, 3.0, 4.0), Point::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn view_transform() {
        assert_eq!(
            Matrix::view_transform(Point::default(), Point::new(0.0, 0.0, -1.0), vector::Y),
            Matrix::default(),
        );

        assert_eq!(
            Matrix::view_transform(Point::default(), Point::new(0.0, 0.0, 1.0), vector::Y),
            Matrix::scaling(Vector::new(-1.0, 1.0, -1.0)),
        );

        assert_eq!(
            Matrix::view_transform(Point::new(0.0, 0.0, 8.0), Point::default(), vector::Y),
            Matrix::translation(Vector::new(0.0, 0.0, -8.0)),
        );

        #[rustfmt::skip]
        assert_eq!(
            Matrix::view_transform(
                Point::new(1.0, 3.0, 2.0),
                Point::new(4.0, -2.0, 8.0),
                vector::Y + vector::X,
            ),
            Matrix::new(4, vec![
                -0.50709, 0.50709, 0.67612, -2.36643,
                0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.00000,
                0.00000, 0.00000, 0.00000, 1.00000,
            ]),
        );
    }
}
