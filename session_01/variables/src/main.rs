#[allow(dead_code, unused_variables, unused_imports, unused_mut)]
use authentication::read_line;

fn double(n: i32) -> i32 {
	n * 2
}

fn double_or_nothing(n: i32) -> i32 {
	if n > 0 {
		n * 2
	} else {
		0
	}
}
fn greet(s: String) -> String {
	println!("Hello {}", s);
	s
}

fn greet_borrow(s: &String) {
	println!("Hello {}", s);
}

fn greet_borrow_mut(s: &mut String) {
	*s = format!("Hello {}", s);
}

fn main() {
	let mut name: String = "World".to_string();
	greet(name.clone());
	greet_borrow(&name);
	greet_borrow_mut(&mut name);
	println!("{}", name);

	let input: String = read_line();
	println!("You typed: [{input}]");
}
