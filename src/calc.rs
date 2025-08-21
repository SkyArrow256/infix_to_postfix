use crate::{primitive::{Bool, Int, Primitive, Type}, tokens::{Operand, Operator, Token}, Expression, PostfixExpression};

/// 計算機
pub struct Calc {

}

impl Calc {
	pub fn new() -> Self {
		Self {  }
	}
	pub fn run(&mut self, code: &PostfixExpression) -> Result<Primitive, EvalError> {
		let mut stack = Vec::new();
		for token in code.as_tokens() {
			match token {
				//オペランドはそのままスタックに
				Token::Item(operand) => {
					match operand {
						Operand::Integer(int) => stack.push(Primitive::Integer(int.clone())),
						Operand::Boolean(bool) => stack.push(Primitive::Boolean(bool.clone())),
						Operand::Variable => todo!(),
					}
				}
				//オペレータはそれぞれ計算
				Token::Symbol(operator) => {
					match operator {
						Operator::LeftParen => unreachable!(),
						Operator::RightParen => unreachable!(),
						Operator::Assign => todo!(),
						Operator::Add => {
							//スタックからポップ
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							//どちらもInt型だったとき、
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								//加算してプッシュ
								stack.push(Primitive::Integer(a + b));
							} else {
								//数値でなければTypeError
								return Err(EvalError::TypeError);
							}
						}
						Operator::Sub => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Integer(a - b));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Mul => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Integer(a * b));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Div => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Integer(a / b));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Mod => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Integer(a % b));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Neg => {
							if let Primitive::Integer(i) = stack.last_mut().unwrap() {
								*i.get_mut() *= -1;
							} else {
								return Err(EvalError::TypeError)
							}
						}
						Operator::Pow => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								let result = a.eval().pow(b.eval() as u32);
								stack.push(Primitive::Integer(Int::new(result)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Eq => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Boolean(Bool::new(a == b)));
							} else if let Primitive::Boolean(a) = a && let Primitive::Boolean(b) = b {
								stack.push(Primitive::Boolean(Bool::new(a == b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::NotEq => {
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Boolean(Bool::new(a != b)));
							} else if let Primitive::Boolean(a) = a && let Primitive::Boolean(b) = b {
								stack.push(Primitive::Boolean(Bool::new(a != b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Greater => {
							if let (Primitive::Integer(b), Primitive::Integer(a)) = (stack.pop().unwrap(), stack.pop().unwrap()) {
								stack.push(Primitive::Boolean(Bool::new(a > b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Less => {
							if let (Primitive::Integer(b), Primitive::Integer(a)) = (stack.pop().unwrap(), stack.pop().unwrap()) {
								stack.push(Primitive::Boolean(Bool::new(a < b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::GreaterEq => {
							if let (Primitive::Integer(b), Primitive::Integer(a)) = (stack.pop().unwrap(), stack.pop().unwrap()) {
								stack.push(Primitive::Boolean(Bool::new(a >= b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::LessEq => {
							if let (Primitive::Integer(b), Primitive::Integer(a)) = (stack.pop().unwrap(), stack.pop().unwrap()) {
								stack.push(Primitive::Boolean(Bool::new(a <= b)));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Not => {
							if let Primitive::Boolean(i) = stack.last_mut().unwrap() {
								*i.get_mut() = !i.eval();
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::And => {
							if let Primitive::Boolean(b) = stack.pop().unwrap() && let Primitive::Boolean(a) = stack.pop().unwrap() {
								stack.push(Primitive::Boolean(Bool::new(a.eval() && b.eval())));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Or => {
							if let Primitive::Boolean(b) = stack.pop().unwrap() && let Primitive::Boolean(a) = stack.pop().unwrap() {
								stack.push(Primitive::Boolean(Bool::new(a.eval() || b.eval())));
							} else {
								return Err(EvalError::TypeError);
							}
						}
					}
				}
			}
		}
		Ok(stack.pop().ok_or(EvalError::NoneReturnValue)?)
	}
}

#[derive(Debug)]
pub enum EvalError {
	TypeError,
	NoneReturnValue
}