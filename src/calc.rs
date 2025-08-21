use crate::{primitive::Primitive, tokens::{Operand, Operator, Token}, Expression, PostfixExpression};

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
						Operand::Integer(int) => stack.push(crate::primitive::Primitive::Integer(int.clone())),
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
							let (b, a) = (stack.pop().unwrap(), stack.pop().unwrap());
							if let Primitive::Integer(a) = a && let Primitive::Integer(b) = b {
								stack.push(Primitive::Integer(a + b));
							} else {
								return Err(EvalError::TypeError);
							}
						}
						Operator::Sub => todo!(),
						Operator::Mul => todo!(),
						Operator::Div => todo!(),
						Operator::Mod => todo!(),
						Operator::Neg => todo!(),
						Operator::Pow => todo!(),
						Operator::Eq => todo!(),
						Operator::NotEq => todo!(),
						Operator::Greater => todo!(),
						Operator::Less => todo!(),
						Operator::GreaterEq => todo!(),
						Operator::LessEq => todo!(),
						Operator::Not => todo!(),
						Operator::And => todo!(),
						Operator::Or => todo!(),
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