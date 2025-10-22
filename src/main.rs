pub mod color;
pub mod vec3;
pub mod ray;
use std::io::{self, Write};

use color::{Color, write_color};

fn main() {

	let image_height = 256;
	let image_width = 256;

	let mut stdout = io::stdout();
	if let Err(e) = writeln!(stdout, "P3\n{} {}\n255\n", image_width, image_height) {
		eprintln!("Couldn't write to file: {}", e);
	}

	for j in 0..image_height {
		for i in 0..image_width {
			let pixel_color = Color::new(
				(i as f64)/(image_width - 1) as f64, 
				(j as f64)/(image_height - 1) as f64, 
				0.0);
      write_color(&mut stdout, pixel_color)
		}
	}
}