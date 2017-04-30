//! A 3 component vector

use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl fmt::Display for Vector3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[ {:.3}, {:.3}, {:.3} ]", self.x, self.y, self.z)
	}
}

impl PartialEq for Vector3 {
	fn eq(&self, other: &Vector3) -> bool {
		return
			self.x == other.x &&
			self.y == other.y &&
			self.z == other.z
	}
}

impl ops::Add for Vector3 {
	type Output = Vector3;

	fn add(self, other: Vector3) -> Vector3 {
		return Vector3::new(
			self.x + other.x,
			self.y + other.y,
			self.z + other.z
		);
	}
}

impl ops::Add<f64> for Vector3 {
	type Output = Vector3;

	fn add(self, other: f64) -> Vector3 {
		return Vector3::new(
			self.x + other,
			self.y + other,
			self.z + other
		);
	}
}

impl ops::Sub for Vector3 {
	type Output = Vector3;

	fn sub(self, other: Vector3) -> Vector3 {
		return Vector3::new(
			self.x - other.x,
			self.y - other.y,
			self.z - other.z
		);
	}
}

impl ops::Sub<f64> for Vector3 {
	type Output = Vector3;

	fn sub(self, other: f64) -> Vector3 {
		return Vector3::new(
			self.x - other,
			self.y - other,
			self.z - other
		);
	}
}

impl ops::Mul for Vector3 {
	type Output = Vector3;

	fn mul(self, other: Vector3) -> Vector3 {
		return Vector3::new(
			self.x * other.x,
			self.y * other.y,
			self.z * other.z
		);
	}
}

impl ops::Mul<f64> for Vector3 {
	type Output = Vector3;

	fn mul(self, other: f64) -> Vector3 {
		return Vector3::new(
			self.x * other,
			self.y * other,
			self.z * other
		);
	}
}

impl ops::Div for Vector3 {
	type Output = Vector3;

	fn div(self, other: Vector3) -> Vector3 {
		return Vector3::new(
			self.x / other.x,
			self.y / other.y,
			self.z / other.z
		);
	}
}

impl ops::Div<f64> for Vector3 {
	type Output = Vector3;

	fn div(self, other: f64) -> Vector3 {
		return Vector3::new(
			self.x / other,
			self.y / other,
			self.z / other
		);
	}
}

impl Vector3 {
	/// Construct a new vector
	///
	/// # Arguments
	///
	/// * `x` - The X component of the vector
	/// * `y` - The Y component of the vector
	/// * `z` - The Z component of the vector
	pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
		return Vector3 { x, y, z }
	}

	/// Construct an origin vector
	///
	/// # Remarks
	///
	/// X, Y, and Z components are set to 0.0.
	pub fn origin() -> Vector3 {
		return Vector3::new(0.0, 0.0, 0.0);
	}

	/// Construct a unit vector
	///
	/// # Remarks
	///
	/// X component is set to 1.0, Y and Z components are set to 0.0.
	pub fn unit_x() -> Vector3 {
		return Vector3::new(1.0, 0.0, 0.0);
	}

	/// Construct a unit vector
	///
	/// # Remarks
	///
	/// Y component is set to 1.0, X and Z components are set to 0.0.
	pub fn unit_y() -> Vector3 {
		return Vector3::new(0.0, 1.0, 0.0);
	}

	/// Construct a unit vector
	///
	/// # Remarks
	///
	/// Z component is set to 1.0, X and Y components are set to 0.0.
	pub fn unit_z() -> Vector3 {
		return Vector3::new(0.0, 0.0, 1.0);
	}

	/// Verify that vector is an origin vector
	pub fn is_origin(&self) -> bool {
		return
			self.x == 0.0 &&
			self.y == 0.0 &&
			self.z == 0.0
	}

	/// Normalize vector
	///
	/// # Remarks
	///
	/// Normalizing a vector means to scale its components up or down so that
	/// the length of the vector is exactly 1.0.
	pub fn normalize(&self) -> Vector3 {
		if self.is_origin() {
			return Vector3::origin();
		}

		return *self / self.len();
	}

	/// Trim vector
	///
	/// # Arguments
	///
	/// * `len` - Length vector should be scaled to
	pub fn trim(&self, len: f64) -> Vector3 {
		return self.normalize() * self.len().min(len);
	}

	/// Cross multiply two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to cross multiply
	pub fn cross(&self, other: Vector3) -> Vector3 {
		return Vector3::new(
			self.y * other.z - self.z * other.y,
			self.z * other.x - self.x * other.z,
			self.x * other.y - self.y * other.x
		);
	}

	/// Dot multiply two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to dot multiply
	pub fn dot(&self, other: Vector3) -> f64 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	/// Get length of a vector
	pub fn len(&self) -> f64 {
		return self.len2().sqrt();
	}

	/// Get square length of a vector
	pub fn len2(&self) -> f64 {
		return self.dot(*self);
	}

	/// Get distance between two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to get distance from
	pub fn dist(&self, other: Vector3) -> f64 {
		return self.dist2(other).sqrt();
	}

	/// Get square distance between two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to get distance from
	pub fn dist2(&self, other: Vector3) -> f64 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		let dz = self.z - other.z;
		return dx * dx + dy * dy + dz * dz;
	}

	/// Rotate a vector about an axis
	///
	/// # Arguments
	///
	/// * `angle` - The angle to rotate a vector by (in radians)
	/// * `axis` - The axis to rotate the vector on
	pub fn rotate(&self, angle: f64, axis: Vector3) -> Vector3 {
		let u = axis.normalize();
		let c = angle.cos();
		let s = angle.sin();

		let m1 = Vector3::new((c + u.x * u.x * (1.0 - c)),       (u.x * u.y * (1.0 - c) - u.z * s), (u.x * u.z * (1.0 - c) + u.y * s));
		let m2 = Vector3::new((u.y * u.x * (1.0 - c) + u.z * s), (c + u.y * u.y * (1.0 - c)),       (u.y * u.z * (1.0 - c) - u.x * s));
		let m3 = Vector3::new((u.z * u.x * (1.0 - c) - u.y * s), (u.z * u.y * (1.0 - c) + u.x * s), (c + u.z * u.z * (1.0 - c))      );

		return Vector3::new(
			self.dot(m1),
			self.dot(m2),
			self.dot(m3)
		);
	}

	/// Get the perpendicular vector of a vector
	pub fn perpendicular(&self) -> Vector3 {
		return Vector3::new(-self.y, self.x, 0.0);
	}

	/// Linear interpolation between two vectors
	///
	/// # Arguments
	///
	/// * `other` - The vector interpolating towards
	/// * `step` - Step between two vectors
	///
	/// # Remarks
	///
	/// The step is a percentage value where 0.0 is 0% and 1.0 is 100%. 0% would
	/// return the `self` vector, 100% would return the `other` vector, and a
	/// value in the middle would return a vector in the middle.
	pub fn lerp(&self, other: Vector3, step: f64) -> Vector3 {
		return *self + (other - *self) * step;
	}
}
