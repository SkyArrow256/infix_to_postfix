use std::io::{self, Write};
use infix_to_postfix::{InfixExpression, PostfixExpression, Calc};

fn main() {
	let mut input = String::new();
	let mut calculator = Calc::new();
	loop {
		input.clear();
		print!("> "); io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).unwrap();
		let infix = InfixExpression::from(input.as_str());
		println!("{infix:?}");
		let postfix = PostfixExpression::try_from(infix).unwrap();
		println!("{postfix:?}");
		match calculator.run(&postfix) {
			Ok(result) =>println!("{result:?}"),
			Err(e) => println!("{e:?}"),
		}
	}
}