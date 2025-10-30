use std::io;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::rtweekend::vec3::Point3;
use crate::sphere::Sphere;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod lambertian;
pub mod material;
pub mod rtweekend;
pub mod sphere;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world)?;

    Ok(())
}
