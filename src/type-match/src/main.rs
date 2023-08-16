#[derive(Debug)]
enum Action {
    Say(String),
    SayAlias(String),
    MoveTo(i32, i32),
    ChangeColor(u16, u16, u16),
    NullAction,
}

fn main() {
    let actions = [
        Action::Say("hello world".to_string()),
        Action::SayAlias("hello world".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColor(1, 2, 3),
        Action::NullAction,
    ];

    for action in actions {
        match action {
            Action::Say(s) | Action::SayAlias(s) => {
                println!("str = {}", s);
            }
            Action::MoveTo(x, y) => {
                println!("x = {}, y = {}", x, y);
            }
            Action::ChangeColor(r, g, b) => {
                println!("rgb = {:?} ", (r, g, b));
            }
            _ => {
                println!("null action")
            }
        }
    }

    // Option
    option_demo();

    // if-let
    if_let_demo();
}

// enum Option<T> {Some(T), None}
fn option_demo(){

}

// if let Mode = type { }
fn if_let_demo(){
    let actions = [Action::Say("hello world".to_string()), Action::NullAction];
    for action in actions {
        if let Action::Say(s) = action {
            println!("str = {}", s);
        }
    }
}
