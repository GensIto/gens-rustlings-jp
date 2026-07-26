trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// 名前付きジェネリクス版
// メリット: where / 戻り値で型名を再利用しやすい。複数境界や型同士の関係を書きやすい。らしい
// デメリット: 境界だけなら署名がやや冗長。
fn compare_license_types<T: Licensed, S: Licensed>(software1: T, software2: S) -> bool {
    // 同等の impl Trait 版（短く書けるが、匿名型のため型名の再利用はしにくい）らしい
    // fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool
    // ↓とのこと
    // 良い設計の本体はトレイト境界そのもので、構文の差は読みやすさの話です。引き継ぎを最優先するなら、単純なうちは impl Trait、複雑さが出たら名前付きに上げる、が現実的です。
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // この行で関数のテストができます。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
