use super::Operand;

//計算スタック
pub struct Stack<'a>(Vec<Operand<'a>>);

impl<'a> Stack<'a> {
    //計算スタックを作成
    pub fn new() -> Self {
        Self(Vec::new())
    }
    //スタックにオペランドを追加
    pub fn push(&mut self, operand: Operand<'a>) {
        self.0.push(operand);
    }
    //スタックからオペランドを取り出す
    pub fn pop(&mut self) -> Operand<'a> {
        if let Some(operand) = self.0.pop() {
            operand
        } else {
            Operand::Int(0)
        }
    }
}
