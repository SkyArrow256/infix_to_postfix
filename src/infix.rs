use crate::Token;
use std::mem;

/// 中置記法で表現された式です。
pub struct InfixExpression(Vec<Token>);

impl InfixExpression {
    /// Tokensへの参照を取得します。
    pub fn tokens(&self) -> &[Token] {
        &self.0
    }
}

impl From<&str> for InfixExpression {
    fn from(code: &str) -> Self {
        use Token::*;
        let mut tokens = Vec::new();

        //文字を1つ受け取って式にトークンを追加 空白はスキップ
        for c in code.chars().filter(|c| !c.is_whitespace()) {
            if c.is_ascii_digit() {
                //文字が数字で、最後のトークンがIntの時、最後のトークンに数字を足す
                let num = c.to_digit(10).unwrap() as i32;
                if let Some(last) = tokens.last_mut() {
                    if let Int(last) = last {
                        //10進数なので10ずらす
                        *last *= 10;
                        *last += num;
                        continue;
                    }
                }
                tokens.push(Int(num));
            } else {
                //文字が演算子のとき
                match c {
                    '(' => tokens.push(ParenLeft),
                    ')' => tokens.push(ParenRight),
                    '+' => tokens.push(Add),
                    '-' => {
                        /*
                            減算として扱われるのは、直前が
                            ・数字・式の時
                            ・右括弧の時
                        */
                        if let Some(first) = tokens.last() {
                            let last = mem::discriminant(first);
                            if last == mem::discriminant(&ParenRight)
                                || last == mem::discriminant(&Int(0))
                                || last == mem::discriminant(&Exp("".to_string()))
                            {
                                tokens.push(Sub);
                                continue;
                            }
                        }
                        tokens.push(UnarySub);
                    }
                    '*' => tokens.push(Mul),
                    '/' => tokens.push(Div),
                    '%' => tokens.push(Mod),
                    '^' => tokens.push(Pow),
                    ':' => tokens.push(Assign),
                    _ => {
                        //文字がそれ以外、つまり式のとき、tokensの最後が式だったならそれに文字を加える
                        if let Some(last) = tokens.last_mut() {
                            if let Exp(str) = last {
                                str.push(c);
                                continue;
                            }
                        }
                        tokens.push(Exp(String::from(c)));
                    }
                }
            }
        }
        Self(tokens)
    }
}
