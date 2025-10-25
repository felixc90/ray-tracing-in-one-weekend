use std::io;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::rtweekend::vec3::Point3;
use crate::sphere::Sphere;

pub mod hittable;
pub mod hittable_list;
pub mod sphere;
pub mod rtweekend;
pub mod camera;

fn main() -> io::Result<()> {
	let mut world = HittableList::new();

	world.add(Box::new(Sphere::new(Point3::new(0,0,-1), 0.5)));
	world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

	let mut cam = Camera::default();
	cam.render(&world)?;

	Ok(())
}