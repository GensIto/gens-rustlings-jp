// Rustのコンパイラーは与えられた参照が有効かどうかをどのように確認すればいいか知る必要があります。
// そのためプログラマーは参照が使われる前にスコープからもしも外れるリスクを把握する必要があります。
// 参照は借り物であり、それ自身に情報を保有していないことを留意してください。
// 所有権がどのようにしてスコープから外れたらどうなるのでしょう？

// TODO: 関数の入出力を更新することでコンパイルエラーを修正してください。
// https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html#%E9%96%A2%E6%95%B0%E3%82%B7%E3%82%B0%E3%83%8B%E3%83%81%E3%83%A3%E3%81%AB%E3%81%8A%E3%81%91%E3%82%8B%E3%83%A9%E3%82%A4%E3%83%95%E3%82%BF%E3%82%A4%E3%83%A0%E6%B3%A8%E9%87%88
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
