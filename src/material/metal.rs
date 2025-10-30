use crate::{
    hittable::HitRecord,
    material::Material,
    rtweekend::{color::Color, ray::Ray, vec3::Vec3},
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = Vec3::reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
