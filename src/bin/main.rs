use std::f64::consts::PI;

use raytracer::{Canvas, Color, Intersection, Matrix, Point, Ray, Sphere, Vector};

fn main() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_size = 200;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;
    let color = Color::new(1.0, 0.0, 0.0);
    let t = Matrix::scaling(Vector::new(1.0, 0.5, 0.5));
    let t = Matrix::rotation_z(PI / 12.0) * t;
    let s = Sphere::new(t);

    let mut c = Canvas::new(canvas_size, canvas_size);

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_size {
            let world_x = half - pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());
            if Intersection::hit(&ray.intersect(&s)).is_some() {
                c.write_pixel(x, y, color);
            }
        }
    }

    c.save();
}
