extern crate crml;
use crml::vector3::Vector3;
use std::f64::consts;

#[test]
fn create() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let o = Vector3::origin();
	let x = Vector3::unit_x();
	let y = Vector3::unit_y();
	let z = Vector3::unit_z();

	assert_eq!(a.x, 1.0);
	assert_eq!(a.y, 2.0);
	assert_eq!(a.z, 3.0);

	assert_eq!(o.x, 0.0);
	assert_eq!(o.y, 0.0);
	assert_eq!(o.z, 0.0);

	assert_eq!(x.x, 1.0);
	assert_eq!(x.y, 0.0);
	assert_eq!(x.z, 0.0);

	assert_eq!(y.x, 0.0);
	assert_eq!(y.y, 1.0);
	assert_eq!(y.z, 0.0);

	assert_eq!(z.x, 0.0);
	assert_eq!(z.y, 0.0);
	assert_eq!(z.z, 1.0);
}

#[test]
fn fmt() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	assert_eq!(a.to_string(), "[ 1.000, 2.000, 3.000 ]");
}

#[test]
fn eq() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::new(1.0, 2.0, 3.0);
	let c = Vector3::new(1.0, 2.0, 4.0);

	assert_eq!(a == b, true);
	assert_eq!(a == c, false);
}

#[test]
fn add() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::new(3.0, 2.0, 1.0);
	let c = Vector3::new(4.0, 4.0, 4.0);
	let d = Vector3::new(3.0, 4.0, 5.0);
	let s = 2.0;

	assert_eq!(a + b, c);
	assert_eq!(a + s, d);
}

#[test]
fn sub() {
	let a = Vector3::new( 1.0, 2.0, 3.0);
	let b = Vector3::new( 3.0, 2.0, 1.0);
	let c = Vector3::new(-2.0, 0.0, 2.0);
	let d = Vector3::new(-1.0, 0.0, 1.0);
	let s = 2.0;

	assert_eq!(a - b, c);
	assert_eq!(a - s, d);
}

#[test]
fn mul() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::new(3.0, 2.0, 1.0);
	let c = Vector3::new(3.0, 4.0, 3.0);
	let d = Vector3::new(2.0, 4.0, 6.0);
	let s = 2.0;

	assert_eq!(a * b, c);
	assert_eq!(a * s, d);
}

#[test]
fn div() {
	let a = Vector3::new(    1.0, 2.0, 3.0);
	let b = Vector3::new(    3.0, 2.0, 1.0);
	let c = Vector3::new(1.0/3.0, 1.0, 3.0);
	let d = Vector3::new(    0.5, 1.0, 1.5);
	let s = 2.0;

	assert_eq!(a / b, c);
	assert_eq!(a / s, d);
}

#[test]
fn is_origin() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::origin();

	assert_eq!(a.is_origin(), false);
	assert_eq!(b.is_origin(), true);
}

#[test]
fn normalize() {
	let a = Vector3::new(1.0, 2.0, 3.0).normalize();
	let b = Vector3::origin().normalize();

	assert_eq!(a.len(), 1.0);
	assert_eq!(b.len(), 0.0);
}

#[test]
fn trim() {
	let a = Vector3::new(1.0, 2.0, 3.0).trim(0.5);
	assert_eq!(a.len(), 0.5);
}

#[test]
fn cross() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::new(3.0, 2.0, 1.0);
	let c = a.cross(b);

	assert_eq!(c.x, -4.0);
	assert_eq!(c.y,  8.0);
	assert_eq!(c.z, -4.0);
}

#[test]
fn dot() {
	let a = Vector3::new(1.0, 2.0, 3.0);
	let b = Vector3::new(3.0, 2.0, 1.0);
	assert_eq!(a.dot(b), 10.0);
}

#[test]
fn len() {
	let a = Vector3::unit_x();
	assert_eq!(a.len(), 1.0);
}

#[test]
fn len2() {
	let a = Vector3::unit_x();
	assert_eq!(a.len2(), 1.0);
}

#[test]
fn dist() {
	let a = Vector3::unit_x();
	assert_eq!(a.dist(a), 0.0);
}

#[test]
fn dist2() {
	let a = Vector3::unit_x();
	assert_eq!(a.dist2(a), 0.0);
}

#[test]
fn rotate() {
	let a = Vector3::unit_x().rotate(consts::PI, Vector3::unit_z());
	let b = Vector3::unit_x() * -1.0;
	assert_eq!(a.x, b.x);
}

#[test]
fn perpendicular() {
	let a = Vector3::new( 1.0, 2.0, 3.0);
	let b = Vector3::new(-2.0, 1.0, 0.0);
	assert_eq!(a.perpendicular(), b);
}

#[test]
fn lerp() {
	let a = Vector3::unit_x();
	let b = Vector3::unit_y();
	let c = Vector3::new(0.5, 0.5, 0.0);
	assert_eq!(a.lerp(b, 0.5), c);
}
