use crate::rtweekend::{interval::{Interval}, vec3::Vec3};

pub type Color = Vec3;
use std::io::{self, Write};

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	if linear_component > 0.0 {
		return linear_component.sqrt();
	}
	0.0
}

pub fn write_color<W: Write>(out: &mut W, color: Color) -> io::Result<()> {
	let mut r = color.x();
	let mut g = color.y();
	let mut b = color.z();

	    // Apply a linear to gamma transform for gamma 2
	r = linear_to_gamma(r);
	g = linear_to_gamma(g);
	b = linear_to_gamma(b);

	let intensity = Interval::new(0.0, 0.999);
	let rbyte = (256.0 * intensity.clamp(r)) as i32;
	let gbyte = (256.0 * intensity.clamp(g)) as i32;
	let bbyte = (256.0 * intensity.clamp(b)) as i32;

	writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

	Ok(())
}