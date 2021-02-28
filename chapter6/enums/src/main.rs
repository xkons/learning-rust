
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:?}", self);
    }

    fn get_coordinates(&self) -> Option<(i32, i32)> {
        match *self {
            Message::Move {x, y} => Some((x, y)),
            Message::Quit => None,
            Message::Write(_) => None,
            Message::ChangeColor(_,_,_) => None
        }
    }
}

fn main() {
    let write_message = Message::Write(String::from("hello"));
    write_message.print();

    let move_message = Message::Move { x: 2, y: 3 };
    match move_message.get_coordinates() {
        Some(coordinates) => println!("Move coordinates: x: {}, y: {}", coordinates.0, coordinates.1),
        _ => {}
    }
    move_message.print();

    let quit_message = Message::Quit;
    quit_message.print();
    let change_color_message = Message::ChangeColor(2,3,4);
    change_color_message.print();
}
