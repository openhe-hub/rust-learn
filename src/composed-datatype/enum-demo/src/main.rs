#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m = Message::Quit;
    let m1 = Message::Move { x: 1, y: 2 };
    let m2 = Message::ChangeColor(1, 2, 3);
    dbg!(m2);
    dbg!(m1);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // println!("{}, {}", six, none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
