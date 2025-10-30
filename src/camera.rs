use std::{
    f64,
    fs::File,
    io::{self, Write},
};

use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::color::{Color, write_color};
use crate::rtweekend::interval::Interval;
use crate::rtweekend::random_double;
use crate::rtweekend::ray::Ray;
use crate::rtweekend::vec3::{Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    image_height: i32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render<T: Hittable>(&mut self, world: &T) -> io::Result<()> {
        self.initialize();

        let mut out = File::create("image.ppm")?;
        // let mut out = io::stdout();
        writeln!(out, "P3\n{} {}\n255\n", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                write_color(&mut out, self.pixel_samples_scale * pixel_color)?;
            }
        }
        Ok(())
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = Point3::new(0, 0, 0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color<T: Hittable>(r: &Ray, depth: i32, world: &T) -> Color {
        if depth <= 0 {
            return Color::new(0, 0, 0);
        }

        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .mat
                .as_ref()
                .is_some_and(|m| m.scatter(&r, &rec, &mut attenuation, &mut scattered))
            {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0, 0, 0);
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            pixel_samples_scale: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}
