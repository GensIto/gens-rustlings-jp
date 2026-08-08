struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // この関数は変更しないでください。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // `Result`型を返すことが好ましいかもしれないが、
            // テスト関数がどのようにパニックを起こすか学びたいためこの表記にしている。
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: このテストではコンストラクタをパスしたサイズの長方形かどうかを確認しましょう。
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 横幅を確認
        assert_eq!(rect.height, 20); // 高さを確認
    }

    // TODO: このテストでは負の横幅を持った長方形を作成しようとした時にパニックを起こすかどうか確認しましょう。
    #[test]
    #[should_panic]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: このテストでは負の高さを持った長方形を作成しようとした時にパニックを起こすかどうか確認しましょう。
    #[test]
    #[should_panic]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
