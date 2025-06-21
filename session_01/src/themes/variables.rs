// IKinder

pub fn main() {
	let n = { 5 * 12 };

	let name = "John".to_owned();
	greet(name.clone());
	greet_borrow(&name);

	let mut name = name;
	greet_borrow_mut(&mut name);

	let input = read_line();
	println!("You typed: [{input}]");
}

fn double_or_nothing(n: i32) -> i32 {
	if n > 0 { n * 2 } else { 0 }
}

fn double(n: i32) -> i32 {
	n * 2
}

fn greet(s: String) -> String {
	println!("Hello {s}");
	s
}

fn greet_borrow(s: &String) {
	println!("Hello {s}");
}

fn greet_borrow_mut(s: &mut String) {
	*s = format!("Mr. {s}");
	println!("Hello {s}");
}

fn read_line() -> String {
	let mut buffer = String::new();
	std::io::stdin()
		.read_line(&mut buffer)
		.ok();
	buffer.trim().into()
}
