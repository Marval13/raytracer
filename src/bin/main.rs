use raytracer::pattern::StripePattern;
use raytracer::{
    point, vector, Camera, Color, Material, Matrix, Object, Pattern, Plane, Point, PointLight,
    Shape, Sphere, Vector, World,
};
use std::f64::consts::PI;

use std::path::Path;

fn main() {
    let matte_gray = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };

    let floor = Plane::new(Matrix::default(), matte_gray);

    let mut sphere1 = Sphere::new(
        Matrix::translation(Vector::new(-0.5, 1.0, 0.5)),
        Material {
            color: Color::new(0.1, 1.0, 0.5),
            pattern: Pattern::Stripe(StripePattern::default()),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );
    sphere1.material.pattern.set_transform(Matrix::scaling(Vector::new(0.2, 0.2, 0.2)));

    let sphere2 = Sphere::new(
        Matrix::translation(Vector::new(1.5, 0.5, -0.5))
            * Matrix::scaling(Vector::new(0.5, 0.5, 0.5)),
        Material {
            color: Color::new(0.5, 1.0, 0.1),
            pattern: Pattern::Stripe(StripePattern::default()),
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
            pattern: Pattern::Stripe(StripePattern::default()),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    );

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

    let world = World::new(
        vec![
            Object::Plane(floor),
            Object::Sphere(sphere1),
            Object::Sphere(sphere2),
            Object::Sphere(sphere3),
        ],
        light,
    );
    let mut camera = Camera::new(300, 150, PI / 3.0);
    camera.transform = Matrix::view_transform(Point::new(0.0, 1.5, -5.0), point::UY, vector::Y);

    camera.render(&world).save(Path::new("./img.ppm"));
}
