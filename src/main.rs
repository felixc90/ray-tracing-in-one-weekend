pub mod math;
use math::Vec3;

fn main() {
  println!("{}", Vec3::new(0, 1, 1) * Vec3::new(1, 1, 1));
	println!("{}",  5.0 * Vec3::new(1, 1, 1));
	println!("{}",  5 * Vec3::new(1, 1, 1));
	println!("{}",  Vec3::new(1, 1, 1) * 5);
	println!("{}",  Vec3::new(1, 1, 1) * 5.0);
	println!("{}",  Vec3::new(1, 1, 1) / 5);
	println!("{}",  Vec3::new(1, 1, 1) / 5.0);
	println!("{}",  Vec3::new(1, 1, 1).dot(Vec3::new(1, 1, 1)));
	println!("{}",  Vec3::new(1, 0, 0).cross(Vec3::new(0, 0, 1)));
	println!("{}",  Vec3::new(1, 0, 0).unit_vector());
	println!("{}",  Vec3::new(1, 1, 0).length());
	println!("{}",  Vec3::new(1, 1, 0).length_squared());
	println!("{}",  Vec3::new(1, 0, 0).unit_vector());
	println!("{}",  Vec3::new(1, 1, 0).unit_vector());
	let mut v = Vec3::new(0,0,0);
	println!("{}", v);
	v += Vec3::new(1,1,1);
	println!("{}", v);
	v /= -2.0;
	println!("{}", v);
	v *= 10;
	println!("{}", v);
}