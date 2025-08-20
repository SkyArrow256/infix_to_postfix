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
//! &strから中置記法の式をfrom()メソッドでトークナイズしたあと、try_intoで逆ポーランド記法に変換します。
//! 文字列はスペースが間に複数入っていても問題ありません。

mod tokens;
mod primitive;
mod infix2;
mod postfix2;

pub use infix2::InfixExpression;
pub use postfix2::PostfixExpression;

/// このライブラリで起こりうるエラーです。
#[derive(Debug)]
pub enum ParseError {
    /// 括弧の不一致
    ParenthesesMismatch,
    /// 数値のパースに失敗
    InvalidNumber,
}

trait Expression {
    fn as_tokens(&self) -> &[tokens::Token];
}