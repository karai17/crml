//! A 2 component vector

use std::fmt;
use std::ops;
use std::f64::consts;

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
	pub x: f64,
	pub y: f64
}

impl fmt::Display for Vector2 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[ {:.3}, {:.3} ]", self.x, self.y)
	}
}

impl PartialEq for Vector2 {
	fn eq(&self, other: &Vector2) -> bool {
		return
			self.x == other.x &&
			self.y == other.y
	}
}

impl ops::Add for Vector2 {
	type Output = Vector2;

	fn add(self, other: Vector2) -> Vector2 {
		return Vector2::new(
			self.x + other.x,
			self.y + other.y
		);
	}
}

impl ops::Add<f64> for Vector2 {
	type Output = Vector2;

	fn add(self, other: f64) -> Vector2 {
		return Vector2::new(
			self.x + other,
			self.y + other
		);
	}
}

impl ops::Sub for Vector2 {
	type Output = Vector2;

	fn sub(self, other: Vector2) -> Vector2 {
		return Vector2::new(
			self.x - other.x,
			self.y - other.y
		);
	}
}

impl ops::Sub<f64> for Vector2 {
	type Output = Vector2;

	fn sub(self, other: f64) -> Vector2 {
		return Vector2::new(
			self.x - other,
			self.y - other
		);
	}
}

impl ops::Mul for Vector2 {
	type Output = Vector2;

	fn mul(self, other: Vector2) -> Vector2 {
		return Vector2::new(
			self.x * other.x,
			self.y * other.y
		);
	}
}

impl ops::Mul<f64> for Vector2 {
	type Output = Vector2;

	fn mul(self, other: f64) -> Vector2 {
		return Vector2::new(
			self.x * other,
			self.y * other
		);
	}
}

impl ops::Div for Vector2 {
	type Output = Vector2;

	fn div(self, other: Vector2) -> Vector2 {
		return Vector2::new(
			self.x / other.x,
			self.y / other.y
		);
	}
}

impl ops::Div<f64> for Vector2 {
	type Output = Vector2;

	fn div(self, other: f64) -> Vector2 {
		return Vector2::new(
			self.x / other,
			self.y / other
		);
	}
}

impl Vector2 {
	/// Construct a new vector
	///
	/// # Arguments
	///
	/// * `x` - The X component of the vector
	/// * `y` - The Y component of the vector
	pub fn new(x: f64, y: f64) -> Vector2 {
		return Vector2 { x, y }
	}

	/// Construct an origin vector
	///
	/// # Remarks
	///
	/// X and Y components are set to 0.0.
	pub fn origin() -> Vector2 {
		return Vector2::new(0.0, 0.0);
	}

	/// Construct a unit vector
	///
	/// # Remarks
	///
	/// X component is set to 1.0, Y component is set to 0.0.
	pub fn unit_x() -> Vector2 {
		return Vector2::new(1.0, 0.0);
	}

	/// Construct a unit vector
	///
	/// # Remarks
	///
	/// Y component is set to 1.0, X component is set to 0.0.
	pub fn unit_y() -> Vector2 {
		return Vector2::new(0.0, 1.0);
	}

	/// Construct a new vector from polar to cartesian
	pub fn from_polar(radius: f64, angle: f64) -> Vector2 {
		return Vector2::new(
			radius * angle.cos(),
			radius * angle.sin()
		);
	}

	/// Verify that vector is an origin vector
	pub fn is_origin(&self) -> bool {
		return
			self.x == 0.0 &&
			self.y == 0.0
	}

	/// Normalize vector
	///
	/// # Remarks
	///
	/// Normalizing a vector means to scale its components up or down so that
	/// the length of the vector is exactly 1.0.
	pub fn normalize(&self) -> Vector2 {
		if self.is_origin() {
			return Vector2::origin();
		}

		return *self / self.len();
	}

	/// Trim vector
	///
	/// # Arguments
	///
	/// * `len` - Length vector should be scaled to
	pub fn trim(&self, len: f64) -> Vector2 {
		return self.normalize() * self.len().min(len);
	}

	/// Cross multiply two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to cross multiply
	pub fn cross(&self, other: Vector2) -> f64 {
		return self.x * other.y - self.y * other.x;
	}

	/// Dot multiply two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to dot multiply
	pub fn dot(&self, other: Vector2) -> f64 {
		return self.x * other.x + self.y * other.y;
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
	pub fn dist(&self, other: Vector2) -> f64 {
		return self.dist2(other).sqrt();
	}

	/// Get square distance between two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to get distance from
	pub fn dist2(&self, other: Vector2) -> f64 {
		let dx = self.x - other.x;
		let dy = self.y - other.y;
		return dx * dx + dy * dy;
	}

	/// Rotate a vector about an axis
	///
	/// # Arguments
	///
	/// * `angle` - The angle to rotate a vector by (in radians)
	pub fn rotate(&self, angle: f64) -> Vector2 {
		let c = angle.cos();
		let s = angle.sin();

		return Vector2::new(
			c * self.x - s * self.y,
			s * self.x + c * self.y
		);
	}

	/// Get the perpendicular vector of a vector
	pub fn perpendicular(&self) -> Vector2 {
		return Vector2::new(-self.y, self.x);
	}

	/// Angle from one vector to another
	///
	/// # Arguments
	///
	/// * `other` - A second vector to get angle to
	pub fn angle_to(&self, other: Vector2) -> f64 {
		return (self.y - other.y).atan2(self.x - other.x);
	}

	/// Angle between two vectors
	///
	/// # Arguments
	///
	/// * `other` - A second vector to get angle between
	pub fn angle_between(&self, other: Vector2) -> f64 {
		return (self.dot(other) / (self.len() * other.len())).acos();
	}

	/// Deconstruct vector from cartesian to polar
	pub fn to_polar(&self) -> (f64, f64) {
		let radius    = self.len();
		let mut angle = self.y.atan2(self.x);

		if angle <= 0.0 {
			angle += 2.0 * consts::PI;
		}

		return (radius, angle);
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
	pub fn lerp(&self, other: Vector2, step: f64) -> Vector2 {
		return *self + (other - *self) * step;
	}
}
