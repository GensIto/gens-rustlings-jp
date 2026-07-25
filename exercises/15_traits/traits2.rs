trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: `AppendBar`を文字列の配列型に移植してください。
// `append_bar`は"Bar"を配列の末尾に追加するメソッドです。
impl AppendBar for Vec<String> {
    // なんでtraitはmutじゃないのにこっちたmutでいいんだろう?
    // 短いまとめ
    // トレイトの self … 誰が所有権を持つか（外との契約）
    // 実装の mut self … 受け取ったあと、中でいじってよいか（中の都合）
    // らしい
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
