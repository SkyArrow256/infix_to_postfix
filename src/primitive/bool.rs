use super::Type;

#[derive(Debug, Clone, PartialEq)]
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