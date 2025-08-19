use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    io::{self, Write},
};

use fnv::FnvHasher;
use infix_to_rpn::{InfixExpression, RpnExpression};

pub struct Calc {
    table: Table,
}

impl Calc {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }
    pub fn run(&mut self) {
        let code = read_stdin();
        let tokens = get_rpn(&code);
        let result = self.calc(tokens);
        println!("{result}");
    }
    fn calc(&mut self, rpn: RpnExpression) -> i32 {
        //計算スタックを作成
        let mut stack = Stack::new();
        use infix_to_rpn::Token::*;
        for token in rpn.tokens() {
            match token {
                Int(i) => stack.push(Operand::Int(*i)),
                Exp(exp) => stack.push(Operand::Exp(exp)),
                Assign => {
                    let (num, name) = (stack.pop(), stack.pop());
                    let name = if let Operand::Exp(name) = name {
                        name
                    } else {
                        panic!();
                    };
                    self.table.set(&name, self.eval(num));
                }
                UnarySub => {
                    let a = stack.pop();
                    stack.push(Operand::Int(-self.eval(a)));
                }
                Add => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a) + self.eval(b)));
                }
                Sub => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a) - self.eval(b)));
                }
                Mul => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a) * self.eval(b)));
                }
                Div => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a) / self.eval(b)));
                }
                Mod => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a) % self.eval(b)));
                }
                Pow => {
                    let (b, a) = (stack.pop(), stack.pop());
                    stack.push(Operand::Int(self.eval(a).pow(self.eval(b) as u32)));
                }
                _ => unreachable!(),
            }
        }
        //最後にスタックに残った値を取り出す なければ0
		match stack.pop() {
			Operand::Int(i) => i,
			Operand::Exp(name) => self.table.get(name)
		}

    }
    //オペランドを評価してi32として返す
    fn eval(&self, operand: Operand) -> i32 {
        match operand {
            Operand::Int(i) => i,
            Operand::Exp(name) => self.table.get(name),
        }
    }
}

//計算スタック
struct Stack<'a>(Vec<Operand<'a>>);

impl<'a> Stack<'a> {
    //計算スタックを作成
    fn new() -> Self {
        Self(Vec::new())
    }
    //スタックにオペランドを追加
    fn push(&mut self, operand: Operand<'a>) {
        self.0.push(operand);
    }
    //スタックからオペランドを取り出す
    fn pop(&mut self) -> Operand<'a> {
        if let Some(operand) = self.0.pop() {
			operand
		} else {
			Operand::Int(0)
		}
    }
}

//変数テーブル
#[derive(Debug)]
struct Table(HashMap<String, i32, BuildHasherDefault<FnvHasher>>);

impl Table {
    //変数テーブルを作成
    fn new() -> Self {
        Self(HashMap::default())
    }
    //変数に数字を代入
    fn set(&mut self, name: &str, num: i32) {
        self.0.insert(name.to_owned(), num);
    }
    //変数から数字を取得
    fn get(&self, name: &str) -> i32 {
        *self.0.get(name).unwrap()
    }
}

//スタックに保持されるオペランド
enum Operand<'a> {
    Int(i32),
    Exp(&'a str),
}

fn read_stdin() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut infix = String::new();
    io::stdin().read_line(&mut infix).unwrap();
    infix
}

fn get_rpn(code: &str) -> RpnExpression {
    let infix = InfixExpression::from(code);
    infix.try_into().unwrap()
}
