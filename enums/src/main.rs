fn main() {
    println!("Hello, world!");
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };   
    let m = Message::Write("Hello, world".to_string());

    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();

    process_message(x);
    // Doesn't work
    // Wrong type
    // process_message(y);
    for message in v1 {
        process_message(message);
    }

    let x = 5;
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    Pass,
}

fn process_color_change(msg: Message) {
    // Doesn't work
    // Not all enum types covered
    // let Message::ChangeColor(r, g, b) = msg; // This causes a compile-time error.
}

fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x: x, y: y } => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    };
}