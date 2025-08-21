use std::fmt::Display;

use super::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bool(bool);

impl Type for Bool {
	type T = bool;
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

impl Display for Bool {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}