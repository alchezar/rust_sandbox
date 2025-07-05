// IKinder

use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

pub fn main() {
	println!("::rwlock\n");

	std::thread::spawn(|| {
		loop {
			println!("Current user (in a thread)");
			let users = USERS.read().unwrap();
			print!("{:?}", users);
			std::thread::sleep(std::time::Duration::from_secs(3))
		}
	});

	'l: loop {
		println!("Enter a name to add to the user list (or 'q' to quit):");
		let mut input = read_line();
		if input == "q" {
			break 'l;
		} else {
			let mut lock = USERS.write().unwrap();
			lock.push(input)
		}
	}
}

fn build_users() -> Vec<String> {
	vec!["Alice".to_owned(), "Bob".to_owned()]
}

fn read_line() -> String {
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.unwrap();

	input.trim().to_owned()
}
