use crate::infix::InfixExpression;
use crate::tokens::{Assoc, Operator, Token};
use crate::{Expression, ParseError};

#[derive(Debug)]
pub struct PostfixExpression(Vec<Token>);

impl Expression for PostfixExpression {
	fn as_tokens(&self) -> &[crate::tokens::Token] {
		&self.0
	}
}

impl TryFrom<InfixExpression> for PostfixExpression {
	type Error = ParseError;
	fn try_from(value: InfixExpression) -> Result<Self, Self::Error> {
		let mut que = Vec::new();
		let mut stack: Vec<&Operator> = Vec::new();
		for token in value.as_tokens() {
			match token {
				//オペランドはそのままキューに入れる
				Token::Item(_) => que.push(token.clone()),
				//オペレータは優先度に合わせて分岐
				Token::Symbol(operator) => {
					if operator == &Operator::LeftParen {
					} else if operator == &Operator::RightParen {
						while let Some(token) = stack.pop() {
							//対応する括弧が見つかったら スタックから捨てて処理を終える
							if token == &Operator::LeftParen { break; }
							//そうでなければキューに追加
							que.push(Token::Symbol(token.clone()));
						}
						continue;
					} else if operator.assoc() == Assoc::Left {
						while !stack.is_empty() && stack.last().unwrap().prec() >= operator.prec() {
							que.push(Token::Symbol(stack.pop().unwrap().clone()));
						}
					} else {
						while !stack.is_empty() && stack.last().unwrap().prec() > operator.prec() {
							que.push(Token::Symbol(stack.pop().unwrap().clone()));
						}
					}
					stack.push(operator);
				}
			}
		}
		println!("{que:?}");
		println!("{stack:?}");
		for token in stack.into_iter().rev() {
			que.push(Token::Symbol(token.clone()));
		}
		Ok(Self(que))
        
	}
}