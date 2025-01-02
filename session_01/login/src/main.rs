use authentication::{login, read_line, LoginAction, LoginRole};

fn main() {
	let mut tries: i32 = 0;
	loop {
		println!("Please enter your username: ");
		let username: String = read_line();
		println!("Please enter your password: ");
		let password: String = read_line();

		match login(&username, &password) {
			Some(LoginAction::Granted(role)) => {
				match role {
					LoginRole::Admin => println!("Admin"),
					LoginRole::User => println!("User"),
				}
				break;
			}
			Some(LoginAction::Denied) => {}
			None => {
				println!("New user");
			}
		}

		println!("Incorrect!");
		tries += 1;
		if tries >= 3 {
			println!("Too many failed logins!");
			break;
		}
	}
}
