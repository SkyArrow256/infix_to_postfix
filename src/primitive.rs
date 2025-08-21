mod int;
mod bool;

pub use self::int::Int;
pub use self::bool::Bool;

pub trait Type {
	type T;
	fn new(item: Self::T) -> Self;
	fn eval(&self) -> Self::T;
	fn get_mut(&mut self) -> &mut Self::T;
}

#[derive(Debug)]
pub enum Primitive {
	Integer(Int),
	Boolean(Bool),
}