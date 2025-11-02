use std::io;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::{Point3, Vec3};
use crate::sphere::Sphere;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod rtweekend;
pub mod sphere;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        Rc::new(material_bubble),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2, 2, 1);
    cam.lookat = Point3::new(0, 0, -1);
    cam.vup = Vec3::new(0, 1, 0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world)?;

    Ok(())
}
