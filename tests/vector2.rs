extern crate crml;
use crml::vector2::Vector2;
use std::f64::consts;

#[test]
fn create() {
	let a = Vector2::new(1.0, 2.0);
	let o = Vector2::origin();
	let x = Vector2::unit_x();
	let y = Vector2::unit_y();
	let c = Vector2::from_polar(1.0, consts::PI);

	assert_eq!(a.x, 1.0);
	assert_eq!(a.y, 2.0);

	assert_eq!(o.x, 0.0);
	assert_eq!(o.y, 0.0);

	assert_eq!(x.x, 1.0);
	assert_eq!(x.y, 0.0);

	assert_eq!(y.x, 0.0);
	assert_eq!(y.y, 1.0);

	assert_eq!(c.x, -1.0);
	assert!(c.y.abs() <= 2.2204460492503131e-16);
}

#[test]
fn fmt() {
	let a = Vector2::new(1.0, 2.0);
	assert_eq!(a.to_string(), "[ 1.000, 2.000 ]");
}

#[test]
fn eq() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(1.0, 2.0);
	let c = Vector2::new(1.0, 3.0);

	assert_eq!(a == b, true);
	assert_eq!(a == c, false);
}

#[test]
fn add() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(2.0, 1.0);
	let c = Vector2::new(3.0, 3.0);
	let d = Vector2::new(3.0, 4.0);
	let s = 2.0;

	assert_eq!(a + b, c);
	assert_eq!(a + s, d);
}

#[test]
fn sub() {
	let a = Vector2::new( 1.0, 2.0);
	let b = Vector2::new( 2.0, 1.0);
	let c = Vector2::new(-1.0, 1.0);
	let d = Vector2::new(-1.0, 0.0);
	let s = 2.0;

	assert_eq!(a - b, c);
	assert_eq!(a - s, d);
}

#[test]
fn mul() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(2.0, 1.0);
	let c = Vector2::new(2.0, 2.0);
	let d = Vector2::new(2.0, 4.0);
	let s = 2.0;

	assert_eq!(a * b, c);
	assert_eq!(a * s, d);
}

#[test]
fn div() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(2.0, 1.0);
	let c = Vector2::new(0.5, 2.0);
	let d = Vector2::new(0.5, 1.0);
	let s = 2.0;

	assert_eq!(a / b, c);
	assert_eq!(a / s, d);
}

#[test]
fn is_origin() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::origin();

	assert_eq!(a.is_origin(), false);
	assert_eq!(b.is_origin(), true);
}

#[test]
fn normalize() {
	let a = Vector2::new(1.0, 2.0).normalize();
	let b = Vector2::origin().normalize();

	assert!((a.len() - 1.0).abs() <= 2.2204460492503131e-16);
	assert_eq!(b.len(), 0.0);
}

#[test]
fn trim() {
	let a = Vector2::new(1.0, 2.0).trim(0.5);
	assert!((a.len() - 0.5).abs() <= 2.2204460492503131e-16);
}

#[test]
fn cross() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(2.0, 1.0);
	let c = a.cross(b);
	assert_eq!(c, -3.0);
}

#[test]
fn dot() {
	let a = Vector2::new(1.0, 2.0);
	let b = Vector2::new(2.0, 1.0);
	assert_eq!(a.dot(b), 4.0);
}

#[test]
fn len() {
	let a = Vector2::unit_x();
	assert_eq!(a.len(), 1.0);
}

#[test]
fn len2() {
	let a = Vector2::unit_x();
	assert_eq!(a.len2(), 1.0);
}

#[test]
fn dist() {
	let a = Vector2::unit_x();
	assert_eq!(a.dist(a), 0.0);
}

#[test]
fn dist2() {
	let a = Vector2::unit_x();
	assert_eq!(a.dist2(a), 0.0);
}

#[test]
fn rotate() {
	let a = Vector2::unit_x().rotate(consts::PI);
	let b = Vector2::unit_x() * -1.0;
	assert_eq!(a.x, b.x);
}

#[test]
fn perpendicular() {
	let a = Vector2::new( 1.0, 2.0);
	let b = Vector2::new(-2.0, 1.0);
	assert_eq!(a.perpendicular(), b);
}

#[test]
fn angle_to() {
	let a = Vector2::unit_x();
	let b = Vector2::unit_x() * -1.0;
	assert_eq!(a.angle_to(b), 0.0);
}

#[test]
fn angle_between() {
	let a = Vector2::unit_x();
	let b = Vector2::unit_y() * -1.0;
	assert_eq!(a.angle_between(b), consts::PI / 2.0);
}

#[test]
fn to_polar() {
	let a = Vector2::from_polar(1.0, consts::PI);
	let (radius, angle) = a.to_polar();

	assert_eq!(radius, 1.0);
	assert_eq!(angle,  consts::PI);
}

#[test]
fn lerp() {
	let a = Vector2::unit_x();
	let b = Vector2::unit_y();
	let c = Vector2::new(0.5, 0.5);
	assert_eq!(a.lerp(b, 0.5), c);
}
