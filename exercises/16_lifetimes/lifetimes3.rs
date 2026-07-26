// ライフタイムは構造体が参照を持つ際にも必要になります。

// TODO: 構造体に関するコンパイルエラーを修正してください。
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

struct BookStr {
    author: String,
    title: String,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    let book_str = BookStr {
        author: "George Orwell".to_owned(),
        title: "1984".to_owned(),
    };

    println!("{} by {}", book.title, book.author);
    println!("{} by {}", book_str.title, book_str.author);
}
