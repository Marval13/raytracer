use crate::color::Color;

use grid::Grid;

use std::fs::File;
use std::io::Write;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Grid<Color>,
}

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            canvas: Grid::init(height, width, Color::new(0.0, 0.0, 0.0)),
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let pixel = self.canvas.get_mut(y, x).unwrap();
        *pixel = c;
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_ppm(&self) -> Vec<String> {
        let mut ppm = vec![
            "P3".to_string(),
            format!("{} {}", self.width, self.height),
            "255".to_string()];

        for row in 0..self.height {
            let mut row_buf = Vec::new();
            for cell in self.canvas.iter_row(row) {
                row_buf.push(format!("{}", (cell.r.clamp(0.0, 1.0) * 255.0).round() as isize));
                row_buf.push(format!("{}", (cell.g.clamp(0.0, 1.0) * 255.0).round() as isize));
                row_buf.push(format!("{}", (cell.b.clamp(0.0, 1.0) * 255.0).round() as isize));
            }
            
            let mut pixel_row = String::new();
            for data in row_buf {
                if pixel_row.len() + data.len() + 1 > 70 {
                    ppm.push(pixel_row.clone());
                    pixel_row = String::new();
                }

                if !pixel_row.is_empty() {
                    pixel_row.push(' ');
                }
                pixel_row.push_str(&data);
            }
            ppm.push(pixel_row);
        }
        ppm.push(String::new());
        ppm
    }

    pub fn save(&self) {
        let mut file = File::create("img.ppm").expect("create failed");
        for line in &self.to_ppm() {
            file.write_all(line.as_bytes()).expect("write failed");
            file.write_all(b"\n").expect("write failed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
        for cell in canvas.canvas.iter() {
            assert_eq!(*cell, Color::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(*canvas.canvas.get(3, 2).unwrap(), Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm[0], String::from("P3"));
        assert_eq!(ppm[1], String::from("5 3"));
        assert_eq!(ppm[2], String::from("255"));
    }

    #[test]
    fn ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = c.to_ppm();

        assert_eq!(ppm[3], String::from("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert_eq!(ppm[4], String::from("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"));
        assert_eq!(ppm[5], String::from("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));
    }

    #[test]
    fn ppm_long_lines() {
        let mut c = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        let ppm = c.to_ppm();

        assert_eq!(ppm[3], String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(ppm[4], String::from("153 255 204 153 255 204 153 255 204 153 255 204 153"));
        assert_eq!(ppm[5], String::from("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"));
        assert_eq!(ppm[6], String::from("153 255 204 153 255 204 153 255 204 153 255 204 153"));
    }

    #[test]
    fn ppm_newline_at_end() {
        let c = Canvas::new(3, 2);
        let ppm = c.to_ppm();
        assert_eq!(ppm[5], String::new());
    }
}
