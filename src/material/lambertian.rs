use crate::{
    material::Material,
    rtweekend::{color::Color, ray::Ray, vec3::Vec3},
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &crate::rtweekend::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let _ = r_in;
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
