use std::io;
use std::rc::Rc;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::rtweekend::color::Color;
use crate::rtweekend::vec3::{Point3, Vec3};
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod rtweekend;
pub mod sphere;

fn main() -> io::Result<()> {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(sphere_material))));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0, 1, 0),
        1.0,
        Rc::new(material1),
    )));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(
        Point3::new(-4, 1, 0),
        1.0,
        Rc::new(material2),
    )));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Box::new(Sphere::new(
        Point3::new(4, 1, 0),
        1.0,
        Rc::new(material3),
    )));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13, 2, 3);
    cam.lookat = Point3::new(0, 0, 0);
    cam.vup = Vec3::new(0, 1, 0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world)?;

    Ok(())
}
