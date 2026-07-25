// この関数を呼び出す代わりに、`string_slice`か`string`をmain関数内で呼び出してください。
// fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 以下に記載された文字列は`String`か`&str`が該当します。
// `placeholder(…)`を`string_slice(…)`か`string(…)`に置き換えてください。
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

  // WARNING: 文字ごとのインデックスではなくバイトごとのインデックスです。
  // 文字のインデックスを利用するためには`s.chars().nth(INDEX)`と記載します。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
