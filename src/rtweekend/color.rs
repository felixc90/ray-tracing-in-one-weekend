use crate::rtweekend::{interval::{Interval}, vec3::Vec3};

pub type Color = Vec3;
use std::io::{self, Write};

pub fn write_color<W: Write>(out: &mut W, color: Color) -> io::Result<()> {
	let r = color.x();
	let g = color.y();
	let b = color.z();

	let intensity = Interval::new(0.0, 0.999);
	let rbyte = (256.0 * intensity.clamp(r)) as i32;
	let gbyte = (256.0 * intensity.clamp(g)) as i32;
	let bbyte = (256.0 * intensity.clamp(b)) as i32;

	writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;

	Ok(())
}