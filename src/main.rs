use infix_to_postfix::{InfixExpression, PostfixExpression};
use std::io::{self, Write};

fn main() {
	loop {
		print!("> ");
		io::stdout().flush().unwrap();
		let mut str = String::new();
		io::stdin().read_line(&mut str).unwrap();
		let infix = InfixExpression::from(str.as_str());
		println!("{infix:?}");
		let postfix: PostfixExpression = infix.try_into().unwrap();
		println!("{postfix:?}");
	}
}