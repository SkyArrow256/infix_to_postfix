use crate::primitive::{Int, Bool};

/// トークン
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Item(Operand),
	Symbol(Operator),
}

/// 値
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
	/// 整数型
    Integer(Int),
	/*
	/// 浮動小数点数型
	Double(f64),
	/// 文字型
	Char(char),
    /// 式
    Exp(i32),
	*/
	/// 論理型
	Boolean(Bool),
	// 変数
	Variable,
}

/// 演算子
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	/// 左括弧 (
    LeftParen,
    /// 右括弧 )
    RightParen,
    /// 代入 =
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
    Neg,
    /// 冪乗 ^
    Pow,
	///　等価 ==
	Eq,
	/// 不等価 !=
	NotEq,
	/// 大なり >
	Greater,
	/// 小なり < 
	Less,
	/// 以上 >=
	GreaterEq,
	/// 以下 <=
	LessEq,
	/// 否定 !
	Not,
	/// 論理和 &
	And,
	/// 論理積 ||
	Or,
}

impl Operator {
	/// 演算子の優先度を取得
	pub fn prec(&self) -> u8 {
		match self {
			Self::RightParen => unreachable!(),
			Self::LeftParen => 0,
			Self::Assign => 1,
			Self::Or => 2,
			Self::And => 3,
			Self::Eq | Self::NotEq | Self::Greater | Self::Less | Self::GreaterEq | Self::LessEq => 4,
			Self::Add | Self::Sub => 5,
			Self::Mul | Self::Div | Self::Mod => 6,
			Self::Neg | Self::Not => 7,
			Self::Pow => 8,
		}
	}
	/// 演算子の結合方向を取得
	pub fn assoc(&self) -> Assoc {
		match self {
			Self::Assign | Self::Pow => Assoc::Right,
			_ => Assoc::Left,
		}
	}
}

/// トークンの結合方向
#[derive(PartialEq)]
pub enum Assoc {
	/// 左結合
	Left,
	/// 右結合
	Right
}