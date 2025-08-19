use crate::infix::InfixExpression;
use crate::{ParseError, Token};

/// 逆ポーランド記法で表現された式です。
pub struct RpnExpression(Vec<Token>);

impl RpnExpression {
    /// Tokensへの参照を取得します。
    pub fn tokens(&self) -> &[Token] {
        &self.0
    }
}

impl TryFrom<InfixExpression> for RpnExpression {
    type Error = ParseError;

    fn try_from(infix: InfixExpression) -> Result<Self, Self::Error> {
        use Token::*;
        let mut que = Vec::new();
        let mut stack = Vec::new();
        for token in infix.tokens() {
            match token {
                Assign => {
                    //右結合なので、Assign以外の全てをpopする
                    while let Some(last) = stack.last() {
                        if *last == Assign {
                            break;
                        }
                        que.push(stack.pop().unwrap());
                    }
                    stack.push(Assign);
                }
                //左括弧はstackに
                ParenLeft => stack.push(ParenLeft),
                ParenRight => {
                    //閉じ括弧が現れるstackのtokenをqueに追加
                    while *stack.last().ok_or(ParseError::ParenthesesMismatch)? != ParenLeft {
                        que.push(stack.pop().unwrap());
                    }
                    //閉じ括弧は不要なのでstackから捨てる
                    stack.pop();
                }
                c @ Add | c @ Sub => {
                    //stackの最後が空または括弧または自分未満の優先順位を持つ演算子になるまでpopしてqueに追加
                    while let Some(last) = stack.last() {
                        if *last == ParenLeft || *last == Assign {
                            break;
                        }
                        que.push(stack.pop().unwrap());
                    }
                    stack.push(c.clone());
                }
                c @ Mul | c @ Div | c @ Mod => {
                    //stackの最後が空または括弧または自分未満の優先順位を持つ演算子になるまでpopしてqueに追加
                    while let Some(last) = stack.last() {
                        if *last == ParenLeft || *last == Assign || *last == Add || *last == Sub {
                            break;
                        }
                        que.push(stack.pop().unwrap());
                    }
                    stack.push(c.clone());
                }
                UnarySub => {
                    //stackの最後が空または括弧または(右結合なので)自分**以下**の優先順位を持つ演算子になるまでpopしてqueに追加
                    //これは冪乗以外の全て
                    while let Some(last) = stack.last() {
                        if *last != Pow {
                            break;
                        }
                        que.push(stack.pop().unwrap());
                    }
                    stack.push(UnarySub);
                }
                //冪乗は最も優先順位が高いのでいつでもstackに追加
                Pow => stack.push(Pow),
                //その他の場合、式なのでqueにそのまま追加
                exp @ _ => que.push(exp.clone()),
            }
        }

        //stackに括弧が残っているなら括弧の数が間違っている
        //数があっていたらstackの中のトークンを最後から取り出してqueに入れて終了
        if stack.contains(&ParenLeft) {
            Err(ParseError::ParenthesesMismatch)
        } else {
            for token in stack.into_iter().rev() {
                que.push(token);
            }
            Ok(Self(que))
        }
    }
}
