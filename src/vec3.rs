use std::ops::{
	Add,
	Sub,
	Mul,
	Div,
	AddAssign,
	SubAssign,
	MulAssign,
	DivAssign,
	Neg
};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64
}

pub type Point3 = Vec3;

impl Vec3 {
	pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
		Vec3 { 
			x: x.into(), 
			y: y.into(), 
			z: z.into()
		}
	}

	pub fn x(&self) -> f64 {
		self.x
	}

	pub fn y(&self) -> f64 {
		self.y
	}

	pub fn z(&self) -> f64 {
		self.z
	}

	pub fn length_squared(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn length(&self) -> f64 {
		f64::sqrt(self.length_squared())
	}

	pub fn dot(&self, other: &Vec3) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z 
	}

	pub fn cross(&self, other: &Vec3) -> Vec3 {
		Vec3::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x,
		)
	}

	pub fn unit_vector(&self) -> Vec3 {
		1.0 / self.length() * *self
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
				x: self.x + other.x,
				y: self.y + other.y,
				z: self.z + other.z,
		};
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z
		}
	}
}

impl<T: Into<f64>> MulAssign<T> for Vec3 {
	fn mul_assign(&mut self, scalar: T) {
		let scalar = scalar.into();
		*self = Self {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar
		}
	}
}

impl<T: Into<f64>> DivAssign<T> for Vec3 {
	fn div_assign(&mut self, scalar: T) {
		let scalar = scalar.into();
		*self = Self {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar
		}
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self {
		Self::new(-self.x, -self.y, -self.z)
	}
}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, other: Self) -> Self {
		Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
	}
}

impl<T: Into<f64>> Mul<T> for Vec3 {
	type Output = Self;

	fn mul(self, scalar: T) -> Self {
		let scalar: f64 = scalar.into();
		Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
	}
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self as f64
    }
}

impl<T: Into<f64>> Div<T> for Vec3 {
	type Output = Self;

	fn div(self, k: T) -> Self {
		let k: f64 = k.into();
		(1.0/k) * self
	}
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}