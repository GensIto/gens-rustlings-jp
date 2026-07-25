#[derive(Debug)]
enum Message {
    // TODO: 以下のコードで使うMessage列挙型の型を定義してください。
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
