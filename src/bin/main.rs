use raytracer::{point, vector, Camera, Vector, World};
use raytracer::{Canvas, Color, Intersection, Material, Matrix, Point, PointLight, Ray, Sphere};
use std::f64::consts::PI;

use std::path::Path;

fn main() {
    chapter7();
}

#[allow(dead_code)]
fn chapter6() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_size = 200;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    let transform = Matrix::scaling(Vector::new(1.0, 0.5, 0.5));
    let transform = Matrix::rotation_z(PI / 12.0) * transform;
    let material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        ..Default::default()
    };
    let sphere = Sphere::new(transform, material);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

    let mut c = Canvas::new(canvas_size, canvas_size);

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_size {
            let world_x = half - pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(origin, (position - origin).normalize());

            if let Some(hit) = Intersection::hit(&ray.intersect(&sphere)) {
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;

                let color = hit.object.material.lighting(point, light, eye, normal);

                c.write_pixel(x, y, color);
            }
        }
    }

    c.save(Path::new("./renders/chapter6.ppm"));
}

fn chapter7() {
    let matte_gray = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let left_wall_transform = Matrix::translation(vector::Z * 5.0)
        * Matrix::rotation_y(-PI / 4.0)
        * Matrix::rotation_x(PI / 2.0)
        * Matrix::scaling(Vector::new(10.0, 0.01, 10.0));

    let right_wall_transform = Matrix::translation(vector::Z * 5.0)
        * Matrix::rotation_y(PI / 4.0)
        * Matrix::rotation_x(PI / 2.0)
        * Matrix::scaling(Vector::new(10.0, 0.01, 10.0));

    let floor = Sphere::new(Matrix::scaling(Vector::new(10.0, 0.01, 10.0)), matte_gray);
    let left_wall = Sphere::new(left_wall_transform, matte_gray);
    let right_wall = Sphere::new(right_wall_transform, matte_gray);

    let sphere1 = Sphere::new(
        Matrix::translation(Vector::new(-0.5, 1.0, 0.5)),
        Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );

    let sphere2 = Sphere::new(
        Matrix::translation(Vector::new(1.5, 0.5, -0.5))
            * Matrix::scaling(Vector::new(0.5, 0.5, 0.5)),
        Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );

    let sphere3 = Sphere::new(
        Matrix::translation(Vector::new(-1.5, 0.33, -0.75))
            * Matrix::scaling(Vector::new(0.33, 0.33, 0.33)),
        Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

    let world = World::new(
        vec![floor, left_wall, right_wall, sphere1, sphere2, sphere3],
        light,
    );
    let mut camera = Camera::new(500, 250, PI / 3.0);
    camera.transform = Matrix::view_transform(Point::new(0.0, 1.5, -5.0), point::UY, vector::Y);

    camera.render(&world).save(Path::new("./renders/chapter7.ppm"));
}
