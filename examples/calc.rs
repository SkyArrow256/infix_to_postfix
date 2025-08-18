use std::io::{self, Write};

fn main() {
    loop {
        print!("> "); io::stdout().flush().unwrap();
        let mut infix = String::new();
        io::stdin().read_line(&mut infix).unwrap();
        let rpn = infix_to_rpn::to_rpn(&infix);
        match rpn {
            Ok(str) => println!("{}", calc(&str).unwrap()),
            Err(e) => println!("{e:?}"),
        }
    }
}

fn calc(exp: &str) -> Option<i32> {
    let mut stack = Stack::new();
    for elem in exp.split_whitespace() {
        let c = elem.chars().next().unwrap();
        if c.is_ascii_digit() {
            stack.push(elem.parse().unwrap());
        } else {
            match c {
                '+' => stack.add(),
                '-' => stack.sub(),
                '*' => stack.mul(),
                '/' => stack.div(),
                '%' => stack.modulo(),
                '^' => stack.pow(),
                _ => return None,
            }
        }
    }
    Some(stack.ret())
}

struct Stack {
    stack: [i32;8],
    ptr: usize,
}

impl Stack {
    fn new() -> Self {
        Self { stack: [0;8], ptr: 0 }
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
    fn ret(&self) -> i32 {
        self.stack[0]
    }
}