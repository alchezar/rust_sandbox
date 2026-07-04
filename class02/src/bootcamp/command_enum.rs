// IKinder

#![allow(dead_code, unused_variables)]

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string: String = match self {
            Command::Undo => String::from("undo"),
            Command::Redo => String::from("redo"),
            Command::AddText(text) => format!("add {}", text),
            Command::MoveCursor(x, y) => format!("{{move_cursor({}, {})}}", x, y),
            Command::Replace { from, to } => format!("{{replace({}, {})}}", from, to),
        };
        json_string
    }
}

pub fn run() {
    println!("Hello, world!");

    let cmd: Command = Command::Undo;
    let cmd: Command = Command::AddText(String::from("string!"));

    let json_string: String = cmd.serialize();

    let age: i32 = 35;
    match age {
        1 => println!("Happy 1st birthday!"),
        2..17 => println!("You are little shit!"),
        x => println!("You are {x} years old."),
    }
}
