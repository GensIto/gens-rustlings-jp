// この関数は変えないでください。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: 一行移動させてコンパイルエラーを解消してください。

    let string1 = String::from("long string is long"); // 'a
    let result; // 'a
    let string2 = String::from("xyz"); // 'a
    {
        // let string2 = String::from("xyz"); // 'b
        result = longest(&string1, &string2); // 'b
                                              // println!("The longest string is '{result}'"); // 'b resultが同じライフタイム
    }
    println!("The longest string is '{result}'"); // 'a
}
