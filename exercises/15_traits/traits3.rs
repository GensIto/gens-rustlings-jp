#![allow(dead_code)]

trait Licensed {
    // TODO: `licensing_info`というデフォルトの移植を追加してください。
    // これにより下記に記す二つの構造体のような移植が関数を繰り返すことなくデフォルトの振る舞いとしてシェアできるようになります。
    // デフォルトのライセンス情報は"Default license"という文字列としてください。
    // https://doc.rust-jp.rs/book-ja/ch10-02-traits.html#%E3%83%87%E3%83%95%E3%82%A9%E3%83%AB%E3%83%88%E5%AE%9F%E8%A3%85
    fn licensing_info(&self) -> String {
        String::from("Default license")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // この行は変えないでください。
impl Licensed for OtherSoftware {} // この行は変えないでください。

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
