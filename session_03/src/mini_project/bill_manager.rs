// IKinder

use std::collections::HashMap;

enum MainMenu {
	Add,
	View,
	Remove,
	Exit,
}

impl MainMenu {
	fn from_str(input: &str) -> Option<Self> {
		match input {
			"1" => Some(MainMenu::Add),
			"2" => Some(MainMenu::View),
			"3" => Some(MainMenu::Remove),
			"0" => Some(MainMenu::Exit),
			_ => None,
		}
	}
	fn show() {
		println!("=== Bill Manager ===");
		println!("1.Add Bill");
		println!("2.View Bill");
		println!("3.Remove Bill");
		println!("0.Exit");
		println!("Enter selection:");
	}
}

#[derive(Debug, Clone)]
struct Bill {
	name: String,
	amount: f64,
}

struct Bills {
	inner: HashMap<String, Bill>,
}

impl Bills {
	fn new() -> Self {
		Self { inner: HashMap::new() }
	}
	fn add(&mut self, bill: Bill) {
		self.inner
			.insert(bill.name.to_string(), bill);
	}
	fn get_all(&self) -> Vec<&Bill> {
		self.inner
			.values()
			.collect()
	}
	fn remove(&mut self, name: &str) -> bool {
		self.inner
			.remove(name)
			.is_some()
	}
}

///
/// Handles input from the user.
///
fn get_input() -> Option<String> {
	// Read the line from the terminal.
	let mut buffer = String::new();
	while std::io::stdin()
		.read_line(&mut buffer)
		.is_err()
	{
		println!("Please enter correct input!");
	}

	// Remove spaces and make input case-insensitive.
	let input = buffer
		.trim()
		.to_lowercase()
		.to_owned();

	// Return the final input.
	if input.is_empty() { None } else { Some(input) }
}

///
/// Parse string to the f64 number.
///
fn get_bill_amount() -> Option<f64> {
	loop {
		let input = get_input()?;
		if input.is_empty() {
			return None;
		};

		// Parse the string into f64 number.
		let parsed_input: Result<f64, _> = input.parse();
		match parsed_input {
			Ok(amount) => return Some(amount),
			Err(_) => println!("Please enter a number!"),
		}
	}
}

mod menu {
	use super::{Bill, Bills, get_bill_amount, get_input};

	pub fn add_bill(bills: &mut Bills) {
		println!("Bill name: ");
		let name = match get_input() {
			Some(input) => input,
			None => return,
		};
		println!("Amount: ");
		let amount = match get_bill_amount() {
			Some(amount) => amount,
			None => return,
		};
		let bill = Bill { name, amount };
		bills.add(bill);
		println!("Bill added!");
	}
	pub fn view_bills(bills: &Bills) {
		for bill in bills.get_all() {
			println!("{:?}", bill)
		}
	}
	pub fn remove_bill(bills: &mut Bills) {
		view_bills(bills);
		println!("Enter bill name to remove: ");
		let name = match get_input() {
			Some(name) => name,
			None => return,
		};
		if Bills::remove(bills, name.as_str()) {
			println!("Bill {name} removed!");
		} else {
			println!("Bill {name} was not found!");
		}
	}
}

pub fn run() {
	let mut bills = Bills::new();

	loop {
		MainMenu::show();
		let input = get_input().expect("No data entered");
		match MainMenu::from_str(&input) {
			Some(MainMenu::Add) => menu::add_bill(&mut bills),
			Some(MainMenu::View) => menu::view_bills(&bills),
			Some(MainMenu::Remove) => menu::remove_bill(&mut bills),
			Some(MainMenu::Exit) => return,
			None => (),
		}
	}
}
