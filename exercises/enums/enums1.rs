// enums1.rs
//
// No hints this time! ;)

// I AM DONE HERE

#[derive(Debug)]
enum Message {
    // DONE: define a few types of messages as used below
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
