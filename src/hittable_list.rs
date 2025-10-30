use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();
            if object.hit(r, Interval::new(ray_t.min(), closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
