#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub mod canvas;
pub mod color;
pub mod point;
pub mod utils;
pub mod vector;

pub use canvas::Canvas;
pub use color::Color;
pub use point::Point;
pub use vector::Vector;
