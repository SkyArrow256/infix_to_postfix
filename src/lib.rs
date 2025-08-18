//! #infix-to-rpn
//! 
//! infix-to-rpnは操車場アルゴリズムを使用して中置記法( ( 1 + 2 ) * 3 ^ 2 )を逆ポーランド記法( 1 2 + 3 2 ^ * )に置き換えるライブラリです。
//! Rustではライブラリクレートとして、C言語のライブラリを扱える言語ならば共有ライブラリとして使用できます。
//! HSPやC#でも動くはずです(未検証)。
//! 
//! 仕様はWikipediaを参考にしました。
//! https://ja.wikipedia.org/wiki/操車場アルゴリズム
//! 
//! ##使い方
//! to_rpn関数に中置記法の式を渡すとResult<String, ParseError>型で帰ってきます。
//! 
//! ##使用可能なオペレーター
//! + - * / % ( )
//! 
//! ##優先度
//! 高い順に
//! ^
//! * / %
//! + -
//! です。 

/// オペレーターとオペランドの間にスペースを挿入します。
fn format(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            '+' | '-' | '*' | '/' | '^' | '(' | ')' | '%' => {
                result.push(' ');
                result.push(c);
                result.push(' ');
            }
            _ => result.push(c),
        }
    }
    //前後のスペースを削除
    result.trim().to_string()
}

/// 入力された中置記法の文字列を逆ポーランド記法に変換します。
pub fn to_rpn(input: &str) -> Result<String, ParseError> {
	let input = format(input);
    let mut que = Que::with_capacity(input.len());
    let mut stack = Vec::new();
    for str in input.split_whitespace() {
		//空白は絶対に取り除かれているのでunwrap
        match str.chars().next().unwrap() {
            //左括弧はそのままスタックに追加
            '(' => stack.push('('),
            ')' => {
                //括弧が閉じるまでスタックの中身をキューに追加
                while *stack.iter().last().ok_or(ParseError::ParenthesesMismatch)? != '(' {
                    que.push_operator(stack.pop().unwrap());
                }
                //スタックから不要な括弧は捨てる
                stack.pop();
            }
            c @ '*' | c @ '/' | c @ '%' => {
                //スタックに何かあれば、スタックの最後が自分未満の優先順位になるまでキューにスタックの中身を追加
                //乗算、除算未満の優先度をもつ演算子は + - また、括弧でも終了する
                while !stack.is_empty() {
                    let last_item = *stack.last().unwrap();
                    if last_item == '(' || last_item == '+' || last_item == '-' { break; }
                    que.push_operator(stack.pop().unwrap());
                }
                stack.push(c);
            }
            c @ '+' | c @ '-' => {
                //スタックに何かあれば、スタックの最後が自分未満の優先順位になるまでキューにスタックの中身を追加
                //加算、減算は優先度が最も低いので括弧だけ考える
                while !stack.is_empty() {
                    let last_item = *stack.last().unwrap();
                    if last_item == '(' { break; }
                    que.push_operator(stack.pop().unwrap());
                }
                stack.push(c);
            }
            //累乗は右結合かつ最も優先順位が高いのでいかなる場合でもそのままキューに追加
            '^' => stack.push('^'),
            _ => que.push_operand(str),
        }
    }
    if stack.contains(&'(') { return Err(ParseError::ParenthesesMismatch) }
    for item in stack.iter().rev() {
        que.push_operator(*item);
    }
    Ok(que.get_result())
}

/// このライブラリで起こりうるエラーです。
#[derive(Debug)]
pub enum ParseError {
	///括弧の不一致
	ParenthesesMismatch,
}

/// フォーマットに便利なメソッドを備えた配列です。
#[derive(Debug)]
struct Que(String);

impl Que {
    fn with_capacity(capacity: usize) -> Self {
        Self(String::with_capacity(capacity))
    }
    fn push_operator(&mut self, c: char) {
        self.0.push(c);
        self.0.push(' ');
    }
    fn push_operand(&mut self, str: &str) {
        self.0.push_str(str);
        self.0.push(' ');
    }
    fn get_result(mut self) -> String {
        self.0.pop();
        self.0
    }
}