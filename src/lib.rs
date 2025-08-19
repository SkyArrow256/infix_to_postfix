//! # infix-to-rpn
//! 
//! **現在多くの機能が未実装です！**
//!
//! infix-to-rpnは操車場アルゴリズムを使用して中置記法( ( 1 + 2 ) * 3 ^ 2 )を逆ポーランド記法( 1 2 + 3 2 ^ * )に置き換えるライブラリです。
//! C言語のライブラリを扱える言語、例えばHSPやC#でも動作するようにする予定です。
//!
//! 仕様はWikipediaを参考にしました。
//! <https://ja.wikipedia.org/wiki/操車場アルゴリズム>
//!
//! ## 使い方
//! &strから中置記法の式をfrom()メソッドでトークナイズしたあと、try_fromで逆ポーランド記法に変換します。
//! 文字列はスペースが間に複数入っていても問題ありません。
//! ```
//! let infix = "-1 / (1+2) + -3^3"
//! let code: Result<RpnExpression, _> = InfixExpression::from(infix).try_into();
//! ```
//!
//! ## 使用できるオペレーターとその優先度
//! 高い順に
//! ``` -(単項), ^, * / %, + -(二項), :(代入) ```
//! です。

use std::mem;

/// このライブラリで起こりうるエラーです。
#[derive(Debug)]
pub enum ParseError {
    /// 括弧の不一致
    ParenthesesMismatch,
    /// 数値のパースに失敗
    InvalidNumber,
}

/// 式に含まれるトークンです。
/// 式(Exp)にこれらのトークンに使われる文字を含まないでください。
/// 例えば、次の式は正しくパースされません
/// ```Int(3) + 1```
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// 左括弧 (
    ParenLeft,
    /// 右括弧 )
    ParenRight,
    /// 整数型
    Int(i32),
    /// 式(変数名、関数名)
    Exp(String),
    /// 代入 :
    Assign,
    /// 加算 +
    Add,
    /// 減算 -
    Sub,
    /// 乗算 *
    Mul,
    /// 除算 /
    Div,
    /// 余剰 %
    Mod,
    /// 単項マイナス -
    UnarySub,
    /// 冪乗 ^
    Pow,
}

/// 中置記法で表現された式です。
pub struct InfixExpression(Vec<Token>);

impl InfixExpression {
    /// Tokensへの参照を取得します。
    fn tokens(&self) -> &[Token] {
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

/// 逆ポーランド記法で表現された式です。
#[derive(Debug)]
pub struct RpnExpression(Vec<Token>);

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
                Pow => {
                    //stackの最後が空または括弧または自分**以下**の(右結合なので)優先順位を持つ演算子になるまでpopしてqueに追加
                    //これは単項マイナス以外の全て
                    while let Some(last) = stack.last() {
                        if !(*last == UnarySub) {
                            break;
                        }
                        que.push(stack.pop().unwrap());
                    }
                    stack.push(Pow);
                }
                //単項マイナスは最も優先順位が高いのでいつでもqueに追加
                UnarySub => stack.push(UnarySub),
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

impl RpnExpression {
    /// Tokensへの参照を取得します。
    pub fn tokens(&self) -> &[Token] {
        &self.0
    }
}