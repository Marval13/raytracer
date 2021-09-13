//use std::f64::consts::PI;
//use raytracer::Vector;
use raytracer::{
    Canvas, Color, Intersection, Material, Matrix, Point, PointLight, Ray, Sphere,
};



fn main() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_size = 200;
    let pixel_size = wall_size / canvas_size as f64;
    let half = wall_size / 2.0;

    //let transform = Matrix::scaling(Vector::new(1.0, 0.5, 0.5));
    //let transform = Matrix::rotation_z(PI / 12.0) * transform;
    let material = Material {
        color: Color::new(1.0, 0.2, 1.0),
        //shininess: 10.0,
        //specular: 0.1,
        //diffuse: 0.5,
        ..Default::default()
    };
    let sphere = Sphere::new(Matrix::default(), material);

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

    c.save();
}
