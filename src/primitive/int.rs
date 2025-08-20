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