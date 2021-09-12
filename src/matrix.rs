use crate::utils::equal;
use crate::Point;
use crate::Vector;

use grid::Grid;

use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub dimension: usize,
    pub grid: Grid<f64>,
}

impl Matrix {
    #[must_use]
    pub fn new(dimension: usize, contents: Vec<f64>) -> Self {
        if contents.len() != dimension * dimension {
            panic!();
        }

        Self {
            dimension,
            grid: Grid::from_vec(contents, dimension),
        }
    }

    #[must_use]
    pub fn eye(dimension: usize) -> Self {
        let mut grid = Grid::new(dimension, dimension);
        for i in 0..dimension {
            *grid.get_mut(i, i).unwrap() = 1.0;
        }

        Self { dimension, grid }
    }

    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        *self.grid.get(row, col).unwrap()
    }

    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut grid = Grid::init(self.dimension, self.dimension, 0.0);
        for row in 0..self.dimension {
            for col in 0..self.dimension {
                *grid.get_mut(row, col).unwrap() = *self.grid.get(col, row).unwrap();
            }
        }

        Self {
            dimension: self.dimension,
            grid,
        }
    }

    #[must_use]
    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        if row >= self.dimension || col >= self.dimension {
            panic!();
        }

        let mut grid = Grid::init(self.dimension - 1, self.dimension - 1, 0.0);
        for new_row in 0..self.dimension - 1 {
            for new_col in 0..self.dimension - 1 {
                *grid.get_mut(new_row, new_col).unwrap() = *self
                    .grid
                    .get(
                        if new_row >= row { new_row + 1 } else { new_row },
                        if new_col >= col { new_col + 1 } else { new_col },
                    )
                    .unwrap();
            }
        }

        Self {
            dimension: self.dimension - 1,
            grid,
        }
    }

    #[must_use]
    pub fn determinant(&self) -> f64 {
        if self.dimension == 2 {
            return self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0);
        }

        let mut determinant = 0.0;
        for row in 0..self.dimension {
            determinant += self.grid.get(row, 0).unwrap() * self.cofactor(row, 0);
        }

        determinant
    }

    #[must_use]
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    #[must_use]
    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row, col) * if (row + col) % 2 == 0 { 1.0 } else { -1.0 }
    }

    #[must_use]
    pub fn inverse(&self) -> Option<Matrix> {
        let determinant = self.determinant();
        if determinant == 0.0 {
            return None;
        }

        let mut grid = Grid::new(self.dimension, self.dimension);
        for row in 0..self.dimension {
            for col in 0..self.dimension {
                *grid.get_mut(row, col).unwrap() = self.cofactor(col, row) / determinant;
            }
        }

        Some(Matrix {
            dimension: self.dimension,
            grid,
        })
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.grid
            .iter()
            .zip(other.grid.iter())
            .all(|(&x, &y)| equal(x, y))
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.dimension != other.dimension {
            panic!();
        }

        let dimension = self.dimension;

        let mut grid = Grid::new(dimension, dimension);

        for row in 0..dimension {
            for col in 0..dimension {
                let cell = grid.get_mut(row, col).unwrap();
                for i in 0..dimension {
                    *cell += self.get(row, i) * other.get(i, col);
                }
            }
        }

        Self { dimension, grid }
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        let x = other.x;
        let y = other.y;
        let z = other.z;

        Vector {
            x: x * self.get(0, 0) + y * self.get(0, 1) + z * self.get(0, 2),
            y: x * self.get(1, 0) + y * self.get(1, 1) + z * self.get(1, 2),
            z: x * self.get(2, 0) + y * self.get(2, 1) + z * self.get(2, 2),
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let x = other.x;
        let y = other.y;
        let z = other.z;

        Point {
            x: x * self.get(0, 0) + y * self.get(0, 1) + z * self.get(0, 2) + self.get(0, 3),
            y: x * self.get(1, 0) + y * self.get(1, 1) + z * self.get(1, 2) + self.get(1, 3),
            z: x * self.get(2, 0) + y * self.get(2, 1) + z * self.get(2, 2) + self.get(2, 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_matrix_4() {
        #[rustfmt::skip]
        let m = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        assert!(equal(m.get(0, 0), 1.0));
        assert!(equal(m.get(0, 3), 4.0));
        assert!(equal(m.get(1, 0), 5.5));
        assert!(equal(m.get(1, 2), 7.5));
        assert!(equal(m.get(2, 2), 11.0));
        assert!(equal(m.get(3, 0), 13.5));
        assert!(equal(m.get(3, 2), 15.5));
    }

    #[test]
    fn new_matrix_2() {
        let m = Matrix::new(2, vec![-3.0, 5.0, 1.0, -2.0]);

        assert!(equal(m.get(0, 0), -3.0));
        assert!(equal(m.get(0, 1), 5.0));
        assert!(equal(m.get(1, 0), 1.0));
        assert!(equal(m.get(1, 1), -2.0));
    }

    #[test]
    fn new_matrix_3() {
        #[rustfmt::skip]
        let m = Matrix::new(3, vec![
            -3.0, 5.0, 0.0,
            1.0,-2.0, -7.0,
            0.0, 1.0, 1.0
        ]);

        assert!(equal(m.get(0, 0), -3.0));
        assert!(equal(m.get(1, 1), -2.0));
        assert!(equal(m.get(2, 2), 1.0));
    }

    #[test]
    fn matrix_equality() {
        #[rustfmt::skip]
        let m1 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        #[rustfmt::skip]
        let m3 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.1,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn matrix_mul() {
        #[rustfmt::skip]
        let m1 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ]);

        #[rustfmt::skip]
        let m3 = Matrix::new(4, vec![
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        ]);

        assert_eq!(m1 * m2, m3);
    }

    #[test]
    fn matrix_mul_point() {
        #[rustfmt::skip]
        let m = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(m * p, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn matrix_mul_eye() {
        #[rustfmt::skip]
        let m1 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let eye = Matrix::eye(4);

        assert_eq!(m1 * eye, m2);
    }

    #[test]
    fn matrix_transpose() {
        #[rustfmt::skip]
        let m1 = Matrix::new(4, vec![
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0,
        ]);

        assert_eq!(m1.transpose(), m2);

        assert_eq!(Matrix::eye(3), Matrix::eye(3));
    }

    #[test]
    fn matrix_minor() {
        #[rustfmt::skip]
        let m1 = Matrix::new(3, vec![
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(2, vec![
            -3.0, 2.0,
            0.0, 6.0,
        ]);

        #[rustfmt::skip]
        let m3 = Matrix::new(4, vec![
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0,
        ]);

        #[rustfmt::skip]
        let m4 = Matrix::new(3, vec![
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0,
        ]);

        assert_eq!(m1.submatrix(0, 2), m2);
        assert_eq!(m3.submatrix(2, 1), m4);
    }

    #[test]
    fn matrix_det() {
        #[rustfmt::skip]
        let m1 = Matrix::new(3, vec![
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        ]);

        assert!(equal(m1.determinant(), -196.0));
        assert!(equal(m2.determinant(), -4071.0));
    }

    #[test]
    fn matrix_inv() {
        #[rustfmt::skip]
        let m1 = Matrix::new(4, vec![
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0,
        ]);

        #[rustfmt::skip]
        let m2 = Matrix::new(4, vec![
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639,
        ]);

        #[rustfmt::skip]
        let m3 = Matrix::new(4, vec![
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0,
        ]);

        #[rustfmt::skip]
        let m4 = Matrix::new(4, vec![
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308,
        ]);

        #[rustfmt::skip]
        let m5 = Matrix::new(4, vec![
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0,
        ]);

        #[rustfmt::skip]
        let m6 = Matrix::new(4, vec![
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333,
        ]);

        assert_eq!(m1.inverse().unwrap(), m2);
        assert_eq!(m3.inverse().unwrap(), m4);
        assert_eq!(m5.inverse().unwrap(), m6);
        assert_eq!(m1.inverse().unwrap().inverse().unwrap(), m1);
        assert_eq!(m3.inverse().unwrap() * m3, Matrix::eye(4));
    }
}
