use std::f64;

pub mod color;
pub mod vec3;
pub mod ray;
pub mod interval;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}