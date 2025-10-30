use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::rtweekend::interval::Interval;
use crate::rtweekend::vec3::Point3;
use crate::rtweekend::ray::Ray;

pub struct Sphere {
	center: Point3,
	radius: f64,
	mat: Rc<dyn Material>
}

impl Sphere {
	pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
		Sphere { center, radius, mat }
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let oc = self.center - r.origin();
		let a = r.direction().length_squared();
		let h = r.direction().dot(&oc);
		let c = oc.length_squared() - self.radius * self.radius;
		let discriminant = h * h - a * c;

		if discriminant < 0.0 {
			return false;
		}

		let sqrtd = discriminant.sqrt();
		let mut root = (h - sqrtd) / a;
		if !ray_t.surrounds(root) {
			root = (h + sqrtd) / a;
			if !ray_t.surrounds(root) {
				return false;
			}
		}

		rec.t = root;
		rec.p = r.at(rec.t);

		let outward_normal = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, &outward_normal);
		rec.mat = Some(Rc::clone(&self.mat));

		return true;
	}
}