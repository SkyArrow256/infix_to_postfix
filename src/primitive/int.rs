use std::ops::{Add, Div, Mul, Sub};

use super::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Int(i32);

impl Type for Int {
	type T = i32;
	fn eval(&self) -> Self::T {
		self.0
	}
	fn new(item: Self::T) -> Self {
		Self(item)
	}
	fn get_mut(&mut self) -> &mut Self::T {
		&mut self.0
	}
}

impl Add for Int {
	type Output = Int;
	fn add(self, rhs: Self) -> Self::Output {
		Int(self.0 + rhs.0)
	}
}

impl Sub for Int {
	type Output = Int;
	fn sub(self, rhs: Self) -> Self::Output {
		Int(self.0 - rhs.0)
	}
}

impl Mul for Int {
	type Output = Int;
	fn mul(self, rhs: Self) -> Self::Output {
		Int(self.0 * rhs.0)
	}
}

impl Div for Int {
	type Output = Int;
	fn div(self, rhs: Self) -> Self::Output {
		Int(self.0 / rhs.0)
	}
}