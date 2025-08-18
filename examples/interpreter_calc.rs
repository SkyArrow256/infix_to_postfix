use infix_to_rpn::{InfixExpression, RpnExpression, Token};
use std::io::{self, Write};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut infix = String::new();
        io::stdin().read_line(&mut infix).unwrap();
        let code: Result<RpnExpression, _> = InfixExpression::from(infix.as_str()).try_into();
        match code {
            Ok(code) => match calc(code) {
                Ok(num) => println!("{num}"),
                Err(e) => println!("{e:?}"),
            },
            Err(e) => println!("{e:?}"),
        }
    }
}

fn calc(code: RpnExpression) -> Result<i32, CalcErr> {
	let mut stack = Stack::new();
	for token in code.tokens() {
		match token {
			Token::UnarySub => stack.unary_sub(),
			Token::Add => stack.add(),
			Token::Sub => stack.sub(),
			Token::Mul => stack.mul(),
			Token::Div => stack.div(),
			Token::Mod => stack.modulo(),
			Token::Pow => stack.pow(),
			Token::Int(num) => stack.push(*num),
			Token::Exp(_exp) => (),
			_ => unreachable!(),
		}
	}
    Ok(stack.ret())
}

struct Stack {
    stack: [i32; 16],
    ptr: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: [0; 16],
            ptr: 0,
        }
    }
    fn push(&mut self, num: i32) {
        self.stack[self.ptr] = num;
        self.ptr += 1;
    }
    fn add(&mut self) {
        self.stack[self.ptr - 2] += self.stack[self.ptr - 1];
        self.ptr -= 1;
    }
    fn sub(&mut self) {
        self.stack[self.ptr - 2] -= self.stack[self.ptr - 1];
        self.ptr -= 1;
    }
    fn mul(&mut self) {
        self.stack[self.ptr - 2] *= self.stack[self.ptr - 1];
        self.ptr -= 1;
    }
    fn div(&mut self) {
        self.stack[self.ptr - 2] /= self.stack[self.ptr - 1];
        self.ptr -= 1;
    }
    fn modulo(&mut self) {
        self.stack[self.ptr - 2] %= self.stack[self.ptr - 1];
        self.ptr -= 1;
    }
    fn pow(&mut self) {
        self.stack[self.ptr - 2] = self.stack[self.ptr - 2].pow(self.stack[self.ptr - 1] as u32);
        self.ptr -= 1;
    }
	fn unary_sub(&mut self) {
		self.stack[self.ptr - 1] *= -1;
	}
    fn ret(&self) -> i32 {
        self.stack[0]
    }
}

#[derive(Debug)]
enum CalcErr {}
