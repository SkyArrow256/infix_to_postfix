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
//! let code: Result<PostfixExpression, _> = InfixExpression::from(infix).try_into();
//! ```
//!
//! ## 使用できるオペレーターとその優先度
//! 高い順に
//! ``` ^, -(単項), * / %, + -(二項), =(代入) ```
//! です。

mod infix;
mod postfix;

pub use infix::InfixExpression;
pub use postfix::PostfixExpression;

/// 式に含まれるトークンです。
/// 式(Exp)にこれらのトークンに使われる文字を含まないでください。
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

/// このライブラリで起こりうるエラーです。
#[derive(Debug)]
pub enum ParseError {
    /// 括弧の不一致
    ParenthesesMismatch,
    /// 数値のパースに失敗
    InvalidNumber,
}
