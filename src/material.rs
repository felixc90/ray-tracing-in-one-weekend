use crate::hittable::HitRecord;
use crate::rtweekend::color::Color;
use crate::rtweekend::ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
