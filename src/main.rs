pub mod color;
pub mod vec3;
pub mod ray;

use std::{fs::File, io::{self, Write}};
use color::{Color, write_color};
use ray::{Ray};
use vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
	let oc = center - r.origin();
	let a = r.direction().length_squared();
	let h = r.direction().dot(&oc);
	let c = oc.length_squared() - radius * radius;
	let discriminant = h * h - a * c;

	if discriminant < 0.0 {
		return -1.0;
	}

	(h - f64::sqrt(discriminant)) / a
}

fn ray_color(r: Ray) -> Color {
	let t = hit_sphere(Point3::new(0,0,-1), 0.5, &r);
	if t > 0.0 {
		let n = (r.at(t) - Vec3::new(0,0,-1)).unit_vector();
    return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
	}
	let unit_direction = r.direction().unit_vector();
	let a = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> io::Result<()> {

	// Image
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;

	// Calculate the image height, and ensure that it's at least 1.
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let image_height = if image_height < 1 { 1 } else { image_height };

	// Camera
	let focal_length = 1.0;
	let viewport_height = 2.0;
	let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
	let camera_center = Point3::new(0, 0, 0);

	// Calculate the vectors across the horizontal and down the vertical viewport edges.
	let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
	let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

	// Calculate the horizontal and vertical delta vectors from pixel to pixel.
	let pixel_delta_u = viewport_u / image_width;
	let pixel_delta_v = viewport_v / image_height;

	// Calculate the location of the upper left pixel.
	let viewport_upper_left = 
		camera_center - Vec3::new(0.0, 0.0, focal_length) - 0.5 * viewport_u - 0.5 * viewport_v;
	let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

	// Render
	
	let mut out = File::create("output.ppm")?;
	// let mut out = io::stdout();
	writeln!(out, "P3\n{} {}\n255\n", image_width, image_height)?;

	for j in 0..image_height {
		for i in 0..image_width {
			let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
			let r = Ray::new(camera_center, pixel_center - camera_center);
			let pixel_color = ray_color(r);
      write_color(&mut out, pixel_color)?;
		}
	}

	Ok(())
}