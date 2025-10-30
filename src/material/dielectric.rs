use crate::{
    hittable::HitRecord,
    material::Material,
    rtweekend::{color::Color, random_double, ray::Ray, vec3::Vec3},
};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            direction = Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }
        *scattered = Ray::new(rec.p, direction);
        true
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
