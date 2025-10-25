use std::f64;

pub mod color;
pub mod vec3;
pub mod ray;
pub mod interval;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * f64::consts::PI / 180.0
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    return rand::random::<f64>();
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    return min + (max - min) * random_double();
}