use raytracer::{Canvas, Color, Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let mut c = Canvas::new(100, 100);

    let mut p = Projectile {
        position: Point::new(0.0, 99.0, 0.0),
        velocity: Vector::new(5.0, -12.0, 0.0),
    };

    let env = Environment {
        gravity: Vector::new(0.0, 1.0, 0.0),
        wind: Vector::new(-0.05, 0.0, 0.0),
    };

    c.write_pixel(
        p.position.x as usize,
        p.position.y as usize,
        Color::new(1.0, 0.0, 0.0),
    );

    loop {
        p = tick(&p, &env);
        if p.position.x < 0.0
            || p.position.x >= 100.0
            || p.position.y < 0.0
            || p.position.y >= 100.0
        {
            break;
        }
        c.write_pixel(
            p.position.x as usize,
            p.position.y as usize,
            Color::new(1.0, 0.0, 0.0),
        );
    }

    c.save();
}

fn tick(p: &Projectile, env: &Environment) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}
