// 集合体内で要素の操作を実行するとき、イテレータは不可欠です。
// このエクササイズによってイテレータを使う構文や、イテレータが使用可能な要素を取り扱う方法を理解するのに役立ちます。

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: 配列に対してイテレータを作成してください。
        let mut fav_fruits_iterator = my_fav_fruits.iter();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"custard apple")); // TODO: `todo!()`を置き換えてください。
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), Some(&"peach")); // TODO: `todo!()`を置き換えてください。
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), None); // TODO: `todo!()`を置き換えてください。
    }
}
