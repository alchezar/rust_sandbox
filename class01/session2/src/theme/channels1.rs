// IKinder

use std::sync::mpsc;

enum Command {
	SayHello,
	Quit,
}

pub fn main() {
	crate::show_name(file!());

	let (tx, rx) = mpsc::channel::<Command>();
	let handle = std::thread::spawn(move || {
		'l: while let Ok(command) = rx.recv() {
			match command {
				Command::SayHello => println!("Hello"),
				Command::Quit => {
					println!("Quitting...");
					break 'l;
				}
			}
		}
	});

	for _ in 0..10 {
		tx.send(Command::SayHello).unwrap();
	}
	println!("Sending quit");
	tx.send(Command::Quit).unwrap();
	handle.join().unwrap();
}
