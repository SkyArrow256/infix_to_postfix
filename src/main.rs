use std::io::{self, Write};
use infix_to_postfix::{Calc, InfixExpression, PostfixExpression, Primitive};

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
			Ok(result) => match result {
				Primitive::Integer(i) => println!("\x1b[1m{i}\x1b[0m"),
				Primitive::Boolean(b) => println!("\x1b[1;32m{b}\x1b[0m") 
			},
			Err(e) => println!("\x1b[1;31m{e:?}\x1b[0m"),
		}
	}
}