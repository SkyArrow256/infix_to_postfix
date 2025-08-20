use crate::tokens::{Operand, Operator, Token};
use crate::{primitive::{self, Type}, Expression};

#[derive(Debug)]
pub struct InfixExpression(Vec<Token>);

impl Expression for InfixExpression {
	fn as_tokens(&self) -> &[crate::tokens::Token] {
		&self.0
	}
}

impl From<&str> for InfixExpression {
	fn from(string: &str) -> Self {
		let mut tokens = Vec::new();
		//OrとAndのためのカウンター（手抜き）
		let mut logic_counter = (0, 0);
		for c in string.chars().filter(|c| !c.is_whitespace()) {
			match c {
				'(' => tokens.push(Token::Symbol(Operator::LeftParen)),
				')' => tokens.push(Token::Symbol(Operator::RightParen)),
				'|' => {
					logic_counter.0 += 1;
					if logic_counter.0 == 2 {
						tokens.push(Token::Symbol(Operator::Or));
						logic_counter.0 = 0;
					}
				}
				'&' => {
					logic_counter.1 += 1;
					if logic_counter.1 == 2 {
						tokens.push(Token::Symbol(Operator::And));
						logic_counter.1 = 0;
					}
				}
				'=' => {
					if let Some(last) = tokens.last() && let Token::Symbol(op) = last {
						match op {
								Operator::Assign => {
									tokens.pop();
									tokens.push(Token::Symbol(Operator::Eq));
								}
								Operator::Greater => {
									tokens.pop();
									tokens.push(Token::Symbol(Operator::GreaterEq));
								}
								Operator::Less => {
									tokens.pop();
									tokens.push(Token::Symbol(Operator::LessEq));	
								}
								Operator::Not => {
									tokens.pop();
									tokens.push(Token::Symbol(Operator::NotEq));
								}
								_ => panic!("この演算子はイコールの直前に置けません"),
							}
					} else {
						tokens.push(Token::Symbol(Operator::Assign));
					}
				}
				'>' => tokens.push(Token::Symbol(Operator::Greater)),
				'<' => tokens.push(Token::Symbol(Operator::Less)),
				'+' => tokens.push(Token::Symbol(Operator::Add)),
				'-' => {
					//この-の前にトークンがあって、
					if let Some(token) = tokens.last() {
						if let Token::Symbol(token) = token {
							//それが右括弧だったとき、減算
							if *token == Operator::RightParen {
								tokens.push(Token::Symbol(Operator::Sub));
								continue;
							}
						} else {
							//それが記号じゃなかった時、減算
							tokens.push(Token::Symbol(Operator::Sub));
							continue;
						}
					}
					//最初のトークンだったり、直前が)以外の演算子だった時、マイナス
					tokens.push(Token::Symbol(Operator::Neg));
				}
				'*' => tokens.push(Token::Symbol(Operator::Mul)),
				'/' => tokens.push(Token::Symbol(Operator::Div)),
				'%' => tokens.push(Token::Symbol(Operator::Mod)),
				'^' => tokens.push(Token::Symbol(Operator::Pow)), 
				'!' => tokens.push(Token::Symbol(Operator::Not)),
				c @ _ if c.is_ascii_digit() => {
					//数値だったとき、10進数なので元の数に10かけてから足す
					let num = c.to_digit(10).unwrap() as i32;
					if let Some(Token::Item(Operand::Integar(i))) = tokens.last_mut() {
						let i = i.get_mut();
						*i *= 10; *i += num;
					} else {
						tokens.push(Token::Item(Operand::Integar(primitive::Int::new(num))));
					}
				}
				_ => todo!("未実装の文字です"),
			}
			//&や|のカウンタは本当に手抜き工事なので要修正！
		}
		Self(tokens)
	}
}