#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod canvas;
pub mod color;
pub mod intersection;
pub mod light;
pub mod material;
pub mod matrix;
pub mod point;
pub mod ray;
pub mod sphere;
pub mod transformations;
pub mod utils;
pub mod vector;
pub mod world;

pub use canvas::Canvas;
pub use color::Color;
pub use intersection::Computations;
pub use intersection::Intersection;
pub use light::PointLight;
pub use material::Material;
pub use matrix::Matrix;
pub use point::Point;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vector::Vector;
pub use world::World;
