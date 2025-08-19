use fnv::FnvHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};
//変数テーブル
pub struct Table(HashMap<String, i32, BuildHasherDefault<FnvHasher>>);

impl Table {
    //変数テーブルを作成
    pub fn new() -> Self {
        Self(HashMap::default())
    }
    //変数に数字を代入
    pub fn set(&mut self, name: &str, num: i32) {
        self.0.insert(name.to_owned(), num);
    }
    //変数から数字を取得
    pub fn get(&self, name: &str) -> i32 {
        *self.0.get(name).unwrap()
    }
}
